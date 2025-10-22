# Spydecy v0.1.0 - Quick Start for decy/deypler

**Release**: v0.1.0 ✅
**Status**: SHIPPED 🚀
**Location**: `github.com:paiml/spydecy.git`

---

## 🎯 For decy Integration

### Step 1: Add Dependency

In `decy/Cargo.toml`:

```toml
[dependencies]
# Add spydecy crates
spydecy-hir = { path = "../spydecy/crates/spydecy-hir" }
spydecy-c = { path = "../spydecy/crates/spydecy-c" }
```

### Step 2: Use C Parser

```rust
// In decy/src/lib.rs or decy-parser
use spydecy_c::parse_c_file;
use spydecy_hir::c::CHIR;

pub fn parse_with_spydecy(path: &Path) -> anyhow::Result<CHIR> {
    spydecy_c::parse_c_file(path)
}
```

### Step 3: CPython API Recognition

```rust
use spydecy_c::cpython::{identify_pattern, CPythonPattern};
use spydecy_hir::c::CHIR;

fn handle_cpython_api(hir: &CHIR) {
    if let Some(pattern) = identify_pattern(hir) {
        match pattern {
            CPythonPattern::ListLength => {
                // Handle list_length pattern
                println!("Found list_length - maps to Vec::len()");
            }
            CPythonPattern::ListAppend => {
                // Handle PyList_Append pattern
            }
            _ => {}
        }
    }
}
```

### System Requirements for decy

```bash
sudo apt-get install libclang-14-dev
```

---

## 🎯 For deypler Integration

### Step 1: Add Dependency

In `deypler/Cargo.toml`:

```toml
[dependencies]
# Add spydecy crates
spydecy-hir = { path = "../spydecy/crates/spydecy-hir" }
spydecy-python = { path = "../spydecy/crates/spydecy-python" }
```

### Step 2: Use Python Parser

```rust
// In deypler/src/lib.rs
use spydecy_python::parse_python;
use spydecy_hir::python::PythonHIR;

pub fn parse_with_spydecy(source: &str, filename: &str) -> anyhow::Result<PythonHIR> {
    spydecy_python::parse_python(source, filename)
}
```

### Step 3: Work with Python HIR

```rust
use spydecy_hir::python::{PythonHIR, BinOp};

fn handle_python_node(hir: &PythonHIR) {
    match hir {
        PythonHIR::Function { name, params, body, .. } => {
            println!("Function: {}", name);
            for param in params {
                println!("  Param: {}", param.name);
            }
        }
        PythonHIR::Call { callee, args, .. } => {
            println!("Call with {} args", args.len());
        }
        _ => {}
    }
}
```

### System Requirements for deypler

```bash
sudo apt-get install python3.10-dev
```

---

## 📦 Shared HIR Types

Both decy and deypler can use the same HIR types:

```rust
use spydecy_hir::{
    types::{Type, PythonType, CType, RustType},
    metadata::Metadata,
    NodeId,
};

// Example: Type compatibility check
let python_list = Type::Python(PythonType::List(Box::new(Type::Python(PythonType::Int))));
let rust_vec = Type::Rust(RustType::Vec(Box::new(Type::Rust(RustType::Int {
    bits: IntSize::I32,
    signed: true
}))));

if python_list.is_compatible(&rust_vec) {
    println!("Types are compatible for unification!");
}
```

---

## 🔧 Build & Test

```bash
# Clone spydecy
cd ../spydecy

# Build release
cargo build --release

# Run tests
cargo test --workspace

# All 47 tests should pass ✅
```

---

## 📊 What's Included

### spydecy-hir (Shared)
- ✅ Unified HIR types
- ✅ Type system (Python, C, Rust)
- ✅ Metadata tracking
- ✅ 17 tests passing

### spydecy-python (For deypler)
- ✅ Python AST parser (PyO3)
- ✅ Python → HIR conversion
- ✅ Type hint support
- ✅ 7 tests passing

### spydecy-c (For decy)
- ✅ C AST parser (clang-sys)
- ✅ CPython API recognition
- ✅ C → HIR conversion
- ✅ 11 tests passing

---

## 🎯 Example: Full Integration

### decy with spydecy

```rust
// decy/src/transpile.rs
use spydecy_c::parse_c_file;
use spydecy_hir::c::CHIR;

pub fn transpile_c_to_rust(path: &Path) -> anyhow::Result<String> {
    // Parse with spydecy
    let hir: CHIR = parse_c_file(path)?;

    // Use decy's existing codegen
    let rust_code = decy_codegen::generate(&hir)?;

    Ok(rust_code)
}
```

### deypler with spydecy

```rust
// deypler/src/transpile.rs
use spydecy_python::parse_python;
use spydecy_hir::python::PythonHIR;

pub fn transpile_python_to_rust(source: &str) -> anyhow::Result<String> {
    // Parse with spydecy
    let hir: PythonHIR = parse_python(source, "input.py")?;

    // Use deypler's codegen
    let rust_code = deypler_codegen::generate(&hir)?;

    Ok(rust_code)
}
```

---

## 📖 Full Documentation

See:
- `RELEASE-v0.1.0.md` - Complete release notes
- `CHANGELOG.md` - Version history
- `README.md` - Full documentation
- `docs/specification/` - Architecture details

---

## ⚠️ Known Limitations

**v0.1.0 is a foundation release:**
- Unification tests incomplete (Sprint 3 in progress)
- Optimizer stubbed (use your existing ones)
- Codegen stubbed (use your existing ones)
- Type inference not yet implemented

**For production use:**
- Use spydecy parsers + your codegens
- Unification coming in v0.2.0 (2 weeks)
- Full pipeline in v1.0.0 (12 weeks)

---

## 🚀 Ready to Use!

The release is **tagged** and **pushed**:
- Tag: `v0.1.0`
- Branch: `main`
- Tests: 47/47 passing ✅
- Release build: tested ✅

**Start integrating now!** 🎉
