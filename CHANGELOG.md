# Changelog

All notable changes to Spydecy will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-10-22

### Added - Initial Release

**Core Infrastructure**
- Production Unified HIR system for cross-layer optimization
- Complete type system bridging Python, C, and Rust
- Metadata tracking for debugging and optimization
- Node-based HIR with cross-references

**Python Support (Sprint 2 Complete)**
- Python AST parsing via PyO3
- Python → Spydecy HIR conversion
- Type hint extraction framework
- Full Python node support (functions, classes, calls, operators)

**C Support (Sprint 3 Complete)**
- C AST parsing via clang-sys (LLVM/Clang integration)
- CPython API pattern recognition (Py_SIZE, PyList_*, etc.)
- C → Spydecy HIR conversion
- Support for CPython implementation code

**Interactive Debugger**
- `spydecy debug --visualize <file.py>` - Python AST visualization
- Colored terminal output with tree visualization
- Source code display with line numbers
- AST statistics and metrics

**Quality & Testing**
- 47/47 tests passing (100% pass rate)
- EXTREME TDD methodology
- Zero-tolerance SATD enforcement
- Comprehensive Makefile with 50+ targets
- Pre-commit hooks for quality gates

**Architecture Validation**
- Sprint 0 tracer bullet validated core concept
- Production HIR scales successfully (12.5x code growth)
- Unification patterns working (len, append, dict.get)

### Crates

- `spydecy` - Main CLI binary
- `spydecy-hir` - Unified HIR (1,887 lines, 17 tests)
- `spydecy-python` - Python parser (412 lines, 7 tests)
- `spydecy-c` - C parser (560 lines, 11 tests)
- `spydecy-debugger` - Interactive debugger (250+ lines, 4 tests)
- `spydecy-optimizer` - Optimizer (stub)
- `spydecy-codegen` - Code generator (stub)
- `spydecy-analyzers` - Pluggable analyzers (stub)
- `spydecy-bootstrap` - Self-hosting (stub)

### Dependencies

**Runtime**
- PyO3 0.22 - Python integration
- clang-sys 1.7 - C parsing via LLVM
- libclang-14-dev (system requirement)
- Python 3.10-dev (system requirement)

**Development**
- PMAT - Quality analysis
- cargo-mutants - Mutation testing
- proptest - Property-based testing
- criterion - Benchmarking

### Documentation

- Comprehensive README with quick start
- Sprint 0 tracer bullet validation report
- Sprint 2 completion report
- Sprint 3 progress report
- Full specification documents
- Incremental debugger roadmap
- Pluggable C-API architecture spec

### Known Limitations

**v0.1.0 is a foundation release:**
- Unification tests not yet complete (Sprint 3 in progress)
- Optimizer is stubbed (Sprint 4+)
- Codegen is stubbed (Sprint 5+)
- C debugger visualization not yet implemented
- Type inference not yet implemented
- Self-hosting not yet implemented

**For decy/deypler integration:**
- Use `spydecy-hir` crate for HIR types
- Use `spydecy-python` for Python parsing
- Use `spydecy-c` for C parsing
- Unification APIs available but not fully tested yet

### Breaking Changes

N/A - Initial release

---

## [Unreleased]

### Planned for v0.2.0 (Sprint 3 completion)
- First unification tests (Python + C → Unified HIR)
- C AST debugger visualization
- Complete unification pattern library

### Planned for v0.3.0 (Sprint 4)
- Cross-layer optimizer
- Boundary elimination working end-to-end
- Interactive step-through debugger

### Planned for v1.0.0
- Full transpilation pipeline working
- Self-hosting capability
- Production-ready for CPython/NumPy conversion
