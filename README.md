# Spydecy - Self-Hosted Python/C-to-Rust Compiler-Debugger

[![Crates.io](https://img.shields.io/crates/v/spydecy.svg)](https://crates.io/crates/spydecy)
[![CI Status](https://github.com/noahgift/spydecy/workflows/CI%20-%20Quality%20Gates/badge.svg)](https://github.com/noahgift/spydecy/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/spydecy.svg)](https://crates.io/crates/spydecy)

**Version 0.2.0 Released** 🎉 • **Full Pipeline Working!** • **Decy Integration Complete!** • **99 Tests Passing**

Unified Python and C-to-Rust transpilation with introspective debugging capabilities.

## 📦 Published Crates

| Crate | Version | Description |
|-------|---------|-------------|
| [spydecy](https://crates.io/crates/spydecy) | 0.2.0 | Main CLI application |
| [spydecy-hir](https://crates.io/crates/spydecy-hir) | 0.2.0 | Unified HIR (High-level IR) |
| [spydecy-python](https://crates.io/crates/spydecy-python) | 0.2.0 | Python AST parser (PyO3) |
| [spydecy-debugger](https://crates.io/crates/spydecy-debugger) | 0.2.0 | Introspective debugger |

## 🎯 Quick Start

### Installation

#### From crates.io (Recommended)

```bash
# Install the latest release
cargo install spydecy

# Verify installation
spydecy --version
```

#### For Development

**Prerequisites:**
- Rust 1.75.0+
- Python 3.10-dev: `sudo apt-get install python3.10-dev`
- libclang-14-dev: `sudo apt-get install libclang-14-dev`
- PMAT: `cargo install pmat`

```bash
# Clone repository
git clone https://github.com/noahgift/spydecy.git
cd spydecy

# Install all development tools
make install-tools

# Setup project
make setup

# Build
make build

# Run quality gates
make quality-gate
```

### Usage

```bash
# Compile Python + C to Rust (full pipeline)
spydecy compile --python my_file.py --c my_file.c --output my_file.rs --verbose

# Quick compile (non-verbose)
spydecy compile --python my_file.py --c my_file.c --output my_file.rs

# Visualize Python AST (NEW!)
spydecy debug --visualize example.py

# Visualize C AST with CPython API detection (NEW!)
spydecy debug --visualize example.c

# Show project info and status
spydecy info

# Get help
spydecy --help
```

## ✨ New in v0.2.0

### Sprint 3: C File Debugging
```bash
# Visualize C AST with CPython API highlighting
spydecy debug --visualize list.c
```

Features:
- 🎨 Color-coded syntax highlighting
- ⚡ CPython API call detection (PyList_Append, Py_SIZE, etc.)
- 🐍 PyObject* parameter tracking
- 📊 Comprehensive statistics

### Phase 2: Decy Integration
Spydecy now uses [decy-parser](https://github.com/noahgift/decy) for comprehensive C parsing:
- ✅ Full C language support (not just CPython patterns)
- ✅ Better error diagnostics
- ✅ Shared maintenance with decy project
- ✅ Foundation for Phase 3 (ownership analysis)

**Architecture**: `decy-parser → adapter → spydecy CAST → Unified HIR → Rust`

## 📚 Documentation

**Start Here**: [Response to Gemini AI Review](docs/specification/RESPONSE-TO-GEMINI-REVIEW.md) ⭐

### Critical Documents
1. **[Sprint 0: Tracer Bullet](docs/specification/SPRINT-0-TRACER-BULLET.md)** - 2-week validation sprint
2. **[Incremental Debugger Roadmap](docs/specification/INCREMENTAL-DEBUGGER-ROADMAP.md)** - Build debugger alongside transpiler
3. **[Decy Integration Plan](DECY-INTEGRATION-PLAN.md)** - Phase 2 Complete ✅
4. **[Pluggable C-API Architecture](docs/specification/PLUGGABLE-C-API-ARCHITECTURE.md)** - Extensible C-API analysis

### Full Specification
- [Main Specification](docs/specification/transpiled-python-c-to-rust-self-hosted-compiler-debugger.md)
- [Specification Index](docs/specification/README.md)

## 🧪 Quality Standards

| Metric | Target | Current | Enforcement |
|--------|--------|---------|-------------|
| Test Coverage | ≥80% | 99/99 ✅ | PMAT + CI |
| Mutation Score | ≥90% | TBD | cargo-mutants |
| Complexity | ≤10 CCN | ✅ | PMAT pre-commit |
| SATD Comments | 0 | 0 ✅ | PMAT (zero tolerance) |
| Clippy Warnings | 0 | CI/CD |
| Unsafe Code | <5 per 1000 LOC | Static analysis |

## 🚀 Development

### Quality Gates
```bash
# Fast quality check
make quality-fast

# Full quality gate (with coverage & mutation)
make quality-gate

# Pre-commit checks
make pre-commit

# Continuous improvement analysis
make kaizen
```

### Testing
```bash
# Run all tests
make test

# Property-based tests (1000 cases/property)
make test-property

# Mutation testing
make mutants

# Code coverage
make coverage
```

### Development Mode
```bash
# Auto-reload on changes
make dev

# Watch and run tests
make watch
```

## 🏗️ Project Status

**Current Version**: v0.2.0 (Released 2025-10-22)
**Current Phase**: Sprint 3 Complete ✅ - Planning Sprint 4

### Completed Milestones ✅

- ✅ **Sprint 0**: Tracer Bullet Validation - Core assumption proven: `len()` unification works!
- ✅ **Sprint 2**: Python Transpiler - Full Python AST parsing with PyO3 (36/36 tests)
- ✅ **v0.1.0 Release**: Published to crates.io with initial foundation
- ✅ **Sprint 3**: C Transpiler Foundation (51/51 tests)
  - C parser complete using clang-sys
  - CPython API pattern recognition working
  - **CORE INNOVATION PROVEN**: Python + C → Rust unification end-to-end!
- ✅ **v0.2.0 Release**: Unification Milestone 🎉
  - Python `len(x)` + C `list_length()` → Rust `Vec::len()` working
  - Complete pipeline validated with production parsers
  - Boundary elimination demonstrated

### Next Steps 🚀

- **Sprint 4**: Cross-Layer Optimization (Est. 2-4 weeks)
  - Add more unification patterns (append, dict.get)
  - Implement C debugger visualization
  - Begin optimizer implementation
  - Performance benchmarking

### Roadmap

- **v0.3.0** (Est. 4-6 weeks): Expanded patterns + optimizer
- **v0.4.0** (Est. 8-10 weeks): Full code generation pipeline
- **v1.0.0** (Est. 12-16 weeks): Production ready - Self-hosting capability

See [CHANGELOG.md](CHANGELOG.md) for detailed release notes.

## 🔬 Architecture

### Multi-Layer Pipeline
```
Python Source → Python HIR ─┐
                            ├─→ Unified HIR → Optimizer → Rust
C Source → C HIR ───────────┘
```

### Key Innovations
1. **Unified Python/C Transpilation** - Leverages CPython's C implementation
2. **Introspective Debugging** - Step through transpilation process
3. **Pluggable C-API** - CPython, NumPy, SciPy, community plugins
4. **Self-Hosting** - Compiler transpiles itself for validation

## 📊 Makefile Targets

```bash
make help              # Show all available targets
make install-tools     # Install all dev tools (PMAT, etc.)
make quality-gate      # Run full quality gate suite
make kaizen            # Continuous improvement analysis
make ci                # Full CI pipeline
```

## 🤝 Contributing

### Development Methodology: EXTREME TDD

1. **RED**: Write failing tests first
2. **GREEN**: Minimal implementation
3. **REFACTOR**: Meet quality gates (≤10 CCN, 0 SATD, 80%+ coverage)

### Quality Requirements
- ✅ All tests pass
- ✅ Coverage ≥80%
- ✅ Mutation score ≥90%
- ✅ Complexity ≤10 CCN
- ✅ Zero SATD comments (TODO/FIXME/HACK)
- ✅ Zero Clippy warnings
- ✅ Code formatted with rustfmt

### Pre-Commit Hooks
Pre-commit hooks automatically enforce quality gates:
- Code formatting
- Clippy lints
- PMAT complexity & SATD checks
- Fast test suite

## 📜 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🤝 Sister Projects

**Spydecy** is part of a family of transpiler projects:

- **[decy](https://github.com/paiml/decy)**: C-to-Rust transpiler with ownership inference
  - ✅ **Already integrated**: decy uses spydecy-debugger for C AST visualization
  - 🔄 **Integration planned**: See [DECY-INTEGRATION-PLAN.md](DECY-INTEGRATION-PLAN.md)
  - **Opportunity**: Use decy's advanced C parser and ownership analysis in Spydecy

## 🙏 Acknowledgments

Inspired by:
- **depyler**: Python-to-Rust transpiler
- **decy**: C-to-Rust transpiler with extreme quality
- **bashrs**: Formal verification and property testing
- **ruchy**: Self-hosting and PMAT integration
- **Toyota Production System**: Jidoka, Kaizen, Genchi Genbutsu

---

**Built with EXTREME quality standards. Zero compromises. 🚀**

**Status**: v0.2.0 Released to crates.io ✅
**Achievement**: 🎉 Core Innovation Proven End-to-End
**Current**: Sprint 3 Complete - Planning Sprint 4
**Next**: v0.3.0 - Expanded patterns + optimizer

📦 **Install now**: `cargo install spydecy`
📖 **Documentation**: [CHANGELOG.md](CHANGELOG.md)
🔗 **Crates.io**: https://crates.io/crates/spydecy
