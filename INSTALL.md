# Installing Spydecy

## Current Status (v0.1.0)

Spydecy is **not yet published to crates.io**. Use one of the methods below:

---

## Method 1: Install from Git (Recommended) ✅

Install directly from the GitHub repository:

```bash
# Install latest release (v0.1.0)
cargo install --git https://github.com/paiml/spydecy.git --tag v0.1.0 spydecy

# Or install from main branch
cargo install --git https://github.com/paiml/spydecy.git spydecy
```

### Verify Installation

```bash
spydecy --version
# Should output: spydecy 0.1.0

spydecy --help
# Shows available commands
```

---

## Method 2: Install from Local Path

If you have the source code:

```bash
cd /path/to/spydecy
cargo install --path .
```

This installs to `~/.cargo/bin/spydecy`

---

## Method 3: Build from Source

For development or custom builds:

```bash
# Clone the repository
git clone https://github.com/paiml/spydecy.git
cd spydecy

# Checkout release tag
git checkout v0.1.0

# Build release
cargo build --release

# Binary will be at: target/release/spydecy

# Optionally, copy to PATH
sudo cp target/release/spydecy /usr/local/bin/
```

---

## System Requirements

### For Python Support
```bash
sudo apt-get install python3.10-dev
```

### For C Support
```bash
sudo apt-get install libclang-14-dev
```

### Verify Dependencies
```bash
# Check Python dev headers
python3-config --includes

# Check libclang
ldconfig -p | grep clang
```

---

## Using as a Library (decy/deypler integration)

Instead of installing the binary, use spydecy crates as dependencies:

### In your `Cargo.toml`:

```toml
[dependencies]
# For HIR types (shared)
spydecy-hir = { path = "../spydecy/crates/spydecy-hir" }

# For Python parsing (deypler)
spydecy-python = { path = "../spydecy/crates/spydecy-python" }

# For C parsing (decy)
spydecy-c = { path = "../spydecy/crates/spydecy-c" }
```

Or from git:

```toml
[dependencies]
spydecy-hir = { git = "https://github.com/paiml/spydecy.git", tag = "v0.1.0" }
spydecy-python = { git = "https://github.com/paiml/spydecy.git", tag = "v0.1.0" }
spydecy-c = { git = "https://github.com/paiml/spydecy.git", tag = "v0.1.0" }
```

---

## Testing Installation

After installation, test with:

```bash
# Create a test Python file
echo 'def hello(): print("Hello")' > test.py

# Visualize the AST
spydecy debug --visualize test.py

# Should display colored AST tree
```

---

## Troubleshooting

### Error: `libclang.so` not found

```bash
# Install libclang
sudo apt-get install libclang-14-dev

# Set environment variable
export LIBCLANG_PATH=/usr/lib/llvm-14/lib
```

### Error: `Python.h` not found

```bash
# Install Python dev headers
sudo apt-get install python3.10-dev
```

### Error: `cargo install` fails

```bash
# Try with verbose output
cargo install --git https://github.com/paiml/spydecy.git --tag v0.1.0 spydecy --verbose

# Or build locally
git clone https://github.com/paiml/spydecy.git
cd spydecy
cargo build --release
```

---

## Future: crates.io Publishing

**Coming in v0.2.0:**

Once published to crates.io, you'll be able to install with:

```bash
cargo install spydecy
```

**Why not published yet?**
- v0.1.0 is a foundation release
- Dependencies need crates.io publishing
- Quality gates need completion
- Documentation finalization needed

**Timeline**: Expected by v0.2.0 (2-3 weeks)

---

## Updating Spydecy

### Git Install
```bash
cargo install --git https://github.com/paiml/spydecy.git --force
```

### Local Install
```bash
cd spydecy
git pull
cargo install --path . --force
```

---

## Uninstalling

```bash
cargo uninstall spydecy
```

---

## Quick Links

- **Repository**: https://github.com/paiml/spydecy
- **Documentation**: See README.md
- **Issues**: GitHub Issues
- **Integration Guide**: See QUICK-START.md
