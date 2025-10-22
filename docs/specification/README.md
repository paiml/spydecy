# Spydecy Specifications

This directory contains technical specifications for the Spydecy project - a self-hosted Python/C-to-Rust compiler-debugger with introspective debugging capabilities.

## Overview

Spydecy combines Python-to-Rust and C-to-Rust transpilation in a unified system that can:
- Transpile Python source code to Rust
- Transpile CPython's C implementation to Rust
- Optimize across both layers
- Debug transpilation issues interactively
- Self-host (transpile itself for validation)

## 🎯 Start Here

### [Response to Gemini AI Review](RESPONSE-TO-GEMINI-REVIEW.md) ⭐ **READ THIS FIRST**
**Status**: All Recommendations Accepted ✅
**Impact**: Transforms theoretical spec into production-ready plan

**Critical Improvements**:
1. Sprint 0 Tracer Bullet - De-risks highest-risk assumption
2. Incremental Debugger Development - 50-100% velocity increase
3. Pluggable C-API Architecture - 10x ecosystem value

## Core Specifications

### [Transpiled Python/C-to-Rust Self-Hosted Compiler-Debugger](transpiled-python-c-to-rust-self-hosted-compiler-debugger.md)
**Status**: Specification Complete (Under Revision)
**Version**: 1.0
**Date**: 2025-10-21

The original specification document covering:

1. **Architecture Overview**
   - Multi-layer transpilation pipeline
   - Python transpilation engine
   - C transpilation engine
   - Unified HIR (High-level IR)
   - Cross-layer optimizer
   - Introspective debugger
   - Self-hosting capability

2. **Core Components**
   - Python Transpiler (depyler integration)
   - C Transpiler (decy integration)
   - Unified HIR representation
   - Cross-layer optimization
   - Interactive debugger
   - Bootstrap compiler

3. **Testing Strategy**
   - Multi-layer testing
   - Property-based testing (100+ properties × 1000 cases)
   - Mutation testing (≥90% kill rate)
   - Differential testing vs CPython/Numpy
   - Self-hosting validation

4. **Quality Gates**
   - PMAT integration
   - 80%+ test coverage
   - ≥90% mutation score
   - Zero SATD tolerance
   - Complexity ≤10 CCN
   - Zero clippy warnings

5. **Roadmap**
   - 20 sprints (40 weeks)
   - 80 detailed tickets
   - Phase 1: Foundation (sprints 1-5)
   - Phase 2: Optimization (sprints 6-10)
   - Phase 3: Debugger (sprints 11-15)
   - Phase 4: Self-Hosting (sprints 16-20)

## 📋 Implementation Documents (Post-Review)

### [Sprint 0: Tracer Bullet](SPRINT-0-TRACER-BULLET.md) 🎯 **MUST DO FIRST**
**Status**: Critical - Must Complete Before Main Project
**Duration**: 2 weeks
**Objective**: Validate Unified HIR concept

**Micro-Target**: Prove Python + C HIR unification works
- Python `len()` → C `list_length()` → Pure Rust `Vec::len()`
- **Go/No-Go Decision**: End of week 2
- **Cost**: $10K insurance on $400K project

### [Incremental Debugger Roadmap](INCREMENTAL-DEBUGGER-ROADMAP.md)
**Status**: Revised Development Plan
**Philosophy**: "Eat Your Own Dog Food"

**Key Change**: Build debugger **alongside transpiler** (not after)
- Sprint 2: Python parser + `visualize python-ast` debugger
- Sprint 4: Unified HIR + **interactive step-through debugger** ⭐
- Impact: 50-100% velocity increase

### [Pluggable C-API Architecture](PLUGGABLE-C-API-ARCHITECTURE.md)
**Status**: Architecture Refactoring
**Impact**: 10x increase in project value

**Key Change**: Trait-based, extensible C-API analysis
- `CPythonAnalyzer` (Sprint 3)
- `NumPyAnalyzer` (Sprint 7)
- `SciPyAnalyzer` (Sprint 10)
- Community plugins (Sprint 15+)

**Ecosystem**: CPython + NumPy + SciPy + Pandas + plugins

## Key Innovations

1. **Unified Python/C Transpilation**
   - Leverages the fact that CPython is written in C
   - Optimizes across language boundaries
   - Eliminates Python-C call overhead
   - **NEW**: Validated via Sprint 0 tracer bullet

2. **Introspective Debugging**
   - Step through transpilation process
   - Visualize transformations
   - Identify issues at Python-C boundary
   - Auto-fix suggestions
   - **NEW**: Built incrementally from Sprint 2 (not deferred to Sprint 11)

3. **Self-Hosting Validation**
   - Compiler transpiles itself
   - Bootstrap process: Stage 0 → Stage 1 → Stage 2
   - Fixed-point validation ensures correctness
   - Ultimate correctness guarantee

4. **CPython/Numpy Ready**
   - Specifically designed for CPython transpilation
   - Handles PyObject*, reference counting, GIL
   - Optimized for numpy arrays and scientific computing
   - **NEW**: Pluggable architecture supports NumPy, SciPy, Pandas, and community extensions

5. **Depyler/Decy Enhancement**
   - Provides debugging tools for existing transpilers
   - Identifies tricky transpilation issues
   - Helps improve both tools

## Use Cases

### 1. CPython to Rust
```bash
spydecy transpile-cpython Objects/dictobject.c \
    --output cpython-rust/dict.rs \
    --optimize cross-layer
```

### 2. Numpy to Rust
```bash
spydecy transpile-numpy numpy/core/src/multiarray/ \
    --output numpy-rust/ \
    --use-ndarray
```

### 3. Python + CPython Together
```bash
spydecy transpile-unified myproject/ \
    --include-cpython-deps \
    --inline-c-calls
```

### 4. Debug Transpilation Issues
```bash
spydecy debug --target depyler \
    --source problematic_python.py \
    --visualize
```

### 5. Self-Hosting Bootstrap
```bash
spydecy bootstrap \
    --stage0 target/release/spydecy \
    --validate
```

## Quality Metrics

| Metric | Target | Status |
|--------|--------|--------|
| Test Coverage | ≥80% | ⏳ TBD |
| Mutation Score | ≥90% | ⏳ TBD |
| Transpilation Success | ≥95% | ⏳ TBD |
| Unsafe Code Density | <5 per 1000 LOC | ⏳ TBD |
| Self-Hosting | 100% | ⏳ TBD |

## Architecture Diagram

```
┌─────────────────────────────────────────────────┐
│              Spydecy System                     │
├─────────────────────────────────────────────────┤
│                                                 │
│  ┌─────────────┐         ┌─────────────┐      │
│  │   Python    │         │      C      │      │
│  │   Source    │         │   Source    │      │
│  └──────┬──────┘         └──────┬──────┘      │
│         │                       │              │
│         ▼                       ▼              │
│  ┌─────────────┐         ┌─────────────┐      │
│  │  Depyler    │         │    Decy     │      │
│  │ Integration │         │ Integration │      │
│  └──────┬──────┘         └──────┬──────┘      │
│         │                       │              │
│         └───────────┬───────────┘              │
│                     ▼                          │
│           ┌──────────────────┐                 │
│           │   Unified HIR    │                 │
│           └─────────┬────────┘                 │
│                     ▼                          │
│           ┌──────────────────┐                 │
│           │  Cross-Layer     │                 │
│           │   Optimizer      │                 │
│           └─────────┬────────┘                 │
│                     ▼                          │
│           ┌──────────────────┐                 │
│           │  Rust Codegen    │                 │
│           └─────────┬────────┘                 │
│                     ▼                          │
│           ┌──────────────────┐                 │
│           │  Introspective   │                 │
│           │    Debugger      │                 │
│           └──────────────────┘                 │
│                                                 │
└─────────────────────────────────────────────────┘
```

## Dependencies

- **depyler**: Python-to-Rust transpiler
- **decy**: C-to-Rust transpiler
- **bashrs**: Reference for quality gates
- **ruchy**: Reference for self-hosting
- **PyO3**: Python-Rust bindings
- **clang-sys**: C parser
- **PMAT**: Quality enforcement framework

## Development Status

### Phase 0: Risk Mitigation (NEW)
- ✅ Gemini AI Review Complete
- ✅ All recommendations accepted and documented
- ⏳ **Sprint 0: Tracer Bullet** (not started) ← **START HERE**

### Phase 1-4: Main Implementation
- ⏳ Sprint 1-20: Full implementation (pending Sprint 0 Go decision)

## Next Steps (Revised)

### Immediate Priority: Sprint 0 Tracer Bullet
1. **Secure team** (1-2 senior engineers)
2. **Week 1**: Manual transpilation + Minimal HIR design
3. **Week 2**: Optimizer + End-to-end test
4. **Go/No-Go Decision**: End of week 2

### If Sprint 0 Succeeds (Go Decision)
1. **Sprint 1**: Project Setup
   - DECY-001: Setup monorepo structure
   - DECY-002: Integrate depyler and decy as libraries
   - DECY-003: Setup quality gates (PMAT)
   - DECY-004: Create test infrastructure

2. **Sprint 2**: Python Transpiler + First Debugger Features
   - DECY-005-008: Python parser, type hints, HIR
   - DECY-005-DEBUG: `spydecy debug visualize python-ast`

3. **Sprint 3**: C Transpiler + C-API Trait
   - DECY-009-012: C parser, CPython API, C HIR
   - **NEW**: `C_API_Analyzer` trait implementation
   - DECY-009-DEBUG: `spydecy debug visualize c-ast`

4. **Sprint 4**: Unified HIR + Interactive Debugger ⭐ **CRITICAL**
   - DECY-013-016: Unified type system, cross-language refs
   - **NEW**: `spydecy debug step --from python --to rust` (interactive)

### If Sprint 0 Fails (No-Go Decision)
- **Architectural Pivot**: Separate Python/C transpilers (no unification)
- **Alternative**: Focus on depyler/decy integration without cross-layer optimization
- **Research Phase**: Solve HIR impedance mismatch before continuing

## Contributing

See the main specification document for:
- Development methodology (EXTREME TDD)
- Quality standards
- Testing requirements
- Code review process

## License

[To be determined]

---

**Last Updated**: 2025-10-21
**Status**: Specification Phase
**Version**: 1.0
