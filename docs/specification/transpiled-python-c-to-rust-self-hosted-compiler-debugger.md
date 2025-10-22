# Spydecy: Self-Hosted Python/C-to-Rust Compiler-Debugger
## Unified Transpilation Architecture with Introspective Debugging

**Version**: 1.0
**Date**: 2025-10-21
**Status**: SPECIFICATION
**Methodology**: EXTREME TDD + Toyota Way + PMAT Quality Gates

---

## Executive Summary

**Spydecy** is a self-hosted compiler-debugger that provides unified Python and C-to-Rust transpilation with unique introspective debugging capabilities. By leveraging the fact that Python (CPython) is written in C, Spydecy creates a closed-loop system where:

1. **Python → Rust transpilation** (via depyler integration)
2. **C → Rust transpilation** (via decy integration)
3. **Self-hosting capability** enables the transpiler to transpile itself
4. **Introspective debugging** allows debugging transpilation issues in both CPython and Python libraries

### Core Innovation

The key insight is that **CPython is written in C**, so a unified Python/C transpiler can:
- Transpile Python source code to Rust
- Transpile CPython's C implementation to Rust
- Debug both layers simultaneously during transpilation
- Identify and fix tricky issues at the Python-C boundary
- Convert entire Python ecosystems (CPython, numpy, pandas) to Rust

---

## Architecture Overview

### Multi-Layer Transpilation Pipeline

```
┌─────────────────────────────────────────────────────────────────┐
│                      SPYDECY ARCHITECTURE                        │
└─────────────────────────────────────────────────────────────────┘

Layer 1: Python Source Transpilation
────────────────────────────────────
    Python Source Code (.py)
            ↓
    ┌──────────────────┐
    │  Python Parser   │ (AST via ast module + custom extensions)
    │  + Type Hints    │
    └────────┬─────────┘
             ↓
    ┌──────────────────┐
    │   Python HIR     │ (High-level Intermediate Representation)
    │  + Type System   │
    └────────┬─────────┘
             ↓
    ┌──────────────────┐
    │  Semantic        │ (Effects, ownership patterns, GC analysis)
    │  Analysis        │
    └────────┬─────────┘
             ↓

Layer 2: CPython C Implementation
──────────────────────────────────
    CPython C Source (.c/.h)
            ↓
    ┌──────────────────┐
    │  C Parser        │ (clang-sys integration from decy)
    │  + Preprocessor  │
    └────────┬─────────┘
             ↓
    ┌──────────────────┐
    │   C HIR          │ (C-level IR with Python object model)
    │  + CPython API   │
    └────────┬─────────┘
             ↓
    ┌──────────────────┐
    │  C Memory        │ (malloc/free → Box, ref counting analysis)
    │  Analysis        │
    └────────┬─────────┘
             ↓

Layer 3: Unified Rust Generation
─────────────────────────────────
             ┌─────────────────────────┐
             │  Cross-Layer Optimizer   │
             │  - Eliminate boundaries  │
             │  - Inline Python→C calls │
             │  - Unify ownership       │
             └───────────┬─────────────┘
                         ↓
             ┌─────────────────────────┐
             │   Unified Rust Codegen   │
             │   - Safe Rust (minimize) │
             │   - Zero-copy where safe │
             │   - Arc<> for ref count  │
             └───────────┬─────────────┘
                         ↓
             ┌─────────────────────────┐
             │  Introspective Debugger  │
             │  - Step through layers   │
             │  - Visualize transforms  │
             │  - Identify issues       │
             └───────────┬─────────────┘
                         ↓
                   Rust Code
                   + Test Suite
                   + Debug Info

Layer 4: Self-Hosting
──────────────────────
    Spydecy (Rust) ──┐
           ↓          │
    Spydecy.py       │
           ↓          │  Self-Transpilation
    Spydecy (Rust')──┘  (validates correctness)
```

---

## Core Components

### 1. Python Transpilation Engine (Depyler Integration)

```rust
// spydecy-python/src/lib.rs

pub struct PythonTranspiler {
    // AST parsing
    parser: PythonParser,

    // Type inference (leveraging type hints + gradual typing)
    type_checker: GradualTypeChecker,

    // Python-specific optimizations
    optimizer: PythonOptimizer,

    // Integration with depyler
    depyler_backend: DepylerBackend,
}

pub struct PythonParser {
    // Use Python's ast module via PyO3
    py_ast: PyAstParser,

    // Custom extensions for advanced features
    extensions: Vec<SyntaxExtension>,

    // Type hint extraction
    type_hints: TypeHintExtractor,
}

impl PythonTranspiler {
    pub fn transpile(&self, source: &str) -> Result<RustCode> {
        // 1. Parse Python to AST
        let py_ast = self.parser.parse(source)?;

        // 2. Type inference with gradual typing
        let typed_ast = self.type_checker.infer_types(&py_ast)?;

        // 3. Convert to Spydecy HIR
        let hir = self.lower_to_hir(&typed_ast)?;

        // 4. Analyze Python semantics
        let analyzed = self.analyze_semantics(&hir)?;

        // 5. Generate Rust via depyler integration
        let rust_code = self.depyler_backend.generate_rust(&analyzed)?;

        Ok(rust_code)
    }

    fn analyze_semantics(&self, hir: &PyHIR) -> Result<AnalyzedHIR> {
        // Analyze Python-specific patterns
        let mut analyzed = AnalyzedHIR::new();

        // 1. Reference counting patterns (convert to Arc/Rc)
        analyzed.refcount_info = self.analyze_refcounting(hir)?;

        // 2. GIL usage (convert to appropriate synchronization)
        analyzed.gil_info = self.analyze_gil_usage(hir)?;

        // 3. Duck typing patterns (convert to trait objects)
        analyzed.duck_typing = self.analyze_duck_typing(hir)?;

        // 4. Dynamic dispatch (convert to enum dispatch)
        analyzed.dynamic_dispatch = self.analyze_dynamic_dispatch(hir)?;

        Ok(analyzed)
    }
}
```

### 2. C Transpilation Engine (Decy Integration)

```rust
// spydecy-c/src/lib.rs

pub struct CTranspiler {
    // C parser via clang
    parser: CParser,

    // CPython-aware analysis
    cpython_analyzer: CPythonAnalyzer,

    // Ownership inference
    ownership: OwnershipAnalyzer,

    // Integration with decy
    decy_backend: DecyBackend,
}

pub struct CPythonAnalyzer {
    // CPython object model understanding
    object_model: CPythonObjectModel,

    // PyObject* analysis
    pyobject_tracker: PyObjectTracker,

    // Reference counting analysis
    refcount_analyzer: RefCountAnalyzer,

    // GIL analysis
    gil_analyzer: GILAnalyzer,
}

impl CTranspiler {
    pub fn transpile(&self, source: &str) -> Result<RustCode> {
        // 1. Parse C code via clang
        let c_ast = self.parser.parse(source)?;

        // 2. Identify CPython-specific patterns
        let cpython_info = self.cpython_analyzer.analyze(&c_ast)?;

        // 3. Convert to Spydecy HIR
        let hir = self.lower_to_hir(&c_ast, &cpython_info)?;

        // 4. Ownership and lifetime inference
        let analyzed = self.ownership.analyze(&hir)?;

        // 5. Generate Rust via decy integration
        let rust_code = self.decy_backend.generate_rust(&analyzed)?;

        Ok(rust_code)
    }
}

impl CPythonAnalyzer {
    pub fn analyze(&self, ast: &CAST) -> Result<CPythonInfo> {
        let mut info = CPythonInfo::new();

        // 1. Identify PyObject* usage patterns
        info.pyobject_usage = self.pyobject_tracker.analyze(ast)?;

        // 2. Reference counting operations
        //    Py_INCREF/Py_DECREF → Arc::clone/drop
        info.refcount_ops = self.refcount_analyzer.find_refcount_ops(ast)?;

        // 3. GIL operations
        //    Py_BEGIN_ALLOW_THREADS → drop GIL guard
        info.gil_ops = self.gil_analyzer.find_gil_ops(ast)?;

        // 4. CPython API calls
        //    PyDict_GetItem → safe Rust HashMap operations
        info.api_calls = self.identify_cpython_api_calls(ast)?;

        Ok(info)
    }
}
```

### 3. Unified HIR (High-Level Intermediate Representation)

```rust
// spydecy-hir/src/lib.rs

/// Unified HIR that represents both Python and C constructs
pub struct UnifiedHIR {
    pub modules: Vec<Module>,
    pub type_system: TypeSystem,
    pub metadata: Metadata,
}

pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
    pub classes: Vec<Class>,
    pub globals: Vec<Global>,
    pub source_language: SourceLanguage,
}

pub enum SourceLanguage {
    Python { version: PythonVersion },
    C { cpython_api_version: String },
    Rust,  // For self-hosting
}

pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
    pub attributes: FunctionAttributes,
}

pub struct FunctionAttributes {
    // Python attributes
    pub is_coroutine: bool,
    pub is_generator: bool,
    pub decorators: Vec<Decorator>,

    // C attributes
    pub is_static: bool,
    pub is_inline: bool,
    pub linkage: Linkage,

    // CPython attributes
    pub uses_gil: bool,
    pub refcount_operations: Vec<RefCountOp>,
    pub cpython_api_calls: Vec<CPythonAPICall>,
}

pub enum Type {
    // Python types
    PythonInt,
    PythonFloat,
    PythonStr,
    PythonList(Box<Type>),
    PythonDict(Box<Type>, Box<Type>),
    PythonTuple(Vec<Type>),
    PythonClass(ClassRef),
    PythonAny,  // dynamic typing

    // C types
    CInt { width: u8, signed: bool },
    CFloat { width: u8 },
    CPointer { inner: Box<Type>, mutability: Mutability },
    CStruct(StructRef),

    // CPython types
    PyObject,  // PyObject*
    PyTypeObject,  // PyTypeObject*
    PyDictObject,  // PyDictObject*

    // Rust types (for self-hosting)
    RustType(RustTypeRef),

    // Unified types
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Reference { inner: Box<Type>, lifetime: Lifetime, mutability: Mutability },
    Arc(Box<Type>),  // For reference counting
    Rc(Box<Type>),
}
```

### 4. Cross-Layer Optimizer

```rust
// spydecy-optimizer/src/lib.rs

pub struct CrossLayerOptimizer {
    // Eliminate Python-C boundary overhead
    boundary_eliminator: BoundaryEliminator,

    // Inline Python calls to C implementations
    inliner: CrossLanguageInliner,

    // Unify ownership across layers
    ownership_unifier: OwnershipUnifier,

    // Reference counting optimization
    refcount_optimizer: RefCountOptimizer,
}

impl CrossLayerOptimizer {
    pub fn optimize(&self, hir: &UnifiedHIR) -> Result<OptimizedHIR> {
        let mut optimized = hir.clone();

        // 1. Eliminate Python → C call boundaries
        //    Example: list.append() → direct Vec::push
        optimized = self.boundary_eliminator.eliminate_boundaries(&optimized)?;

        // 2. Inline small Python→C calls
        //    Example: PyDict_GetItem → HashMap::get
        optimized = self.inliner.inline_cross_language_calls(&optimized)?;

        // 3. Unify ownership models
        //    Python ref counting + C malloc/free → unified Arc/Box
        optimized = self.ownership_unifier.unify(&optimized)?;

        // 4. Optimize reference counting
        //    Eliminate redundant Py_INCREF/Py_DECREF
        optimized = self.refcount_optimizer.optimize(&optimized)?;

        Ok(optimized)
    }
}

pub struct BoundaryEliminator;

impl BoundaryEliminator {
    pub fn eliminate_boundaries(&self, hir: &UnifiedHIR) -> Result<UnifiedHIR> {
        let mut eliminated = hir.clone();

        for module in &mut eliminated.modules {
            for func in &mut module.functions {
                // Find Python→C calls
                let calls = self.find_python_to_c_calls(&func.body);

                for call in calls {
                    // If the C implementation is simple, inline it
                    if let Some(c_impl) = self.find_c_implementation(&call) {
                        if self.is_inlineable(&c_impl) {
                            // Replace call with inlined C → Rust
                            self.inline_call(&mut func.body, &call, &c_impl)?;
                        }
                    }
                }
            }
        }

        Ok(eliminated)
    }
}
```

### 5. Introspective Debugger

```rust
// spydecy-debugger/src/lib.rs

/// Interactive debugger for transpilation process
pub struct IntrospectiveDebugger {
    // Transpilation state
    state: TranspilationState,

    // Breakpoints
    breakpoints: Vec<Breakpoint>,

    // Visualization
    visualizer: TransformVisualizer,

    // Issue tracker
    issues: IssueTracker,
}

pub struct TranspilationState {
    // Current source (Python or C)
    pub source: SourceCode,

    // Current HIR
    pub hir: UnifiedHIR,

    // Current Rust output
    pub rust_output: Option<RustCode>,

    // Transformation history
    pub history: Vec<Transformation>,
}

pub enum Breakpoint {
    // Break on specific Python construct
    PythonConstruct(PythonNodeKind),

    // Break on specific C construct
    CConstruct(CNodeKind),

    // Break on CPython API call
    CPythonAPI(String),

    // Break on ownership inference
    OwnershipInference,

    // Break on type error
    TypeError,

    // Break on unsafe code generation
    UnsafeGeneration,
}

impl IntrospectiveDebugger {
    pub fn start_session(&mut self) -> Result<()> {
        println!("Spydecy Interactive Debugger");
        println!("Commands: step, continue, break, inspect, visualize, quit");

        loop {
            let command = self.read_command()?;

            match command {
                Command::Step => self.step()?,
                Command::Continue => self.continue_until_breakpoint()?,
                Command::Break(bp) => self.add_breakpoint(bp),
                Command::Inspect(target) => self.inspect(target)?,
                Command::Visualize => self.visualize_current_state()?,
                Command::IssuesReport => self.show_issues()?,
                Command::Quit => break,
            }
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<()> {
        // Execute one transformation step
        let next_transform = self.state.next_transformation()?;

        println!("Applying transformation: {}", next_transform.name());

        // Apply transformation
        let new_state = next_transform.apply(&self.state)?;

        // Check for issues
        if let Some(issue) = self.issues.check_transformation(&next_transform) {
            println!("⚠️  Issue detected: {}", issue.description);
            println!("   Recommendation: {}", issue.recommendation);
        }

        self.state = new_state;
        self.visualize_current_state()?;

        Ok(())
    }

    pub fn visualize_current_state(&self) -> Result<()> {
        println!("\n═══ Current State ═══");

        // Show source
        println!("\n📄 Source ({}):", self.state.source.language);
        self.visualizer.show_source_snippet(&self.state.source)?;

        // Show HIR
        println!("\n🔧 HIR:");
        self.visualizer.show_hir_summary(&self.state.hir)?;

        // Show Rust output (if available)
        if let Some(rust) = &self.state.rust_output {
            println!("\n🦀 Rust Output:");
            self.visualizer.show_rust_snippet(rust)?;
        }

        // Show ownership info
        println!("\n🔐 Ownership:");
        self.visualizer.show_ownership_graph(&self.state.hir)?;

        Ok(())
    }

    pub fn identify_cpython_numpy_issues(&self) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check for common CPython transpilation issues
        issues.extend(self.check_refcount_issues());
        issues.extend(self.check_gil_issues());
        issues.extend(self.check_pyobject_lifetime_issues());

        // Check for numpy-specific issues
        issues.extend(self.check_numpy_array_issues());
        issues.extend(self.check_numpy_dtype_issues());

        issues
    }
}
```

### 6. Self-Hosting Capability

```rust
// spydecy-bootstrap/src/lib.rs

/// Bootstrap compiler for self-hosting
pub struct BootstrapCompiler {
    // Stage 0: Hand-written Rust compiler
    stage0: Box<dyn Compiler>,

    // Stage 1: Spydecy written in Python, transpiled by stage0
    stage1: Option<Box<dyn Compiler>>,

    // Stage 2: Spydecy transpiled by stage1 (validates correctness)
    stage2: Option<Box<dyn Compiler>>,
}

impl BootstrapCompiler {
    pub fn bootstrap(&mut self) -> Result<()> {
        println!("Starting Spydecy bootstrap process...");

        // Stage 0 → Stage 1
        println!("\n[Stage 0 → Stage 1]");
        println!("Transpiling spydecy.py using stage0 (Rust)...");

        let spydecy_py = std::fs::read_to_string("spydecy/spydecy.py")?;
        let stage1_rust = self.stage0.compile(&spydecy_py)?;

        // Build stage1
        println!("Building stage1 compiler...");
        self.build_rust(&stage1_rust, "stage1")?;

        self.stage1 = Some(self.load_compiler("stage1")?);

        // Stage 1 → Stage 2
        println!("\n[Stage 1 → Stage 2]");
        println!("Transpiling spydecy.py using stage1 (self-hosted)...");

        let stage2_rust = self.stage1.as_ref().unwrap().compile(&spydecy_py)?;

        // Build stage2
        println!("Building stage2 compiler...");
        self.build_rust(&stage2_rust, "stage2")?;

        self.stage2 = Some(self.load_compiler("stage2")?);

        // Validation: Compare stage1 and stage2 output
        println!("\n[Validation]");
        println!("Comparing stage1 and stage2 outputs...");

        let test_program = include_str!("test_program.py");
        let output1 = self.stage1.as_ref().unwrap().compile(test_program)?;
        let output2 = self.stage2.as_ref().unwrap().compile(test_program)?;

        if output1 == output2 {
            println!("✅ Bootstrap successful! Outputs match.");
        } else {
            println!("❌ Bootstrap failed. Outputs differ.");
            self.show_diff(&output1, &output2)?;
            return Err(BootstrapError::OutputMismatch);
        }

        Ok(())
    }
}
```

---

## Testing Strategy (EXTREME TDD)

### 1. Multi-Layer Testing

```rust
// tests/integration/multi_layer_tests.rs

#[test]
fn test_python_to_rust_list_operations() {
    // Python source
    let python_code = r#"
def process_list(items):
    result = []
    for item in items:
        if item > 0:
            result.append(item * 2)
    return result
"#;

    // Transpile
    let rust_code = transpile_python(python_code).unwrap();

    // Verify generated Rust
    assert!(rust_code.contains("Vec<"));
    assert!(!rust_code.contains("unsafe"));

    // Compile and execute
    let compiled = compile_rust(&rust_code).unwrap();
    let result = execute(&compiled, &[1, -2, 3, 4]).unwrap();

    // Verify behavior matches Python
    assert_eq!(result, vec![2, 6, 8]);
}

#[test]
fn test_cpython_dict_transpilation() {
    // C source from CPython
    let c_code = r#"
PyObject *
PyDict_GetItem(PyObject *op, PyObject *key)
{
    if (!PyDict_Check(op)) {
        return NULL;
    }
    PyDictObject *mp = (PyDictObject *)op;
    // ... implementation
}
"#;

    // Transpile
    let rust_code = transpile_c(c_code).unwrap();

    // Verify generated Rust
    assert!(rust_code.contains("Option<"));
    assert!(rust_code.contains("HashMap"));
    assert!(!rust_code.contains("*mut"));  // No raw pointers in safe code
}

#[test]
fn test_unified_python_c_optimization() {
    // Python code that calls CPython C API internally
    let python_code = r#"
def get_value(d, key):
    return d.get(key, None)
"#;

    // Transpile with cross-layer optimization
    let rust_code = transpile_unified(python_code).unwrap();

    // Should inline directly to HashMap::get (no boundary)
    assert!(rust_code.contains("HashMap::get"));
    assert!(!rust_code.contains("cpython_api"));  // No FFI boundary
}
```

### 2. Property-Based Testing

```rust
// tests/property/transpilation_properties.rs

use proptest::prelude::*;

proptest! {
    /// Property: Python → Rust → execution matches Python execution
    #[test]
    fn python_rust_equivalence(
        python_code in valid_python_program()
    ) {
        // Execute with Python
        let python_output = execute_python(&python_code)?;

        // Transpile to Rust
        let rust_code = transpile_python(&python_code)?;

        // Execute Rust
        let rust_output = execute_rust(&rust_code)?;

        // Outputs must match
        prop_assert_eq!(python_output, rust_output);
    }

    /// Property: C → Rust preserves CPython semantics
    #[test]
    fn cpython_rust_equivalence(
        c_code in valid_cpython_code()
    ) {
        // Compile with gcc
        let c_output = compile_and_execute_c(&c_code)?;

        // Transpile to Rust
        let rust_code = transpile_c(&c_code)?;

        // Compile and execute Rust
        let rust_output = compile_and_execute_rust(&rust_code)?;

        // Behavior must match
        prop_assert_eq!(c_output, rust_output);
    }

    /// Property: Self-hosting is idempotent
    #[test]
    fn self_hosting_idempotence(
        spydecy_code in valid_spydecy_compiler_code()
    ) {
        // Stage 1: Transpile with stage0
        let stage1_rust = stage0_transpile(&spydecy_code)?;

        // Stage 2: Transpile with stage1
        let stage2_rust = stage1_transpile(&spydecy_code)?;

        // Stage 3: Transpile with stage2
        let stage3_rust = stage2_transpile(&spydecy_code)?;

        // Stage 2 and 3 should be identical (fixed point)
        prop_assert_eq!(stage2_rust, stage3_rust);
    }
}
```

### 3. Mutation Testing

```toml
# .cargo-mutants.toml

[[mutants]]
include = ["src/python_transpiler/*.rs"]
operators = [
    "refcount_operations",   # Mutate Py_INCREF/Py_DECREF patterns
    "gil_operations",        # Mutate GIL acquire/release
    "type_inference",        # Mutate type inference decisions
]

[[mutants]]
include = ["src/c_transpiler/*.rs"]
operators = [
    "pointer_analysis",      # Mutate pointer → reference conversion
    "ownership_inference",   # Mutate Box vs Arc decisions
    "lifetime_inference",    # Mutate lifetime annotations
]

[[mutants]]
include = ["src/cross_layer_optimizer/*.rs"]
operators = [
    "boundary_elimination",  # Mutate inlining decisions
    "refcount_optimization", # Mutate ref counting optimizations
]

timeout_multiplier = 5
minimum_test_time_ms = 100
```

### 4. Differential Testing

```rust
// tests/differential/cpython_numpy_tests.rs

#[test]
fn test_cpython_dict_operations() {
    let corpus = load_cpython_dict_tests();

    for test_case in corpus {
        // Execute with CPython
        let cpython_result = execute_cpython(&test_case.python_code)?;

        // Transpile dict implementation to Rust
        let rust_dict = transpile_dict_implementation()?;

        // Execute with Rust implementation
        let rust_result = execute_rust_with_transpiled_dict(
            &test_case.python_code,
            &rust_dict
        )?;

        // Must match
        assert_eq!(cpython_result, rust_result);
    }
}

#[test]
fn test_numpy_array_operations() {
    let corpus = load_numpy_test_suite();

    for test_case in corpus {
        // Execute with numpy
        let numpy_result = execute_numpy(&test_case.code)?;

        // Transpile numpy array implementation
        let rust_array = transpile_numpy_ndarray()?;

        // Execute with Rust
        let rust_result = execute_rust_with_transpiled_ndarray(
            &test_case.code,
            &rust_array
        )?;

        // Must match (within floating point tolerance)
        assert_approx_eq!(numpy_result, rust_result, tolerance = 1e-10);
    }
}
```

---

## Quality Gates (PMAT Integration)

```toml
# spydecy-quality.toml

[complexity]
cyclomatic_threshold = 10
cognitive_threshold = 15
max_nesting_depth = 4
max_function_lines = 80

[satd]
enabled = true
zero_tolerance = true
patterns = ["TODO", "FIXME", "HACK", "XXX", "WORKAROUND", "BUG"]

[coverage]
minimum_coverage = 80.0
enforce_on_new_code = true
target_modules = [
    "spydecy-python/",
    "spydecy-c/",
    "spydecy-hir/",
    "spydecy-optimizer/",
    "spydecy-codegen/",
    "spydecy-debugger/",
]

[mutation_testing]
enabled = true
minimum_kill_rate = 0.90
timeout_seconds = 300
target_modules = ["python_transpiler", "c_transpiler", "optimizer", "debugger"]

[property_testing]
enabled = true
minimum_properties = 100
cases_per_property = 1000

[verification]
enabled = true
require_clippy_pass = true
require_rustfmt_pass = true
require_cargo_test_pass = true
require_self_hosting_pass = true  # Unique to spydecy

[security]
max_unsafe_blocks = 5
check_memory_safety = true
check_cpython_api_safety = true
```

---

## Use Cases & Applications

### 1. CPython to Rust

```bash
# Transpile CPython's dict implementation
spydecy transpile-cpython Objects/dictobject.c \
    --output cpython-rust/dict.rs \
    --optimize cross-layer \
    --verify

# Generated Rust will be safe and zero-copy where possible
```

### 2. Numpy to Rust

```bash
# Transpile numpy array implementation
spydecy transpile-numpy numpy/core/src/multiarray/ \
    --output numpy-rust/ \
    --use-ndarray \
    --optimize simd

# Leverages Rust's ndarray + SIMD
```

### 3. Python + CPython Together

```bash
# Transpile Python code that uses CPython internals
spydecy transpile-unified myproject/ \
    --include-cpython-deps \
    --inline-c-calls \
    --output myproject-rust/

# Eliminates Python-C boundaries entirely
```

### 4. Debugging Depyler/Decy Issues

```bash
# Interactive debugging session
spydecy debug --target depyler \
    --source problematic_python.py \
    --visualize \
    --breakpoint-on unsafe

# Step through transpilation, identify issues
```

### 5. Self-Hosting Bootstrap

```bash
# Bootstrap the compiler
spydecy bootstrap \
    --stage0 target/release/spydecy \
    --source spydecy/spydecy.py \
    --validate

# Validates compiler correctness through self-transpilation
```

---

## Roadmap (20 Sprints, 40 Weeks)

### Phase 1: Foundation (Sprints 1-5, Weeks 1-10)

**Sprint 1: Project Setup**
- DECY-001: Setup monorepo structure
- DECY-002: Integrate depyler and decy as libraries
- DECY-003: Setup quality gates (PMAT)
- DECY-004: Create test infrastructure

**Sprint 2: Python Transpiler**
- DECY-005: Python AST parser (PyO3 integration)
- DECY-006: Type hint extraction
- DECY-007: Gradual type checking
- DECY-008: Python HIR generation

**Sprint 3: C Transpiler**
- DECY-009: C parser (clang-sys integration)
- DECY-010: CPython API identification
- DECY-011: PyObject tracking
- DECY-012: C HIR generation

**Sprint 4: Unified HIR**
- DECY-013: Unified type system
- DECY-014: Cross-language references
- DECY-015: Metadata tracking
- DECY-016: HIR validation

**Sprint 5: Basic Codegen**
- DECY-017: Python → Rust codegen
- DECY-018: C → Rust codegen
- DECY-019: Rustfmt integration
- DECY-020: Clippy compliance

### Phase 2: Optimization (Sprints 6-10, Weeks 11-20)

**Sprint 6: Cross-Layer Optimizer**
- DECY-021: Boundary elimination
- DECY-022: Cross-language inlining
- DECY-023: Ownership unification
- DECY-024: Refcount optimization

**Sprint 7: CPython Optimization**
- DECY-025: PyObject* → safe Rust
- DECY-026: Py_INCREF/DECREF → Arc
- DECY-027: GIL elimination where safe
- DECY-028: CPython API inlining

**Sprint 8: Memory Optimization**
- DECY-029: Escape analysis
- DECY-030: Stack vs heap allocation
- DECY-031: Copy-on-write (COW)
- DECY-032: Zero-copy string handling

**Sprint 9: Performance Validation**
- DECY-033: Benchmark suite
- DECY-034: CPython comparison
- DECY-035: Numpy comparison
- DECY-036: Performance regression tests

**Sprint 10: Safety Verification**
- DECY-037: Unsafe block minimization
- DECY-038: Borrow checker validation
- DECY-039: Memory safety proofs
- DECY-040: Security audit

### Phase 3: Debugger (Sprints 11-15, Weeks 21-30)

**Sprint 11: Debugger Foundation**
- DECY-041: Transpilation state tracking
- DECY-042: Breakpoint system
- DECY-043: Step-through execution
- DECY-044: Visualization basics

**Sprint 12: Advanced Visualization**
- DECY-045: HIR visualization
- DECY-046: Ownership graph display
- DECY-047: Transformation history
- DECY-048: Diff visualization

**Sprint 13: Issue Detection**
- DECY-049: Refcount issue detection
- DECY-050: GIL issue detection
- DECY-051: Lifetime issue detection
- DECY-052: Unsafe pattern detection

**Sprint 14: Recommendations**
- DECY-053: Auto-fix suggestions
- DECY-054: Pattern library
- DECY-055: Best practices guidance
- DECY-056: Code improvement hints

**Sprint 15: Integration**
- DECY-057: Depyler integration
- DECY-058: Decy integration
- DECY-059: Interactive REPL
- DECY-060: MCP server mode

### Phase 4: Self-Hosting (Sprints 16-20, Weeks 31-40)

**Sprint 16: Bootstrap Foundation**
- DECY-061: Stage 0 compiler (Rust)
- DECY-062: Spydecy.py implementation
- DECY-063: Bootstrap build system
- DECY-064: Validation framework

**Sprint 17: Stage 1 Compiler**
- DECY-065: Stage 0 → Stage 1 transpilation
- DECY-066: Stage 1 testing
- DECY-067: Stage 1 validation
- DECY-068: Issue tracking

**Sprint 18: Stage 2 Compiler**
- DECY-069: Stage 1 → Stage 2 transpilation
- DECY-070: Output comparison
- DECY-071: Fixed point verification
- DECY-072: Correctness proofs

**Sprint 19: Production Hardening**
- DECY-073: CPython full transpilation
- DECY-074: Numpy full transpilation
- DECY-075: Pandas transpilation
- DECY-076: Real-world testing

**Sprint 20: Release Preparation**
- DECY-077: Documentation
- DECY-078: Examples & tutorials
- DECY-079: MCP integration finalization
- DECY-080: v1.0 release

---

## Success Metrics

### Technical Metrics

| Metric | Target | Measured By |
|--------|--------|-------------|
| Test Coverage | ≥80% | cargo llvm-cov |
| Mutation Score | ≥90% | cargo-mutants |
| Transpilation Success | ≥95% | Corpus evaluation |
| Performance vs CPython | Within 20% | Benchmarks |
| Performance vs Numpy | Within 10% | Benchmarks |
| Unsafe Code Density | <5 per 1000 LOC | Static analysis |
| Self-Hosting Success | 100% | Bootstrap validation |

### Quality Metrics

| Metric | Target | Enforcement |
|--------|--------|-------------|
| Complexity (CCN) | ≤10 | PMAT pre-commit |
| SATD Comments | 0 | PMAT pre-commit |
| Clippy Warnings | 0 | CI/CD |
| Property Tests | ≥100 | Test suite |
| Differential Tests | ≥1000 | CPython/Numpy corpus |

---

## Unique Value Propositions

1. **Unified Python/C Transpilation**: First tool to handle both Python source and CPython C implementation together

2. **Introspective Debugging**: Step through transpilation process, visualize transformations, identify issues in real-time

3. **Self-Hosting Validation**: Compiler transpiles itself, providing ultimate correctness guarantee

4. **Zero-Copy Optimization**: Eliminates Python-C boundaries through cross-layer analysis

5. **CPython/Numpy Ready**: Specifically designed to handle the complexity of CPython and numerical libraries

6. **Depyler/Decy Enhancement**: Provides debugging capabilities for these existing transpilers

7. **Production-Grade Quality**: EXTREME TDD + 80% coverage + 90% mutation score from day one

---

## References & Inspiration

- **Depyler**: Python-to-Rust transpiler
- **Decy**: C-to-Rust transpiler
- **Bashrs**: Bash-to-Rust with extreme quality
- **Ruchy**: Self-hosting scripting language
- **CPython**: Python reference implementation
- **PyPy**: Self-hosting Python interpreter
- **Rust**: Memory-safe systems programming
- **PMAT**: Quality enforcement framework

---

## Conclusion

**Spydecy** represents a novel approach to language transpilation by recognizing that Python (CPython) is implemented in C, and leveraging this to create a unified transpilation and debugging system. By combining:

1. Python transpilation (via depyler)
2. C transpilation (via decy)
3. Cross-layer optimization
4. Introspective debugging
5. Self-hosting validation

Spydecy provides a powerful tool for:
- Converting entire Python ecosystems to safe Rust
- Debugging tricky transpilation issues in depyler and decy
- Transpiling CPython itself and numeric libraries
- Validating transpiler correctness through self-hosting

The EXTREME TDD methodology with 80%+ coverage, 90%+ mutation score, and zero technical debt ensures production-grade quality from day one.

---

**Status**: SPECIFICATION COMPLETE - Ready for Implementation
**Next Step**: Begin Sprint 1 - Project Setup

