# Sprint 5: Real-World CPython Validation

**Date Started**: 2025-10-23
**Status**: 🚧 IN PROGRESS
**Goal**: Validate core innovation on actual CPython stdlib code

---

## 🎯 Sprint 5 Objectives

**Primary Goal**: Port 2-3 real CPython functions from `Objects/listobject.c`

**Success Criteria**:
1. ✅ Functions compile to safe Rust (zero unsafe)
2. ✅ Performance within 20% of hand-written Rust
3. ✅ Generated code is idiomatic Rust
4. ✅ All tests pass with new patterns

---

## 📋 Target Functions

### Already Working (Validated)
- ✅ `list_length()` → `Vec::len()` (Sprint 3)
- ✅ `PyList_Append()` → `Vec::push()` (Sprint 3)
- ✅ `PyDict_GetItem()` → `HashMap::get()` (Sprint 3)

### Sprint 5 Targets

**1. list_reverse() - PRIORITY 1**
```c
static PyObject *
list_reverse_impl(PyListObject *self)
{
    Py_ssize_t n = Py_SIZE(self);
    PyObject **items = self->ob_item;
    PyObject *tmp;

    for (Py_ssize_t i = 0; i < n / 2; i++) {
        tmp = items[i];
        items[i] = items[n - i - 1];
        items[n - i - 1] = tmp;
    }
    Py_RETURN_NONE;
}
```

**Target Rust**:
```rust
fn reverse<T>(v: &mut Vec<T>) {
    let n = v.len();
    for i in 0..n/2 {
        v.swap(i, n - i - 1);
    }
}
```

**Challenges**:
- Loop with arithmetic
- Array indexing with pointer arithmetic
- Mutable operations

**2. list_clear() - PRIORITY 2**
```c
static int
list_clear(PyListObject *a)
{
    Py_ssize_t i;
    PyObject **item = a->ob_item;
    if (item != NULL) {
        i = Py_SIZE(a);
        Py_SET_SIZE(a, 0);
        while (--i >= 0) {
            Py_XDECREF(item[i]);
        }
        PyMem_FREE(item);
    }
    return 0;
}
```

**Target Rust**:
```rust
fn clear<T>(v: &mut Vec<T>) {
    v.clear();
}
```

**Challenges**:
- Reference counting (must be eliminated)
- Memory management (PyMem_FREE)
- Mutable list modification

---

## 🏗️ Implementation Strategy

### Phase 1: Pattern Recognition (2-3 hours)
1. Identify new patterns in target functions
2. Extend Unifier with new pattern matchers
3. Add tests for each pattern

### Phase 2: Real-World Test Suite (1-2 hours)
1. Create `tests/real_world/` directory
2. Port actual CPython test cases
3. Add performance benchmarks

### Phase 3: Implementation (3-4 hours)
1. Implement `list_reverse()` pattern
2. Implement `list_clear()` pattern
3. Use debugger to diagnose issues

### Phase 4: Validation (1 hour)
1. Run benchmarks
2. Compare generated code quality
3. Document findings

---

## 🔬 Test Structure

```
tests/real_world/
├── cpython/
│   ├── list_reverse.c      # Actual CPython code
│   └── list_clear.c
├── python/
│   ├── test_reverse.py     # Python using list.reverse()
│   └── test_clear.py
└── benchmarks/
    ├── bench_reverse.rs    # Criterion benchmarks
    └── bench_clear.rs
```

---

## 📊 Performance Targets

From SPECIFICATION.md Section 30:

**Compile Time**: < 10s for 1000 LOC Python
**Memory**: < 100MB peak for 1000 LOC
**Generated Code**: Within 20% of hand-written Rust

**Sprint 5 Targets**:
- `list_reverse()`: Within 10% of `Vec::reverse()`
- `list_clear()`: Identical to `Vec::clear()`
- Zero unsafe code
- Zero FFI calls

---

## 🐛 Debugger Usage

**Expected Issues**:
1. Loop pattern recognition
2. Pointer arithmetic → slice indexing
3. Reference counting elimination

**Debug Strategy**:
```bash
spydecy debug step --python test_reverse.py --c list_reverse.c

(spydecy-debug) break boundary
(spydecy-debug) continue
# Examine unification
(spydecy-debug) inspect unified
# Check optimization
(spydecy-debug) step
(spydecy-debug) inspect rust
```

---

## 📈 Success Metrics

**Minimum Viable**:
- ✅ 1 new function ported successfully
- ✅ Generated code compiles
- ✅ Tests pass

**Target**:
- ✅ 2-3 functions ported
- ✅ Performance within 20%
- ✅ Idiomatic Rust output

**Stretch**:
- ✅ 5+ functions ported
- ✅ Performance within 10%
- ✅ Pattern library documented

---

## 🚀 Next Steps

1. **NOW**: Implement `list_reverse()` pattern
2. Add new unification pattern to `spydecy-hir`
3. Create real-world test suite
4. Benchmark and validate

---

**Last Updated**: 2025-10-23
**Status**: 📝 PLANNING COMPLETE → STARTING IMPLEMENTATION
