# Quick Start

This guide will walk you through your first Spydecy unification in 5 minutes.

## Installation

```bash
cargo install spydecy
```

## Your First Unification

Let's demonstrate the **core innovation** of Spydecy: unifying Python `len()` with C `list_length()` into pure Rust `Vec::len()`.

### Using the Optimizer

```rust,ignore
use spydecy_optimizer::{OptimizationPipeline, BoundaryEliminationPass};
use spydecy_hir::unified::UnifiedHIR;
use spydecy_hir::{Language, NodeId, types::Type, metadata::Metadata};
use spydecy_hir::unified::{CrossMapping, UnificationPattern};

fn main() -> anyhow::Result<()> {
    // Create a UnifiedHIR node (normally from parser + unifier)
    let hir = UnifiedHIR::Call {
        id: NodeId::new(1),
        target_language: Language::Python,
        callee: "Vec::len".to_owned(),
        args: vec![],
        inferred_type: Type::Unknown,
        source_language: Language::Python,
        cross_mapping: Some(CrossMapping {
            python_node: None,
            c_node: None,
            pattern: UnificationPattern::LenPattern,
            boundary_eliminated: false,
        }),
        meta: Metadata::new(),
    };

    // Run optimization pipeline
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(hir)?;

    println!("✅ Optimization complete!");
    Ok(())
}
```

### What Just Happened?

1. **Unified HIR Created**: Represents Python `len()` + C `list_length()`
2. **Optimization Pipeline**: Ran boundary elimination pass
3. **Pure Rust Output**: Generates code targeting `Vec::len()` with zero FFI

## Next Steps

- [First Unification](./first-unification.md) - Complete unification example
- [Unification Patterns](./unification-patterns.md) - Learn all 3 core patterns
- [CLI Reference](./cli-reference.md) - Command-line interface guide
