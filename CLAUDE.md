# Claude Code Development Guide for Spydecy

This document contains important information for Claude Code when working on the Spydecy project.

## Project Overview

Spydecy is a self-hosted Python/C-to-Rust compiler-debugger with introspective debugging capabilities. The core innovation is a Unified HIR (High-level Intermediate Representation) that unifies Python and C representations for cross-layer optimization.

**Key Achievement**: Sprint 0 validated the core assumption - Python `len()` + C `list_length()` can be unified to pure Rust `Vec::len()` with zero FFI and zero unsafe code.

## Repository Structure

```
spydecy/
├── crates/
│   ├── spydecy-hir/         # Unified HIR (core innovation)
│   ├── spydecy-python/      # Python AST parser (PyO3)
│   ├── spydecy-c/           # C AST parser (clang-sys)
│   ├── spydecy-debugger/    # Introspective debugger
│   ├── spydecy-optimizer/   # Cross-layer optimizer
│   ├── spydecy-codegen/     # Rust code generator
│   ├── spydecy-analyzers/   # Static analyzers
│   └── spydecy-bootstrap/   # Bootstrapping tools
├── sprint0-tracer-bullet/   # Sprint 0 validation (SUCCESS ✅)
├── src/                     # Main CLI application
└── docs/                    # Documentation
```

## Development Workflow

### Before Starting Work

**ALWAYS** generate fresh project context for optimal code understanding:

```bash
pmat context --output deep_context.md
```

This file is gitignored and should be regenerated frequently to get the latest project state.

### Quality Gates

The project uses **extreme quality standards** enforced by pre-commit hooks:

```bash
make pre-commit   # Run before committing
```

Quality checks include:
1. **Formatting**: `cargo fmt --all`
2. **Linting**: `cargo clippy` with `-D warnings` (all warnings are errors)
3. **PMAT Analysis**: Complexity and SATD detection
4. **Fast Tests**: Unit tests must pass
5. **Build Verification**: Project must compile

### Allowed Clippy Lints

Due to the strict linting policy, certain lints are allowed at the crate level:

**spydecy-hir**: Test-related lints
- `clippy::expect_used`, `clippy::panic`, `clippy::similar_names` (tests only)
- `clippy::module_name_repetitions`, `clippy::unnested_or_patterns`

**spydecy-python**: PyO3 and test-related
- `clippy::doc_markdown`, `clippy::single_match`, `clippy::str_to_string`
- `clippy::unwrap_used`, `clippy::panic`

**spydecy-c**: clang-sys FFI-related
- `clippy::doc_markdown`, `clippy::borrow_as_ptr`, `clippy::ptr_as_ptr`
- `clippy::ptr_cast_constness`, `clippy::cast_sign_loss`
- `clippy::ref_option`, `clippy::wildcard_imports`
- Unsafe code is **only** allowed in this crate for FFI

**spydecy-debugger**: Formatting and test-related
- `clippy::format_push_string`, `clippy::uninlined_format_args`
- `clippy::str_to_string`, `clippy::unwrap_used`

### Crates.io Publishing

The project is a workspace with multiple crates that must be published in dependency order:

1. `spydecy-hir` (no workspace dependencies)
2. `spydecy-python` (depends on spydecy-hir)
3. `spydecy-debugger` (depends on spydecy-hir + spydecy-python)
4. `spydecy` (main crate, depends on spydecy-debugger)

**Important**: All workspace crates have been configured with:
- `description`, `repository`, `authors`, `keywords`, `categories`
- Version specifications for path dependencies (e.g., `version = "0.1.0", path = "..."`)

### Commit Guidelines

**DO NOT commit** without passing pre-commit checks. If checks fail:

1. Fix formatting: `make format`
2. Fix lints: `make lint`
3. Fix PMAT issues: Reduce complexity or remove SATD comments

Use the commit message format:
```
Brief description of change

- Detailed change 1
- Detailed change 2

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

## Architecture Principles

### Zero Tolerance Policies

1. **NO TODO/FIXME/HACK comments** (`clippy::todo` = deny)
2. **NO unwrap/expect in production code** (allowed only in tests)
3. **NO unsafe code** (except spydecy-c for FFI)
4. **NO dead code** (`dead_code` = deny)
5. **NO unused imports/variables** (`unused_imports/variables` = deny)

### Core Innovation: Unified HIR

The Unified HIR is the heart of Spydecy. It recognizes patterns between Python and C:

```
Python len(x)  ←─────┐
                      ├──→ Unified HIR ──→ Rust x.len()
C list_length() ──────┘
```

**Pattern Recognition Examples**:
- `len()` + `list_length()` → `Vec::len()`
- `append()` + `PyList_Append()` → `Vec::push()`
- `dict.get()` + `PyDict_GetItem()` → `HashMap::get()`

See `crates/spydecy-hir/src/unified.rs:Unifier::unify()` for the implementation.

### Testing Philosophy

- **TDD Required**: Write tests before implementation
- **Property-Based Testing**: Use `proptest` for edge cases
- **Mutation Testing**: Use `cargo-mutants` to verify test quality
- **Fast Tests**: Unit tests must run in < 2 seconds

## Common Tasks

### Adding a New Feature

1. Generate context: `pmat context --output deep_context.md`
2. Write tests first (TDD)
3. Implement feature
4. Run quality gates: `make pre-commit`
5. Commit with proper message

### Fixing a Bug

1. Generate context: `pmat context --output deep_context.md`
2. Write a failing test that reproduces the bug
3. Fix the bug
4. Verify all tests pass: `make test`
5. Run quality gates: `make pre-commit`

### Refactoring

1. Generate context: `pmat context --output deep_context.md`
2. Ensure all tests pass before starting
3. Make incremental changes
4. Run tests after each change: `cargo test`
5. Run quality gates: `make pre-commit`

### Adding Dependencies

- Keep dependencies minimal
- Prefer well-maintained crates
- Add to appropriate workspace member's Cargo.toml
- Document why the dependency is needed

## Performance Targets

From SPECIFICATION.md Section 30:

- **Compile Time**: < 10s for 1000 LOC Python
- **Memory**: < 100MB peak for 1000 LOC
- **Generated Code**: Within 20% of hand-written Rust performance

## Useful Commands

```bash
# Development
make dev              # Start development environment
make build            # Build all workspace members
make test             # Run all tests
make bench            # Run benchmarks

# Quality
make format           # Format all code
make lint             # Run clippy
make pre-commit       # Run all pre-commit checks
make pmat-check       # Run PMAT quality analysis
make pmat-report      # Generate PMAT reports

# PMAT Context
pmat context --output deep_context.md   # Generate project context
pmat analyze complexity --path .        # Analyze complexity
pmat analyze satd --path .              # Find technical debt

# Publishing (in order)
cd crates/spydecy-hir && cargo publish
cd crates/spydecy-python && cargo publish
cd crates/spydecy-debugger && cargo publish
cargo publish

# Git Workflow
git add <files>
git commit -m "message"    # Triggers pre-commit hooks
git push
```

## Known Issues & Workarounds

### Pre-commit Hook PMAT Command

The Makefile had incorrect PMAT command syntax. Fixed in this version:
- **Old** (broken): `pmat analyze complexity . --fail-on-violation`
- **New** (correct): `pmat analyze complexity --path . --fail-on-violation`

### Clippy Lints in Workspace

The workspace has very strict clippy configuration. Some lints are allowed at crate-level for valid reasons (FFI, tests, formatting). See "Allowed Clippy Lints" section above.

## Resources

- **Main Documentation**: `docs/`
- **Architecture**: `ARCHITECTURE.md`
- **Roadmap**: `ROADMAP.md`
- **Sprint 0 Success**: `sprint0-tracer-bullet/`
- **Specification**: `SPECIFICATION.md`
- **Quick Start**: `QUICK-START.md`

## Claude Code Best Practices

1. **Always** generate `deep_context.md` before major work
2. **Always** run `make pre-commit` before committing
3. **Never** add TODO/FIXME/HACK comments
4. **Never** use `unwrap()` in production code (tests are OK)
5. **Always** write tests first (TDD)
6. **Always** use `.to_owned()` instead of `.to_string()` for string literals
7. **Always** use `expect()` with descriptive messages instead of `unwrap()`
8. **Never** commit without fixing all clippy warnings
9. **Always** respect the zero-tolerance policies
10. **Always** follow the crates.io publishing order

## Version

- **Current Version**: 0.1.0 (Sprint 0 Complete ✅)
- **Rust Version**: 1.75.0 (MSRV)
- **Last Updated**: 2025-10-22
