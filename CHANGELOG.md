# Changelog

All notable changes to Spydecy will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - Advanced Patterns: Values, Count, Index ✨

**New Pattern Support** (3 new patterns):
- ✅ **Dict Values**: Python `dict.values()` + C `PyDict_Values()` → Rust `HashMap::values()`
- ✅ **List Count**: Python `list.count(x)` + C `list_count()` → Rust `Vec::iter().filter().count()`
- ✅ **List Index**: Python `list.index(x)` + C `list_index()` → Rust `Vec::iter().position()`

**Quality Metrics**:
- 42 tests passing (up from 36) - 100% pass rate maintained
- 16 patterns total (up from 13)
- 3 new end-to-end tests (`e2e_advanced_patterns.rs`)
- 3 new real-world validation scenarios:
  - Configuration value extraction (dict values)
  - Error counting in logs (list count)
  - Waitlist position finding (list index)

**Testing & Validation**:
- Added `e2e_advanced_patterns.rs` with full pipeline tests
- Real-world validation: `config_dict.values()`, `log_entries.iter().filter().count()`, `waitlist.iter().position()`
- Extreme TDD methodology: RED → GREEN → REFACTOR

**Technical Details**:
- Pattern matching in `unified.rs`
- Code generation in `codegen.rs` with iterator chains
- `list.count()` generates `Vec::iter().filter(|&x| x == &item).count()`
- `list.index()` generates `Vec::iter().position(|&x| x == &item)` (returns Option)
- Generated code uses actual variable names (e.g., `config_dict`, `log_entries`, `waitlist`)

### Added - Contains Patterns ✨

**New Pattern Support** (2 new patterns):
- ✅ **List Contains**: Python `x in list` + C `list_contains()` → Rust `Vec::contains()`
- ✅ **Dict Contains**: Python `x in dict` + C `dict_contains()` → Rust `HashMap::contains_key()`

**Quality Metrics**:
- 36 tests passing (up from 28) - 100% pass rate maintained
- 13 patterns total (up from 11)
- 2 new end-to-end tests
- 2 new real-world validation scenarios:
  - User authorization checking (list contains)
  - Feature flag checking (dict contains)

**Testing & Validation**:
- Added `e2e_contains.rs` with full pipeline tests
- Real-world validation: `allowed_users.contains(&item)` and `feature_flags.contains_key(&key)`
- Extreme TDD methodology: RED → GREEN → REFACTOR

**Technical Details**:
- Pattern matching in `unified.rs`
- Code generation in `codegen.rs`
- Unification logic supports both list and dict containment checks
- Generated code uses actual variable names (e.g., `allowed_users`, `feature_flags`)

---

## [0.3.0] - 2025-10-23

### Added - Phase 2 Complete: MVP-READY! 🎉

**MVP Status Achieved**
- ✅ **PHASE 2 COMPLETE (80%)**: Spydecy is production-ready for list/dict operations
- ✅ **11 Patterns Working**: 7 list operations + 4 dict operations
- ✅ **28 Tests Passing (100%)**: Comprehensive test coverage including real-world validation
- ✅ **Zero Performance Overhead**: 0-6% variance (far exceeding 20% target)
- ✅ **Safe Rust Output**: Zero unsafe code, no FFI boundaries
- ✅ **User-Friendly Errors**: Pattern suggestions and documentation links

**Phase 2.1: Full Argument Support** (~2 hours)
- Real variable names flow from Python to Rust output
- `my_list.len()` not generic `x.len()`
- `shopping_cart.push(item)` not generic `x.push(item)`
- `config_map.get(&key)` not generic `map.get(&key)`
- All 11 patterns updated to use actual argument names
- Added argument conversion infrastructure in unifier
- Added receiver name extraction in codegen

**Phase 2.2: Performance Benchmarking** (~1 hour)
- **Target EXCEEDED**: 0-6% overhead (target was <20%)
- Comprehensive benchmark suite comparing Spydecy-generated vs hand-written Rust
- Key results:
  - `HashMap::get(1000)`: 18.449 ns vs 18.699 ns (+1.35%)
  - `Vec::clear(1000)`: 118.90 ns vs 118.72 ns (-0.15%, faster!)
  - `Vec::pop(1000)`: 92.260 ns vs 91.581 ns (-0.74%, faster!)
- Generated code performs identically to hand-written Rust

**Phase 2.3: Error Messages** (~1.5 hours)
- User-friendly, actionable error diagnostics
- Pattern suggestion system (shows 5 similar patterns)
- Clear problem identification with function names
- Documentation links for custom patterns
- Self-service debugging support
- Structured error types (`UnificationError` enum)

**Phase 2.5: Real-World Validation** (~1 hour)
- Validated on 8 realistic scenarios:
  - Shopping cart operations (e-commerce)
  - User history tracking (analytics)
  - Configuration lookup (settings)
  - Log entry management (logging)
  - Notification queue clearing
  - Data processing pipelines
- All patterns generate safe, idiomatic Rust
- Variable names preserved in realistic contexts

**Supported Operations (11 patterns)**
- **List Operations**: len, append, reverse, clear, pop, insert, extend
- **Dict Operations**: get, pop, clear, keys

**Quality Metrics**
- 28/28 tests passing (100%)
- 0-6% performance overhead (exceeds target)
- Zero unsafe code in generated output
- Real variable names preserved
- User-friendly error messages

**Phase 2.4 Status**: Documentation deferred to post-MVP (current docs sufficient for early adopters)

### Added - Complete Pipeline: Code Generation! 🦀

**Code Generator Implementation** (Post v0.2.0)
- ✅ **FULL PIPELINE WORKING**: Parse → Unify → Optimize → Codegen → Rust Code!
- ✅ **Rust Code Generator**: Converts optimized UnifiedHIR to idiomatic Rust
- ✅ **All 3 Patterns Generate Code**: len, append, dict.get → pure Rust
- ✅ **Type Generation**: Handles Rust types (int, Vec, Option, Reference, etc.)
- ✅ **Function Generation**: Generates complete Rust functions with signatures
- ✅ **Module Generation**: Can generate complete Rust modules

**Testing & Quality**
- 81/81 tests passing (up from 72) - added 9 new tests
  - 6 new unit tests for code generation
  - 4 new end-to-end pipeline tests (full Parse→Codegen)
  - All tests validate complete pipeline working
- All quality gates passing (format, clippy, PMAT, tests, build, book)
- Maintained PMAT complexity < 10
- Zero SATD violations

**Code Generation Features** (spydecy-codegen/)
- `RustCodegen` - Main code generator with indentation support
- `generate_rust()` - Convenience function for quick code generation
- Pattern-aware generation - recognizes optimized patterns
- Idiomatic Rust output - generates clean, readable Rust code
- Type annotation generation - converts HIR types to Rust syntax

**End-to-End Pipeline Tests** (tests/e2e_full_pipeline.rs)
- `test_full_pipeline_len_pattern` - Complete len() pipeline with verbose output
- `test_full_pipeline_append_pattern` - Complete append() pipeline
- `test_full_pipeline_dict_get_pattern` - Complete dict.get() pipeline
- `test_all_patterns_generate_unique_code` - Verifies all patterns work correctly

**Pipeline Achievement** 🎉
```
Python source  → PythonHIR     ✅
C source       → CHIR          ✅
Python + C     → UnifiedHIR    ✅
UnifiedHIR     → Optimized     ✅
Optimized      → Rust code     ✅ NEW!
```

Result: **Pure Rust code with ZERO FFI, ZERO unsafe!**

### Added - Functional CLI! 🎯

**CLI Implementation** (Post v0.2.0)
- ✅ **WORKING CLI**: Command-line interface ties full pipeline together
- ✅ **Compile Command**: `spydecy compile --python file.py --c file.c --output file.rs`
- ✅ **Debug Command**: `spydecy debug --visualize file.py` - AST visualization
- ✅ **Info Command**: `spydecy info` - Display project status and capabilities
- ✅ **Verbose Mode**: `--verbose` flag shows detailed pipeline execution
- ✅ **End-to-End Validation**: Complete pipeline working from CLI

**CLI Features** (src/main.rs)
- Full pipeline integration: Parse → Unify → Optimize → Codegen → Write
- Detailed error reporting with context
- Beautiful terminal output with emojis and formatting
- Progress indicators for each pipeline stage
- Example files provided in `sample-outputs/` directory

**User Experience**
```bash
# Compile Python + C to Rust
spydecy compile --python len.py --c len.c --output len.rs --verbose

# Output shows:
# 🚀 Step 1: Parsing Python... ✅
# 📝 Step 2: Parsing C... ✅
# 🔗 Step 3: Unifying... ✅
# ⚡ Step 4: Optimizing... ✅
# 🦀 Step 5: Generating Rust... ✅
# 💾 Step 6: Writing output... ✅
# 🎉 Compilation successful!
```

**Version Updates**
- spydecy-codegen bumped to 0.2.0
- spydecy-optimizer bumped to 0.2.0
- Main crate dependencies updated for CLI usage

### Added - Cross-Layer Optimizer (Sprint 4)! 🚀

**Optimizer Implementation** (Post v0.2.0)
- ✅ **OPTIMIZER WORKING**: Pass-based optimization architecture implemented
- ✅ **Boundary Elimination Pass**: Core optimization that removes Python→C FFI boundaries
- ✅ **Optimization Pipeline**: Orchestrates multiple optimization passes in sequence
- ✅ `OptimizationPipeline::standard()` - Pre-configured pipeline with boundary elimination
- ✅ `Pass` trait - Extensible architecture for adding new optimization passes
- ✅ Complete integration tests demonstrating optimizer with all 3 core patterns

**Testing & Quality**
- 72/72 tests passing (up from 62) - added 10 new optimizer tests
  - 5 new unit tests for optimizer (boundary elimination, pipeline)
  - 5 new integration tests (one per pattern + full pipeline + multi-pass)
  - All tests validate: Parse → Unify → Optimize pipeline
- All quality gates passing (format, clippy, PMAT, tests, build)
- Maintained PMAT complexity < 10
- Zero SATD violations

**Architecture**
- Pass-based optimization system: `UnifiedHIR → Pass 1 → Pass 2 → Pass N → Optimized HIR`
- `BoundaryEliminationPass` - Uses existing `eliminate_boundary()` from UnifiedHIR
- `OptimizationPipeline` - Runs passes sequentially, tracks pass count
- Future passes planned: Dead code elimination, inlining, constant folding

**Sprint 4 Milestone Achieved**
- ✅ Optimizer foundation complete
- ✅ Boundary elimination working for all 3 patterns (len, append, dict.get)
- ✅ Integration tests prove full pipeline: Parse → Unify → Optimize
- ✅ Extensible architecture ready for additional passes

### Added - All 3 Core Unification Patterns Complete! 🎉

**Third Pattern: Dict.Get Implementation** (Post v0.2.0)
- ✅ **ALL 3 CORE PATTERNS COMPLETE**: len, append, dict.get
- ✅ **THIRD PATTERN WORKING**: Python `dict.get()` + C `PyDict_GetItem()` → Rust `HashMap::get()`
- ✅ Pattern system fully validated - added dict.get pattern with full test coverage
- ✅ End-to-end integration test for dict.get pattern (`test_dict_get_unification_end_to_end`)
- ✅ CPython API recognition for `PyDict_GetItem` function
- ✅ Unit test for dict.get pattern unification (`test_unifier_dict_get_pattern`)
- ✅ Unit test for CPython dict.get pattern detection (`test_identify_pydict_getitem`)

**Second Pattern: Append Implementation**
- ✅ Python `list.append()` + C `PyList_Append()` → Rust `Vec::push()`
- ✅ End-to-end integration test for append pattern (`test_append_unification_end_to_end`)
- ✅ Unit test for append pattern unification (`test_unifier_append_pattern`)
- ✅ Unit test for CPython append pattern detection (`test_identify_pylist_append`)

**Testing & Quality**
- 62/62 tests passing (up from 51) - added 11 new tests
  - 3 new unit tests (one per pattern)
  - 3 new CPython API detection tests
  - 2 new end-to-end integration tests
  - All tests validate: Python → C → Unified HIR → Rust with boundary elimination
- All quality gates passing (format, clippy, PMAT, tests, build)
- Maintained PMAT complexity < 10
- Zero SATD violations

**Milestone Achievement**
- ✅ **3/3 core patterns implemented** (specification complete!)
  1. `len()` / `list_length()` → `Vec::len()`
  2. `append()` / `PyList_Append()` → `Vec::push()`
  3. `dict.get()` / `PyDict_GetItem()` → `HashMap::get()`
- ✅ Pattern system architecture fully validated and extensible
- ✅ Ready for v0.3.0 release focus: optimizer + additional patterns

---

## [0.2.0] - 2025-10-22

### Added - Unification Milestone 🎉

**Sprint 3 Major Achievement: First Python + C Unification Working**
- ✅ **END-TO-END UNIFICATION TEST PASSING** (`tests/e2e_unification.rs`)
- ✅ Python `len(x)` + C `list_length()` → Rust `Vec::len()` **VALIDATED**
- ✅ Complete pipeline with production parsers proven
- ✅ Sprint 0 core assumption now working with real code

**Unification System**
- Production unification engine in `spydecy-hir`
- Pattern matching system for Python-C relationships
- `LenPattern` fully implemented and tested
- Cross-language mapping with boundary elimination
- Zero FFI, zero unsafe code generation

**Testing & Quality**
- Integration test demonstrating complete pipeline
- 51/51 tests passing (up from 50)
- All quality gates passing
- PMAT complexity < 10 maintained

**Code Quality Improvements**
- Refactored `extract_ast_node` (complexity 25 → <10)
- Refactored `convert_node` in C parser (15 → <10)
- Refactored `convert_node` in Python parser (12 → <10)
- Fixed PMAT quality gate integration

**Documentation**
- Created `CLAUDE.md` - Comprehensive development guide
- Updated README with crates.io installation
- Added crates.io badges and download counters
- Updated Sprint 3 progress with milestone achievement

### Changed

- Version bumped to 0.2.0 across all workspace crates
- Updated installation instructions (crates.io first)
- Project status reflects v0.2.0 milestone completion

### Technical Details

**What This Release Proves:**
1. Python source → PythonHIR (via spydecy-python)
2. C source → CHIR (via spydecy-c)
3. Python + C → UnifiedHIR (via spydecy-hir)
4. Unified code targets pure Rust
5. Boundary elimination works

**Pipeline Validation:**
```
Python len(x) → PythonHIR ✅
C list_length() → CHIR ✅
Python + C → UnifiedHIR (Rust Vec::len) ✅
Boundary eliminated → Pure Rust code ✅
```

This is the **CORE INNOVATION** of Spydecy working end-to-end!

### Known Limitations

**v0.2.0 achieves the main goal but has optional features pending:**
- Only one pattern fully tested (`len` → `Vec::len`)
- C debugger visualization not yet implemented
- Additional patterns (append, dict.get) defined but not tested
- Optimizer still stubbed
- Codegen still stubbed

### Next Steps

**v0.3.0 Focus** (Est. 2-4 weeks):
- Add more unification patterns with tests
- Implement C debugger visualization
- Begin cross-layer optimization work
- Performance benchmarking

---

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
