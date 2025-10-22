# Introduction

**Spydecy** is a self-hosted Python/C-to-Rust compiler-debugger with introspective debugging capabilities.

## The Core Innovation

Spydecy recognizes patterns between Python and C implementations, unifying them into pure Rust code with **zero FFI** and **zero unsafe code**.

```text
Python len(x)  ←─────┐
                      ├──→ Unified HIR ──→ Rust x.len()
C list_length() ──────┘
```

## Example: Unifying `len()` and `list_length()`

Given these implementations:

**Python:**
```python
def my_len(x):
    return len(x)
```

**C:**
```c
static Py_ssize_t
list_length(PyListObject *self) {
    return Py_SIZE(self);
}
```

Spydecy **unifies** them into pure Rust:

```rust,no_run
// Generated Rust code - zero FFI, zero unsafe
fn my_len<T>(x: &Vec<T>) -> usize {
    x.len()
}
```

## Key Features

- ✅ **3 Core Patterns**: len, append, dict.get
- ✅ **Cross-Layer Optimizer**: Eliminates Python→C boundaries
- ✅ **Introspective Debugger**: Visualize ASTs with `spydecy debug`
- ✅ **EXTREME TDD**: 72/72 tests passing, zero tolerance for technical debt
- ✅ **Production Ready**: Published to crates.io

## Status

- **Version**: 0.2.0
- **Tests**: 72/72 passing (100%)
- **Quality**: PMAT complexity < 10, zero SATD
- **Phase**: Sprint 4 (Optimizer working!)

## Quick Example

```rust,ignore
use spydecy_optimizer::{OptimizationPipeline};
use spydecy_hir::unified::UnifiedHIR;

fn optimize(hir: UnifiedHIR) -> anyhow::Result<UnifiedHIR> {
    let pipeline = OptimizationPipeline::standard();
    pipeline.run(hir)
}
```

## Next Steps

- [Installation](./installation.md) - Get started with Spydecy
- [Quick Start](./quick-start.md) - Your first unification
- [Unification Patterns](./unification-patterns.md) - Deep dive into patterns
