# Sprint 3: C Transpiler Foundation - IN PROGRESS 🚧

**Date Started**: 2025-10-22
**Status**: Just Started
**Previous Sprint**: Sprint 2 Complete ✅ (36/36 tests)

---

## 🎯 Sprint 3 Goals

Sprint 3 focuses on building the C transpiler foundation to enable **first unification tests**:

1. ⏳ C Parser using clang-sys (following decy's approach)
2. ⏳ CPython API pattern recognition (Py_SIZE, PyList_*, etc.)
3. ⏳ C → HIR converter
4. ⏳ C-API trait system (pluggable architecture)
5. ⏳ Second debugger feature: `spydecy debug --visualize c_file.c`
6. ⏳ **FIRST UNIFICATION TESTS**: Python + C → Unified HIR → Rust

---

## ✅ Completed

### Research Phase
- ✅ Analyzed decy's C parser implementation
- ✅ Confirmed clang-sys approach (version 1.7, clang_3_9 feature)
- ✅ Verified libclang-14-dev installed on system
- ✅ Updated spydecy-c Cargo.toml with dependencies

**Key Findings from decy**:
- Uses `clang-sys` 1.7 with FFI to libclang
- Creates `CXIndex` for parsing context
- Uses `clang_parseTranslationUnit2` to parse source
- Walks AST using clang cursor visitors
- Requires `unsafe_code = "allow"` for FFI bindings

### C Parser Implementation (spydecy-c/) ✅
- ✅ **C parser core working** (parser.rs - 340 lines)
- ✅ **CPython API recognition** (cpython.rs - 60 lines)
- ✅ **C → HIR converter** (hir_converter.rs - 160 lines)
- ✅ **11/11 tests passing** ✅
- ✅ **Workspace tests: 47/47** (grew from 36 → 47)

**Key Features Implemented**:
- clang-sys FFI integration with LLVM/Clang
- C AST parsing (functions, calls, returns, variables)
- CPython API pattern detection (Py_SIZE, PyList_*, etc.)
- C → Spydecy HIR conversion
- Type parsing (int, void, Py_ssize_t, PyListObject*, etc.)
- CPython macro identification

---

## ⏳ In Progress

### Next: Second Debugger Feature
- ⏳ Implement `spydecy debug --visualize c_file.c`
- ⏳ Reuse visualization framework from Sprint 2

---

## 📋 Remaining Tasks

1. Implement C parser core
2. Add CPython API pattern detection
3. Build C → HIR converter
4. Create C-API trait system
5. Implement `visualize c-ast` debugger command
6. Write first Python + C unification tests
7. Validate: Sprint 0's `len()` pattern end-to-end

---

## 🔗 Sprint Dependencies

**Sprint 2 Outputs (Available)**:
- ✅ Production Unified HIR
- ✅ Python parser working
- ✅ Python → HIR converter
- ✅ Debugger visualization framework

**Sprint 3 Will Enable**:
- First real unification: Python `len(x)` + C `list_length()` → Rust `x.len()`
- CPython API recognition
- Cross-language pattern matching
- Complete transpilation pipeline validation

---

**Last Updated**: 2025-10-22
**Status**: Just Started 🚀
**Next Milestone**: Basic C parser working
