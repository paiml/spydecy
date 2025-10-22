# Spydecy - Self-Hosted Python/C-to-Rust Compiler-Debugger

[![CI Status](https://github.com/noahgift/spydecy/workflows/CI%20-%20Quality%20Gates/badge.svg)](https://github.com/noahgift/spydecy/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

**EXTREME TDD** • **Zero Tolerance SATD** • **80%+ Coverage** • **90%+ Mutation Score**

Unified Python and C-to-Rust transpilation with introspective debugging capabilities.

## 🎯 Quick Start

### Prerequisites
- Rust 1.75.0+
- PMAT (cargo install pmat)
- Standard Rust tooling (rustfmt, clippy, cargo-llvm-cov, cargo-mutants)

### Installation

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

## 📚 Documentation

**Start Here**: [Response to Gemini AI Review](docs/specification/RESPONSE-TO-GEMINI-REVIEW.md) ⭐

### Critical Documents
1. **[Sprint 0: Tracer Bullet](docs/specification/SPRINT-0-TRACER-BULLET.md)** - 2-week validation sprint
2. **[Incremental Debugger Roadmap](docs/specification/INCREMENTAL-DEBUGGER-ROADMAP.md)** - Build debugger alongside transpiler
3. **[Pluggable C-API Architecture](docs/specification/PLUGGABLE-C-API-ARCHITECTURE.md)** - Extensible C-API analysis

### Full Specification
- [Main Specification](docs/specification/transpiled-python-c-to-rust-self-hosted-compiler-debugger.md)
- [Specification Index](docs/specification/README.md)

## 🧪 Quality Standards

| Metric | Target | Enforcement |
|--------|--------|-------------|
| Test Coverage | ≥80% | PMAT + CI |
| Mutation Score | ≥90% | cargo-mutants |
| Complexity | ≤10 CCN | PMAT pre-commit |
| SATD Comments | 0 | PMAT (zero tolerance) |
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

**Current Phase**: Sprint 0 Preparation

### Phase 0: Risk Mitigation (NEW)
- ✅ Gemini AI Review Complete
- ✅ All recommendations accepted and documented
- ⏳ Sprint 0: Tracer Bullet (2 weeks) ← **NEXT**

### Roadmap
- **Sprint 0**: Tracer Bullet validation (2 weeks)
- **Sprint 1-5**: Foundation (Python/C transpilers, Unified HIR)
- **Sprint 6-10**: Optimization (Cross-layer optimizer, NumPy/SciPy)
- **Sprint 11-15**: Advanced Debugger (Graphical, LSP, MCP)
- **Sprint 16-20**: Self-Hosting (Bootstrap, production hardening)

See [RESPONSE-TO-GEMINI-REVIEW.md](docs/specification/RESPONSE-TO-GEMINI-REVIEW.md) for details.

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

## 🙏 Acknowledgments

Inspired by:
- **depyler**: Python-to-Rust transpiler
- **decy**: C-to-Rust transpiler with extreme quality
- **bashrs**: Formal verification and property testing
- **ruchy**: Self-hosting and PMAT integration
- **Toyota Production System**: Jidoka, Kaizen, Genchi Genbutsu

---

**Built with EXTREME quality standards. Zero compromises. 🚀**

**Status**: Sprint 0 Preparation
**Next**: Run 2-week tracer bullet to validate Unified HIR
