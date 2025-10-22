# Sprint 0: Tracer Bullet - Unified HIR Validation
## 2-Week Risk Mitigation Sprint

**Status**: CRITICAL - Must Complete Before Main Project
**Duration**: 2 weeks
**Team**: 1-2 senior engineers
**Outcome**: Go/No-Go decision on Unified HIR architecture

---

## Executive Summary

**Problem**: The Unified HIR is the foundation of the entire Spydecy architecture. If the Python HIR and C HIR cannot be successfully unified, the core value proposition (cross-layer optimization) fails.

**Risk**: This is a **make-or-break assumption** that the current roadmap doesn't validate until Sprint 4 (week 8), risking 8 weeks of wasted effort.

**Solution**: Build a minimal end-to-end "tracer bullet" that proves the Unified HIR concept works before committing to the full 40-week roadmap.

> **Reference**: Hunt, A., & Thomas, D. (2000). *The Pragmatic Programmer: From Journeyman to Master*. Addison-Wesley.
> "Tracer bullets work because they operate in the same environment and under the same constraints as the final system."

---

## Tracer Bullet Objective

Build the **smallest possible** end-to-end system that:
1. Transpiles a trivial Python function to Rust (via depyler)
2. Transpiles the CPython C implementation of that function to Rust (via decy)
3. Unifies both HIRs into a single representation
4. Performs cross-layer optimization (eliminates the Python→C boundary)
5. Generates working Rust code that can be compiled and tested

---

## Micro-Target: `len()` Function

### Python Code
```python
# test_len.py
def my_len(x):
    """Return the length of x using built-in len()"""
    return len(x)
```

### CPython C Implementation
```c
// Objects/listobject.c (simplified)
static Py_ssize_t
list_length(PyListObject *self)
{
    return Py_SIZE(self);
}

PyObject *
PyList_Size(PyObject *op)
{
    if (!PyList_Check(op)) {
        PyErr_BadInternalCall();
        return -1;
    }
    return Py_SIZE(op);
}
```

### Expected Output (After Optimization)
```rust
// Optimized: Python→C boundary eliminated entirely
pub fn my_len<T>(x: &Vec<T>) -> usize {
    x.len()  // Direct Rust, no FFI
}
```

---

## Week-by-Week Plan

### Week 1: Manual Transpilation & HIR Design

#### Day 1-2: Manual Transpilation
```bash
# Step 1: Transpile Python
echo "def my_len(x): return len(x)" | depyler transpile --output python_out.rs

# Step 2: Transpile C
decy transpile Objects/listobject.c --function list_length --output c_out.rs

# Step 3: Analyze the outputs
diff python_out.rs c_out.rs
```

**Deliverable**: Document showing:
- Depyler's Rust output (high-level, safe)
- Decy's Rust output (low-level, possibly unsafe)
- Semantic gaps between the two

#### Day 3-4: Minimal HIR Design

Design the **absolute minimum** HIR structures needed:

```rust
// spydecy-hir-minimal/src/lib.rs

/// Minimal HIR to test unification concept
pub enum MiniHIR {
    // Python-level constructs
    PythonFunction {
        name: String,
        body: Vec<MiniHIR>,
    },
    PythonCall {
        callee: String,  // e.g., "len"
        args: Vec<MiniHIR>,
    },
    PythonVar(String),

    // C-level constructs
    CFunction {
        name: String,
        body: Vec<MiniHIR>,
    },
    CFieldAccess {
        object: Box<MiniHIR>,
        field: String,  // e.g., "Py_SIZE"
    },
    CPointer(Box<MiniHIR>),

    // Unified constructs
    UnifiedCall {
        target_language: Language,
        callee: String,
        args: Vec<MiniHIR>,
    },
}

pub enum Language {
    Python,
    C,
    Rust,  // After optimization
}
```

**Deliverable**: Minimal HIR that can represent both the Python `len()` call and the C `Py_SIZE` macro.

#### Day 5: Bridging the Gap

Write the unification logic:

```rust
impl MiniHIR {
    pub fn unify_python_and_c(
        python_hir: MiniHIR,
        c_hir: MiniHIR,
        call_graph: &CallGraph,
    ) -> Result<MiniHIR> {
        match (python_hir, c_hir) {
            // Key case: Python len() → C list_length()
            (
                PythonCall { callee: "len", args },
                CFunction { name: "list_length", .. }
            ) => {
                // This is the "magic" - recognize the relationship
                Ok(UnifiedCall {
                    target_language: Language::Rust,
                    callee: "vec_len".to_string(),  // Map to Rust
                    args,
                })
            }
            // ... other cases
        }
    }
}
```

**Deliverable**: Proof-of-concept unification function that can merge the Python and C HIRs.

### Week 2: Optimization & Validation

#### Day 6-7: Boundary Elimination

Implement the optimizer:

```rust
// spydecy-optimizer-minimal/src/lib.rs

pub struct BoundaryEliminator;

impl BoundaryEliminator {
    pub fn eliminate(&self, hir: &MiniHIR) -> Result<MiniHIR> {
        match hir {
            UnifiedCall {
                target_language: Language::Python,
                callee: "len",
                args,
            } => {
                // Eliminate Python→C boundary
                // Replace with direct Rust Vec::len()
                Ok(UnifiedCall {
                    target_language: Language::Rust,
                    callee: "Vec::len".to_string(),
                    args: args.clone(),
                })
            }
            // ... other cases
        }
    }
}
```

**Deliverable**: Optimizer that converts `UnifiedCall` nodes from Python/C to pure Rust.

#### Day 8-9: Code Generation

Generate the final Rust:

```rust
// spydecy-codegen-minimal/src/lib.rs

impl MiniHIR {
    pub fn codegen(&self) -> TokenStream {
        match self {
            PythonFunction { name, body } => quote! {
                pub fn #name<T>(x: &Vec<T>) -> usize {
                    #(#body)*
                }
            },
            UnifiedCall { target_language: Language::Rust, callee, args } => {
                match callee.as_str() {
                    "Vec::len" => quote! { x.len() },
                    _ => panic!("Unknown Rust function: {}", callee),
                }
            }
            // ... other cases
        }
    }
}
```

**Deliverable**: Working code generator that produces compilable Rust.

#### Day 10: End-to-End Test

```rust
#[test]
fn test_tracer_bullet_e2e() {
    // 1. Parse Python
    let python_code = "def my_len(x): return len(x)";
    let python_hir = parse_python_to_mini_hir(python_code).unwrap();

    // 2. Parse C
    let c_code = include_str!("list_length.c");
    let c_hir = parse_c_to_mini_hir(c_code).unwrap();

    // 3. Unify
    let unified = MiniHIR::unify_python_and_c(
        python_hir,
        c_hir,
        &build_call_graph(),
    ).unwrap();

    // 4. Optimize
    let optimized = BoundaryEliminator.eliminate(&unified).unwrap();

    // 5. Generate Rust
    let rust_code = optimized.codegen();

    // 6. Compile and test
    let compiled = compile_rust(&rust_code).unwrap();
    let result = compiled.my_len(&vec![1, 2, 3]);
    assert_eq!(result, 3);

    // 7. Verify no FFI boundary
    assert!(!rust_code.to_string().contains("extern"));
    assert!(!rust_code.to_string().contains("PyObject"));
}
```

**Deliverable**: Passing end-to-end test that proves the concept works.

---

## Success Criteria

### Must Have (Go Decision)
1. ✅ **Unification Works**: Python HIR + C HIR → Unified HIR (manually verified)
2. ✅ **Optimization Works**: Cross-layer optimizer eliminates Python→C boundary
3. ✅ **Code Compiles**: Generated Rust compiles without errors
4. ✅ **Behavior Correct**: Output matches Python `len()` behavior
5. ✅ **No FFI**: Generated Rust contains no `extern` or FFI calls

### Nice to Have (Not Required for Go)
- Performance benchmark (Rust vs Python)
- Property-based tests
- Multiple test cases

### No-Go Criteria
- ❌ **Cannot Unify HIRs**: If the semantic gap is too large to bridge
- ❌ **Cannot Eliminate Boundary**: If optimization fails to inline the C call
- ❌ **Generated Code Unsafe**: If output contains `unsafe` blocks
- ❌ **Behavior Mismatch**: If Rust output differs from Python

---

## Risk Assessment

### If Tracer Bullet Succeeds
- **Confidence**: High that Unified HIR architecture is viable
- **Action**: Proceed with main roadmap (Sprints 1-20)
- **Timeline**: 40 weeks as planned

### If Tracer Bullet Fails
- **Confidence**: Unified HIR may not be feasible
- **Action**: Pivot to alternative architecture
- **Alternative 1**: Separate Python and C transpilers (no cross-layer optimization)
- **Alternative 2**: Focus on depyler/decy integration without unification
- **Alternative 3**: Research phase to solve HIR impedance mismatch

---

## Deliverables

### Code
```
spydecy-tracer-bullet/
├── Cargo.toml
├── src/
│   ├── mini_hir.rs           # Minimal HIR definition
│   ├── python_parser.rs      # Python → Mini HIR
│   ├── c_parser.rs            # C → Mini HIR
│   ├── unifier.rs             # Unification logic
│   ├── optimizer.rs           # Boundary eliminator
│   └── codegen.rs             # Rust code generator
├── tests/
│   └── e2e_test.rs            # End-to-end test
└── examples/
    ├── test_len.py            # Test Python code
    └── list_length.c          # Test C code
```

### Documentation
1. **Design Document**: `docs/tracer-bullet-design.md`
   - HIR design rationale
   - Unification algorithm
   - Optimization strategy

2. **Results Report**: `docs/tracer-bullet-results.md`
   - Test results
   - Performance measurements
   - Go/No-Go recommendation

3. **Lessons Learned**: `docs/tracer-bullet-lessons.md`
   - What worked
   - What didn't
   - Implications for full project

---

## Timeline

```
Week 1:
  Mon-Tue: Manual transpilation & gap analysis
  Wed-Thu: Minimal HIR design
  Fri:     Unification proof-of-concept

Week 2:
  Mon-Tue: Boundary elimination optimizer
  Wed-Thu: Code generation
  Fri:     E2E test & Go/No-Go decision
```

---

## Budget

- **Time**: 2 weeks (10 working days)
- **Team**: 1-2 senior engineers
- **Cost**: ~$10,000 (at $100/hour × 100 hours)
- **ROI**: Validates/invalidates $400,000+ main project

**Cost of Failure**: If we skip this and the main project fails at Sprint 4, we waste 8 weeks (~$80,000).

**Value**: This is **insurance**. For 2.5% of the total project cost, we validate the riskiest assumption.

---

## Decision Point: End of Week 2

**Go Decision**:
- All 5 "Must Have" criteria met
- Team confident in HIR design
- **Action**: Begin Sprint 1 of main roadmap

**No-Go Decision**:
- Any "No-Go" criteria met
- HIR unification failed
- **Action**: Architectural pivot or project cancellation

---

## References

1. Hunt, A., & Thomas, D. (2000). *The Pragmatic Programmer*. Addison-Wesley.
2. Beck, K. (2002). *Test Driven Development: By Example*. Addison-Wesley.
3. Toyota Production System: Jidoka (stop and fix problems)
4. Boehm, B. (1988). "A Spiral Model of Software Development and Enhancement"

---

**Status**: SPECIFICATION COMPLETE
**Next Step**: Secure team and begin Week 1
**Risk Level**: CRITICAL - Project success depends on this
