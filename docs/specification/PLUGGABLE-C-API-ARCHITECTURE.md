# Pluggable C-API Architecture
## From CPython Transpiler to C-Extension Framework

**Principle**: Kaizen (Continuous Improvement)
**Philosophy**: Design for Extensibility
**Impact**: Transforms Spydecy from a niche tool into a platform

---

## Executive Summary

**Original Design**: `CPythonAnalyzer` - hardcoded to understand CPython's C API

**Problem**: The Python scientific ecosystem (NumPy, SciPy, Pandas, scikit-learn) uses **different C APIs**:
- NumPy: `PyArrayObject`, `PyArray_*` functions
- SciPy: BLAS/LAPACK wrappers, Fortran interop
- Pandas: Custom extension types
- Pillow: Image buffer APIs

A transpiler that only understands CPython's API will fail on these libraries.

**Solution**: Refactor from a monolithic `CPythonAnalyzer` to a **pluggable, trait-based architecture** where domain-specific C-API analyzers can be added incrementally.

**Impact**: Transforms Spydecy from "transpiling one project" to "transforming an entire ecosystem."

---

## Architectural Comparison

### Original Architecture (Monolithic)

```rust
pub struct CTranspiler {
    parser: CParser,
    cpython_analyzer: CPythonAnalyzer,  // ❌ Hardcoded
    ownership: OwnershipAnalyzer,
    decy_backend: DecyBackend,
}

impl CPythonAnalyzer {
    pub fn analyze(&self, ast: &CAST) -> Result<CPythonInfo> {
        // Hardcoded knowledge of CPython API
        self.find_py_incref_decref(ast)?;
        self.find_gil_operations(ast)?;
        self.find_pydict_operations(ast)?;
        // ... etc
    }
}
```

**Problems**:
- ❌ Cannot handle NumPy's `PyArrayObject*`
- ❌ Cannot handle SciPy's BLAS wrappers
- ❌ Cannot handle Pandas' extension types
- ❌ Requires rewriting core code for each new API
- ❌ No separation of concerns

---

### Pluggable Architecture (Trait-Based)

```rust
pub trait C_API_Analyzer: Send + Sync {
    /// Unique identifier for this analyzer
    fn name(&self) -> &str;

    /// Priority (higher = runs first)
    fn priority(&self) -> u32 { 100 }

    /// Can this analyzer handle this file?
    fn can_analyze(&self, ast: &CAST, context: &AnalysisContext) -> bool;

    /// Analyze API calls
    fn analyze_api_calls(&self, ast: &CAST) -> Vec<RecognizedAPICall>;

    /// Analyze reference counting patterns
    fn analyze_refcounting(&self, ast: &CAST) -> Vec<RefCountOp>;

    /// Analyze memory management
    fn analyze_memory(&self, ast: &CAST) -> MemoryAnalysis;

    /// Analyze type system
    fn analyze_types(&self, ast: &CAST) -> TypeAnalysis;

    /// Generate Rust equivalents
    fn generate_rust_bindings(&self, api_call: &RecognizedAPICall) -> RustCode;
}

pub struct CTranspiler {
    parser: CParser,
    api_analyzers: Vec<Box<dyn C_API_Analyzer>>,  // ✅ Pluggable!
    ownership: OwnershipAnalyzer,
    decy_backend: DecyBackend,
}

impl CTranspiler {
    pub fn new() -> Self {
        Self {
            parser: CParser::new(),
            api_analyzers: vec![
                Box::new(CPythonAnalyzer),      // ✅ CPython support
                Box::new(NumPyAnalyzer),         // ✅ NumPy support
                Box::new(SciPyAnalyzer),         // ✅ SciPy support
                Box::new(PandasAnalyzer),        // ✅ Pandas support
                Box::new(PillowAnalyzer),        // ✅ Pillow support
            ],
            ownership: OwnershipAnalyzer::new(),
            decy_backend: DecyBackend::new(),
        }
    }

    pub fn transpile(&self, source: &str) -> Result<RustCode> {
        let ast = self.parser.parse(source)?;

        // Run all applicable analyzers
        let mut all_info = CAPIInfo::new();
        for analyzer in &self.api_analyzers {
            if analyzer.can_analyze(&ast, &context) {
                all_info.merge(analyzer.analyze(&ast))?;
            }
        }

        // Continue with unified analysis
        let hir = self.lower_to_hir(&ast, &all_info)?;
        // ... rest of transpilation
    }
}
```

**Benefits**:
- ✅ Each analyzer is independent, testable
- ✅ New analyzers can be added without changing core code
- ✅ Analyzers can be maintained by domain experts (NumPy team maintains `NumPyAnalyzer`)
- ✅ Clear separation of concerns
- ✅ Extensible to any C API

---

## Concrete Implementations

### 1. CPython Analyzer

```rust
// spydecy-analyzers/src/cpython.rs

pub struct CPythonAnalyzer;

impl C_API_Analyzer for CPythonAnalyzer {
    fn name(&self) -> &str {
        "CPython 3.9-3.13"
    }

    fn priority(&self) -> u32 {
        200  // High priority (Python core)
    }

    fn can_analyze(&self, ast: &CAST, context: &AnalysisContext) -> bool {
        // Check for CPython headers
        ast.includes.iter().any(|inc| {
            inc.path.contains("Python.h") ||
            inc.path.contains("object.h") ||
            inc.path.contains("pyport.h")
        })
    }

    fn analyze_api_calls(&self, ast: &CAST) -> Vec<RecognizedAPICall> {
        let mut calls = Vec::new();

        for func_call in ast.find_all_function_calls() {
            match func_call.name.as_str() {
                // Reference counting
                "Py_INCREF" => calls.push(RecognizedAPICall::RefCount {
                    operation: RefCountOp::Increment,
                    object: func_call.args[0].clone(),
                }),
                "Py_DECREF" => calls.push(RecognizedAPICall::RefCount {
                    operation: RefCountOp::Decrement,
                    object: func_call.args[0].clone(),
                }),

                // GIL operations
                "PyGILState_Ensure" => calls.push(RecognizedAPICall::GIL {
                    operation: GILOp::Acquire,
                }),
                "PyGILState_Release" => calls.push(RecognizedAPICall::GIL {
                    operation: GILOp::Release,
                }),

                // Dict operations
                "PyDict_GetItem" => calls.push(RecognizedAPICall::Dict {
                    operation: DictOp::Get,
                    dict: func_call.args[0].clone(),
                    key: func_call.args[1].clone(),
                }),

                // List operations
                "PyList_Append" => calls.push(RecognizedAPICall::List {
                    operation: ListOp::Append,
                    list: func_call.args[0].clone(),
                    item: func_call.args[1].clone(),
                }),

                _ => {}
            }
        }

        calls
    }

    fn generate_rust_bindings(&self, api_call: &RecognizedAPICall) -> RustCode {
        match api_call {
            RecognizedAPICall::RefCount { operation, object } => {
                match operation {
                    RefCountOp::Increment => quote! {
                        Arc::clone(&#object)
                    },
                    RefCountOp::Decrement => quote! {
                        drop(#object)
                    },
                }
            }
            RecognizedAPICall::Dict { operation: DictOp::Get, dict, key } => {
                quote! {
                    #dict.get(&#key)
                }
            }
            RecognizedAPICall::List { operation: ListOp::Append, list, item } => {
                quote! {
                    #list.push(#item)
                }
            }
            // ... other cases
        }
    }
}
```

---

### 2. NumPy Analyzer

```rust
// spydecy-analyzers/src/numpy.rs

pub struct NumPyAnalyzer;

impl C_API_Analyzer for NumPyAnalyzer {
    fn name(&self) -> &str {
        "NumPy 1.20+"
    }

    fn priority(&self) -> u32 {
        150  // Medium-high priority
    }

    fn can_analyze(&self, ast: &CAST, context: &AnalysisContext) -> bool {
        // Check for NumPy headers
        ast.includes.iter().any(|inc| {
            inc.path.contains("numpy/arrayobject.h") ||
            inc.path.contains("numpy/ndarrayobject.h")
        })
    }

    fn analyze_types(&self, ast: &CAST) -> TypeAnalysis {
        let mut type_analysis = TypeAnalysis::new();

        // Recognize NumPy types
        for type_decl in ast.find_all_type_declarations() {
            match type_decl.name.as_str() {
                "PyArrayObject" => {
                    type_analysis.add_mapping(
                        "PyArrayObject*",
                        "ndarray::ArrayD<f64>",  // Rust ndarray type
                    );
                }
                "PyArray_Descr" => {
                    type_analysis.add_mapping(
                        "PyArray_Descr*",
                        "ndarray::ArrayDtype",
                    );
                }
                // ... other NumPy types
                _ => {}
            }
        }

        type_analysis
    }

    fn analyze_api_calls(&self, ast: &CAST) -> Vec<RecognizedAPICall> {
        let mut calls = Vec::new();

        for func_call in ast.find_all_function_calls() {
            match func_call.name.as_str() {
                // Array creation
                "PyArray_SimpleNew" => calls.push(RecognizedAPICall::NumPy {
                    operation: NumPyOp::CreateArray {
                        ndim: func_call.args[0].clone(),
                        shape: func_call.args[1].clone(),
                        dtype: func_call.args[2].clone(),
                    },
                }),

                // Array access
                "PyArray_GETPTR1" => calls.push(RecognizedAPICall::NumPy {
                    operation: NumPyOp::GetElement {
                        array: func_call.args[0].clone(),
                        index: func_call.args[1].clone(),
                    },
                }),

                // Array operations
                "PyArray_Sum" => calls.push(RecognizedAPICall::NumPy {
                    operation: NumPyOp::Sum {
                        array: func_call.args[0].clone(),
                        axis: func_call.args[1].clone(),
                    },
                }),

                // Broadcasting
                "PyArray_Broadcast" => calls.push(RecognizedAPICall::NumPy {
                    operation: NumPyOp::Broadcast {
                        arrays: func_call.args.clone(),
                    },
                }),

                _ => {}
            }
        }

        calls
    }

    fn generate_rust_bindings(&self, api_call: &RecognizedAPICall) -> RustCode {
        match api_call {
            RecognizedAPICall::NumPy { operation } => match operation {
                NumPyOp::CreateArray { ndim, shape, dtype } => {
                    quote! {
                        ndarray::Array::from_shape_vec(
                            ndarray::IxDyn(#shape),
                            vec![0.0; #shape.iter().product()]
                        ).unwrap()
                    }
                }
                NumPyOp::GetElement { array, index } => {
                    quote! {
                        #array[#index]
                    }
                }
                NumPyOp::Sum { array, axis } => {
                    quote! {
                        #array.sum_axis(ndarray::Axis(#axis))
                    }
                }
                // ... other cases
            }
        }
    }
}
```

---

### 3. SciPy Analyzer

```rust
// spydecy-analyzers/src/scipy.rs

pub struct SciPyAnalyzer;

impl C_API_Analyzer for SciPyAnalyzer {
    fn name(&self) -> &str {
        "SciPy 1.7+"
    }

    fn can_analyze(&self, ast: &CAST, context: &AnalysisContext) -> bool {
        // Check for SciPy headers or BLAS/LAPACK calls
        ast.includes.iter().any(|inc| {
            inc.path.contains("scipy/") ||
            inc.path.contains("cblas.h") ||
            inc.path.contains("lapacke.h")
        })
    }

    fn analyze_api_calls(&self, ast: &CAST) -> Vec<RecognizedAPICall> {
        let mut calls = Vec::new();

        for func_call in ast.find_all_function_calls() {
            match func_call.name.as_str() {
                // BLAS Level 1 (vector operations)
                "cblas_daxpy" => calls.push(RecognizedAPICall::BLAS {
                    level: 1,
                    operation: BLASOp::AXPY {
                        alpha: func_call.args[0].clone(),
                        x: func_call.args[1].clone(),
                        y: func_call.args[2].clone(),
                    },
                }),

                // BLAS Level 2 (matrix-vector)
                "cblas_dgemv" => calls.push(RecognizedAPICall::BLAS {
                    level: 2,
                    operation: BLASOp::GEMV {
                        matrix: func_call.args[0].clone(),
                        vector: func_call.args[1].clone(),
                    },
                }),

                // BLAS Level 3 (matrix-matrix)
                "cblas_dgemm" => calls.push(RecognizedAPICall::BLAS {
                    level: 3,
                    operation: BLASOp::GEMM {
                        a: func_call.args[0].clone(),
                        b: func_call.args[1].clone(),
                        c: func_call.args[2].clone(),
                    },
                }),

                // LAPACK (linear algebra)
                "dgesv_" => calls.push(RecognizedAPICall::LAPACK {
                    operation: LAPACKOp::SolveLinearSystem {
                        matrix: func_call.args[0].clone(),
                        rhs: func_call.args[1].clone(),
                    },
                }),

                _ => {}
            }
        }

        calls
    }

    fn generate_rust_bindings(&self, api_call: &RecognizedAPICall) -> RustCode {
        match api_call {
            RecognizedAPICall::BLAS { level: 1, operation } => match operation {
                BLASOp::AXPY { alpha, x, y } => {
                    quote! {
                        // Use ndarray-linalg for BLAS operations
                        use ndarray_linalg::*;
                        #y.scaled_add(#alpha, &#x)
                    }
                }
                // ... other BLAS ops
            }
            RecognizedAPICall::LAPACK { operation } => match operation {
                LAPACKOp::SolveLinearSystem { matrix, rhs } => {
                    quote! {
                        use ndarray_linalg::Solve;
                        #matrix.solve(&#rhs).unwrap()
                    }
                }
                // ... other LAPACK ops
            }
        }
    }
}
```

---

## Analyzer Registry & Discovery

```rust
// spydecy-analyzers/src/registry.rs

pub struct AnalyzerRegistry {
    analyzers: Vec<Box<dyn C_API_Analyzer>>,
}

impl AnalyzerRegistry {
    pub fn new() -> Self {
        Self {
            analyzers: Vec::new(),
        }
    }

    /// Register a built-in analyzer
    pub fn register(&mut self, analyzer: Box<dyn C_API_Analyzer>) {
        self.analyzers.push(analyzer);
    }

    /// Auto-discover analyzers from plugins
    pub fn discover_plugins(&mut self, plugin_dir: &Path) -> Result<()> {
        for entry in std::fs::read_dir(plugin_dir)? {
            let path = entry?.path();
            if path.extension() == Some(std::ffi::OsStr::new("so")) {
                // Load dynamic library
                let lib = libloading::Library::new(&path)?;

                // Get analyzer factory function
                let factory: libloading::Symbol<fn() -> Box<dyn C_API_Analyzer>> =
                    unsafe { lib.get(b"create_analyzer")? };

                // Create and register analyzer
                let analyzer = factory();
                self.register(analyzer);
            }
        }
        Ok(())
    }

    /// Get all analyzers that can handle this file
    pub fn select_analyzers(
        &self,
        ast: &CAST,
        context: &AnalysisContext,
    ) -> Vec<&Box<dyn C_API_Analyzer>> {
        let mut selected: Vec<_> = self.analyzers.iter()
            .filter(|a| a.can_analyze(ast, context))
            .collect();

        // Sort by priority (highest first)
        selected.sort_by_key(|a| std::cmp::Reverse(a.priority()));

        selected
    }
}
```

---

## Plugin Development

Third-party developers can create custom analyzers:

```rust
// my-custom-analyzer/src/lib.rs

use spydecy_analyzers::C_API_Analyzer;

pub struct TensorFlowAnalyzer;

impl C_API_Analyzer for TensorFlowAnalyzer {
    fn name(&self) -> &str {
        "TensorFlow C API"
    }

    fn can_analyze(&self, ast: &CAST, context: &AnalysisContext) -> bool {
        ast.includes.iter().any(|inc| inc.path.contains("tensorflow/c/"))
    }

    fn analyze_api_calls(&self, ast: &CAST) -> Vec<RecognizedAPICall> {
        // Custom TensorFlow analysis
        // ...
    }

    fn generate_rust_bindings(&self, api_call: &RecognizedAPICall) -> RustCode {
        // Generate Rust code using tch-rs (Rust TensorFlow bindings)
        // ...
    }
}

// Plugin entry point
#[no_mangle]
pub extern "C" fn create_analyzer() -> Box<dyn C_API_Analyzer> {
    Box::new(TensorFlowAnalyzer)
}
```

**Usage**:
```bash
# Install third-party analyzer
$ spydecy plugin install tensorflow-analyzer.so

# Transpile TensorFlow C code
$ spydecy transpile my_tensorflow_model.c
```

---

## Benefits of Pluggable Architecture

### 1. Ecosystem Transformation
- ✅ **CPython**: Core Python implementation
- ✅ **NumPy**: Scientific computing arrays
- ✅ **SciPy**: BLAS/LAPACK linear algebra
- ✅ **Pandas**: Data manipulation
- ✅ **Pillow**: Image processing
- ✅ **scikit-learn**: Machine learning
- ✅ **TensorFlow**: Deep learning (via plugin)
- ✅ **PyTorch**: Deep learning (via plugin)

**Impact**: Can transpile the **entire Python scientific stack** to safe, fast Rust.

### 2. Community Contributions
- Domain experts maintain analyzers for their libraries
- NumPy team maintains `NumPyAnalyzer`
- SciPy team maintains `SciPyAnalyzer`
- Third-party developers create custom analyzers

### 3. Incremental Development
- **Phase 1**: Ship with `CPythonAnalyzer` only
- **Phase 2**: Add `NumPyAnalyzer`
- **Phase 3**: Add `SciPyAnalyzer`
- **Phase 4**: Open plugin system to community

### 4. Separation of Concerns
- Core transpiler doesn't need to know about every C API
- Each analyzer is independently testable
- Analyzers can be versioned separately (e.g., `NumPyAnalyzer-1.20` vs `NumPyAnalyzer-2.0`)

---

## Roadmap Integration

### Phase 1 (Sprints 1-5): CPython Only
- Implement `C_API_Analyzer` trait
- Build `CPythonAnalyzer` as first implementation
- Validate architecture with tracer bullet

### Phase 2 (Sprints 6-10): NumPy Support
- Implement `NumPyAnalyzer`
- Test on NumPy core arrays
- Validate cross-layer optimization with NumPy

### Phase 3 (Sprints 11-15): SciPy Support
- Implement `SciPyAnalyzer`
- BLAS/LAPACK mapping to `ndarray-linalg`
- Test on SciPy linear algebra routines

### Phase 4 (Sprints 16-20): Plugin System
- Publish plugin API
- Document plugin development
- Community contributions (TensorFlow, PyTorch, etc.)

---

## Testing Strategy

### Per-Analyzer Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpython_analyzer_refcount() {
        let c_code = r#"
            Py_INCREF(obj);
            Py_DECREF(obj);
        "#;

        let ast = parse_c(c_code).unwrap();
        let analyzer = CPythonAnalyzer;
        let calls = analyzer.analyze_api_calls(&ast);

        assert_eq!(calls.len(), 2);
        assert!(matches!(calls[0], RecognizedAPICall::RefCount { .. }));
    }

    #[test]
    fn test_numpy_analyzer_array_creation() {
        let c_code = r#"
            PyArrayObject* arr = PyArray_SimpleNew(2, dims, NPY_FLOAT64);
        "#;

        let ast = parse_c(c_code).unwrap();
        let analyzer = NumPyAnalyzer;
        let calls = analyzer.analyze_api_calls(&ast);

        assert_eq!(calls.len(), 1);
        assert!(matches!(calls[0], RecognizedAPICall::NumPy { .. }));
    }
}
```

---

## Conclusion

The pluggable C-API architecture transforms Spydecy from a **CPython transpiler** into a **platform for transpiling the entire Python C-extension ecosystem**.

**Key Innovations**:
1. **Trait-based design**: Clean separation between core transpiler and API-specific knowledge
2. **Plugin system**: Community can contribute analyzers for their favorite libraries
3. **Incremental rollout**: Ship with CPython support, add NumPy/SciPy later
4. **Ecosystem transformation**: Can transpile NumPy, SciPy, Pandas, scikit-learn, and more

**Impact**: This architectural decision multiplies the project's value by **10x**, enabling the transpilation of the entire Python scientific computing stack to safe, high-performance Rust.

---

**Status**: ARCHITECTURE COMPLETE
**Next Step**: Implement `C_API_Analyzer` trait in Sprint 3
**Expected Impact**: Enables transpilation of entire Python ecosystem
