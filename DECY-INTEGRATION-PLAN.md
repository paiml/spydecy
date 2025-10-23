# Decy-Spydecy Integration Plan

## Current State Analysis

### Architecture Comparison

| Feature | **Spydecy** (Python+C→Rust) | **Decy** (C→Rust) |
|---------|---------------------------|-------------------|
| **Version** | 0.2.0 | 0.2.0 |
| **C Parser** | **decy-parser** ✅ (via adapter) | clang-sys (comprehensive) |
| **HIR** | Unified HIR (Python+C+Rust) | Decy HIR (C-oriented, Rust-target) |
| **Core Innovation** | Cross-language unification | Ownership/lifetime inference |
| **Unsafe Minimization** | Not primary focus | **Critical goal** (<5/1000 LOC) |
| **Pipeline** | Parse→Unify→Optimize→Codegen | Parse→HIR→Analyze→Ownership→Verify→Codegen |
| **Testing** | 99/99 tests (100%) ✅ | EXTREME TDD + mutation testing |
| **Debugger** | spydecy-debugger (Python+C AST) ✅ | **USES spydecy-debugger** + C extensions |
| **Quality** | PMAT + clippy + rustfmt | PMAT + Toyota Way + roadmap-driven |

### Existing Integration

✅ **Already Integrated**: decy uses `spydecy-debugger = "0.1.0"` for C AST visualization!

```toml
# decy/Cargo.toml
[workspace.dependencies]
spydecy-debugger = "0.1.0"
```

**Files using spydecy-debugger:**
- `decy/crates/decy-debugger/src/lib.rs` - Core integration
- `decy/crates/decy-debugger/src/visualize_ast.rs` - AST visualization

## Integration Opportunities

### 1. 🔄 Unified C Parser (High Priority)

**Problem**: Both projects parse C using clang-sys, but with different approaches:
- **Spydecy**: Simple C parser in `spydecy-c` (560 lines, CPython API patterns only)
- **Decy**: Comprehensive C parser in `decy-parser` (full C support)

**Opportunity**: Replace spydecy-c with decy-parser for better C support

**Benefits**:
- ✅ Full C language support (not just CPython patterns)
- ✅ Better error handling and diagnostics
- ✅ Shared maintenance burden
- ✅ Leverage decy's C parsing expertise

**Implementation**:
```rust
// Option A: Make spydecy-c depend on decy-parser
[dependencies]
decy-parser = "0.2.0"

// Option B: Make spydecy-c a thin wrapper around decy-parser
// with CPython pattern detection on top
```

**Status**: ✅ **COMPLETE** (Phase 2)

**Completed Actions**:
- [x] Evaluated decy-parser API compatibility
- [x] Created adapter layer (`decy_adapter.rs` - 280 lines)
- [x] Migrated spydecy-c to use decy-parser internally (all tests passing)
- [x] Maintained CPython pattern detection (100% compatible)
- [x] Added CPython type declarations for parser compatibility

---

### 2. 🌉 HIR Bridge Layer (Medium Priority)

**Problem**: Two different HIR systems:
- **Spydecy**: `UnifiedHIR` (Python + C + Rust concepts)
- **Decy**: `HirType`, `HirFunction`, etc. (C + Rust concepts)

**Opportunity**: Create a bridge between the two HIRs

**Design**:
```rust
// New crate: spydecy-decy-bridge
pub trait DecyHirAdapter {
    fn from_decy_hir(decy_hir: decy_hir::HirFunction) -> spydecy_hir::c::CHIR;
    fn to_unified(python: PythonHIR, c_from_decy: CHIR) -> UnifiedHIR;
}

// This allows:
// decy HIR → spydecy CHIR → UnifiedHIR → Rust
```

**Benefits**:
- ✅ Spydecy can leverage decy's advanced C parsing
- ✅ Spydecy maintains its cross-language unification
- ✅ Both projects remain independent but interoperable

**Action Items**:
- [ ] Design HIR mapping between decy and spydecy types
- [ ] Create `spydecy-decy-bridge` crate
- [ ] Add integration tests with real C code
- [ ] Document conversion semantics

---

### 3. 🎯 Shared Ownership Inference (High Value)

**Problem**: Spydecy doesn't have ownership/lifetime inference (decy's killer feature)

**Opportunity**: Use decy's ownership analysis to improve Spydecy's generated Rust

**Design**:
```rust
// Spydecy pipeline with decy ownership:
Python + C → UnifiedHIR
           → decy_ownership::analyze(unified_hir)
           → Optimized HIR with ownership annotations
           → Generate Rust with minimal unsafe
```

**Benefits**:
- ✅ Spydecy generates safer Rust code
- ✅ Reduce unsafe blocks in generated code
- ✅ Better borrow checker compatibility
- ✅ Leverage decy's sophisticated ownership algorithms

**Action Items**:
- [ ] Study decy-ownership crate API
- [ ] Design ownership annotation layer for UnifiedHIR
- [ ] Integrate ownership inference into spydecy optimizer
- [ ] Update codegen to emit ownership-aware Rust

---

### 4. 📊 Shared Debugger Foundation (Already Exists!)

**Status**: ✅ **Already integrated** - decy uses spydecy-debugger

**Current Integration**:
```rust
// decy/crates/decy-debugger/Cargo.toml
[dependencies]
spydecy-debugger = "0.1.0"
```

**Enhancement Opportunity**: Upgrade to spydecy-debugger 0.2.0

**Action Items**:
- [ ] Update decy dependency: `spydecy-debugger = "0.2.0"`
- [ ] Test C AST visualization with new version
- [ ] Share improvements bidirectionally

---

### 5. 🧪 Shared Testing Infrastructure (Low Priority)

**Opportunity**: Share test fixtures and quality standards

**Shared Elements**:
- PMAT quality analysis
- Mutation testing with cargo-mutants
- Property-based testing with proptest
- Coverage targets (80%+)
- Clippy configurations

**Action Items**:
- [ ] Create shared `test-fixtures` crate
- [ ] Standardize quality gate configurations
- [ ] Share C test cases between projects
- [ ] Cross-project benchmark comparisons

---

## Recommended Roadmap

### Phase 1: Foundation (Sprint 5) - 2 weeks
- [ ] Upgrade decy to spydecy-debugger 0.2.0
- [ ] Document existing integration
- [ ] Study decy-parser API thoroughly
- [ ] Design HIR bridge architecture

### Phase 2: C Parser Unification (Sprint 6-7) - 4 weeks
- [ ] Create spydecy-decy-bridge crate
- [ ] Implement decy_hir → spydecy_hir conversion
- [ ] Migrate spydecy-c to use decy-parser internally
- [ ] Maintain CPython pattern detection layer
- [ ] All 81 tests still passing

### Phase 3: Ownership Integration (Sprint 8-9) - 4 weeks
- [ ] Integrate decy-ownership into spydecy optimizer
- [ ] Add ownership annotations to UnifiedHIR
- [ ] Update codegen to emit safer Rust
- [ ] Measure unsafe code reduction

### Phase 4: Cross-Project Benefits (Sprint 10) - 2 weeks
- [ ] Share improvements bidirectionally
- [ ] Unified documentation
- [ ] Cross-project examples
- [ ] Joint testing infrastructure

---

## Architecture Vision

```
┌─────────────────────────────────────────────────────────┐
│                   User Code                             │
├─────────────────┬───────────────────────────────────────┤
│  Python Code    │         C Code                        │
└────────┬────────┴──────────────┬────────────────────────┘
         │                       │
         │ spydecy-python        │ decy-parser (shared!)
         │                       │
         ▼                       ▼
  ┌──────────────┐      ┌──────────────┐
  │ Python HIR   │      │  Decy HIR    │
  └──────┬───────┘      └──────┬───────┘
         │                     │
         │              spydecy-decy-bridge
         │                     │
         │                     ▼
         │              ┌──────────────┐
         │              │ Spydecy CHIR │
         │              └──────┬───────┘
         │                     │
         └──────────┬──────────┘
                    │
                    ▼
            ┌──────────────┐
            │ UnifiedHIR   │
            └──────┬───────┘
                   │
                   │ spydecy-optimizer
                   │    + decy-ownership (integrated!)
                   ▼
            ┌──────────────┐
            │ Optimized    │
            │ UnifiedHIR   │
            └──────┬───────┘
                   │
                   │ spydecy-codegen
                   ▼
            ┌──────────────┐
            │  Pure Rust   │
            │ (minimal     │
            │  unsafe!)    │
            └──────────────┘

     Debugging: spydecy-debugger (shared by both!)
```

---

## Key Design Principles

1. **Independence**: Both projects remain independently usable
2. **Shared Components**: Extract common functionality into shared crates
3. **Clean Boundaries**: Clear interfaces between projects
4. **Bidirectional Value**: Both projects benefit from integration
5. **Quality Maintained**: All quality gates continue passing

---

## Success Metrics

- [ ] Spydecy uses decy-parser for C parsing
- [ ] Decy uses spydecy-debugger 0.2.0
- [ ] spydecy-decy-bridge crate published to crates.io
- [ ] Spydecy generates Rust with <5 unsafe/1000 LOC (using decy-ownership)
- [ ] Both projects maintain 100% test pass rate
- [ ] Shared documentation and examples
- [ ] Cross-project integration tests passing

---

## Questions for Discussion

1. **C Parser Strategy**: Should spydecy-c be deprecated in favor of decy-parser, or should it remain as a thin wrapper?

2. **HIR Ownership**: Should UnifiedHIR be extended to include decy's ownership concepts, or keep them separate?

3. **Version Synchronization**: Should both projects maintain version parity (both 0.2.0), or evolve independently?

4. **Crate Naming**: Should the bridge crate be `spydecy-decy-bridge` or something more generic like `py-c-rust-bridge`?

5. **Maintenance Responsibility**: How do we coordinate releases and breaking changes across projects?

---

## Next Steps

1. **Immediate** (Today):
   - [ ] Review this plan with project stakeholders
   - [ ] Get approval for Phase 1 work

2. **This Week**:
   - [ ] Upgrade decy to spydecy-debugger 0.2.0
   - [ ] Begin decy-parser API study
   - [ ] Draft HIR bridge design doc

3. **Next Sprint**:
   - [ ] Implement spydecy-decy-bridge prototype
   - [ ] Create proof-of-concept integration
   - [ ] Run benchmarks comparing approaches

---

**Status**: Draft v1.0 - Ready for Review
**Author**: Claude Code
**Date**: 2025-10-22
