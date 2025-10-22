# Installation

## From crates.io (Recommended)

```bash
cargo install spydecy
```

Verify the installation:

```bash
spydecy --version
```

## From Source

**Prerequisites:**
- Rust 1.75.0+
- Python 3.10-dev: `sudo apt-get install python3.10-dev`
- libclang-14-dev: `sudo apt-get install libclang-14-dev`
- PMAT: `cargo install pmat`

**Clone and build:**

```bash
git clone https://github.com/noahgift/spydecy.git
cd spydecy
make install-tools
cargo build --release
```

## Development Setup

For contributing to Spydecy:

```bash
# Install all development tools
make install-tools

# Run quality gates
make pre-commit

# Run tests
make test
```

## Next Steps

- [Quick Start](./quick-start.md) - Your first unification example
- [CLI Reference](./cli-reference.md) - Complete command reference
