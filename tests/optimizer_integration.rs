//! Optimizer Integration Tests
//!
//! Tests the complete optimization pipeline with all 3 core patterns.

use anyhow::Result;
use spydecy_c::parse_c;
use spydecy_hir::{
    c::CHIR,
    metadata::Metadata,
    python::PythonHIR,
    types::Type,
    unified::{CrossMapping, UnificationPattern, UnifiedHIR, Unifier},
    Language, NodeId,
};
use spydecy_optimizer::{BoundaryEliminationPass, OptimizationPipeline, Pass};

#[test]
fn test_optimizer_with_len_pattern() -> Result<()> {
    // Create UnifiedHIR for len pattern (simulated)
    let unified = UnifiedHIR::Call {
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

    // Run optimizer
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified)?;

    // Verify boundary eliminated
    if let UnifiedHIR::Call { cross_mapping, .. } = optimized {
        let mapping = cross_mapping.expect("Mapping should exist");
        assert!(
            mapping.boundary_eliminated,
            "Optimizer should eliminate len pattern boundary"
        );
    }

    Ok(())
}

#[test]
fn test_optimizer_with_append_pattern() -> Result<()> {
    // Create UnifiedHIR for append pattern
    let unified = UnifiedHIR::Call {
        id: NodeId::new(1),
        target_language: Language::Rust,
        callee: "Vec::push".to_owned(),
        args: vec![],
        inferred_type: Type::Unknown,
        source_language: Language::Python,
        cross_mapping: Some(CrossMapping {
            python_node: None,
            c_node: None,
            pattern: UnificationPattern::AppendPattern,
            boundary_eliminated: false,
        }),
        meta: Metadata::new(),
    };

    // Run optimizer
    let pass = BoundaryEliminationPass::new();
    let optimized = pass.run(unified)?;

    // Verify boundary eliminated
    if let UnifiedHIR::Call { cross_mapping, .. } = optimized {
        let mapping = cross_mapping.expect("Mapping should exist");
        assert!(
            mapping.boundary_eliminated,
            "Optimizer should eliminate append pattern boundary"
        );
    }

    Ok(())
}

#[test]
fn test_optimizer_with_dict_get_pattern() -> Result<()> {
    // Create UnifiedHIR for dict.get pattern
    let unified = UnifiedHIR::Call {
        id: NodeId::new(1),
        target_language: Language::Rust,
        callee: "HashMap::get".to_owned(),
        args: vec![],
        inferred_type: Type::Unknown,
        source_language: Language::Python,
        cross_mapping: Some(CrossMapping {
            python_node: None,
            c_node: None,
            pattern: UnificationPattern::DictGetPattern,
            boundary_eliminated: false,
        }),
        meta: Metadata::new(),
    };

    // Run optimizer
    let mut pipeline = OptimizationPipeline::new();
    pipeline.add_pass(Box::new(BoundaryEliminationPass::new()));
    let optimized = pipeline.run(unified)?;

    // Verify boundary eliminated
    if let UnifiedHIR::Call { cross_mapping, .. } = optimized {
        let mapping = cross_mapping.expect("Mapping should exist");
        assert!(
            mapping.boundary_eliminated,
            "Optimizer should eliminate dict.get pattern boundary"
        );
    }

    Ok(())
}

#[test]
fn test_full_pipeline_len_pattern() -> Result<()> {
    // Step 1: Create HIRs (simulated for simplicity)
    let python_hir = PythonHIR::Call {
        id: NodeId::new(1),
        callee: Box::new(PythonHIR::Variable {
            id: NodeId::new(2),
            name: "len".to_owned(),
            inferred_type: None,
            meta: Metadata::new(),
        }),
        args: vec![],
        kwargs: vec![],
        inferred_type: None,
        meta: Metadata::new(),
    };

    let c_source = r"
static int
list_length(void *self) {
    return 0;
}
";
    let c_hir_module = parse_c(c_source, "test.c")?;
    let CHIR::TranslationUnit { declarations, .. } = c_hir_module else {
        panic!("Expected TranslationUnit");
    };
    let c_hir = declarations.first().expect("Should have function").clone();

    // Step 2: Unify
    let mut unifier = Unifier::new();
    let unified = unifier.unify(&python_hir, &c_hir)?;

    // Step 3: Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified)?;

    // Step 4: Verify
    if let UnifiedHIR::Call {
        target_language,
        callee,
        cross_mapping,
        ..
    } = optimized
    {
        // Verify it targets Rust
        assert_eq!(target_language, Language::Rust);
        assert_eq!(callee, "Vec::len");

        // Verify boundary was eliminated
        let mapping = cross_mapping.expect("Should have mapping");
        assert!(
            mapping.boundary_eliminated,
            "Full pipeline should eliminate boundary"
        );
    }

    println!("✅ Full pipeline test passed: Parse → Unify → Optimize");

    Ok(())
}

#[test]
fn test_pipeline_with_multiple_passes() -> Result<()> {
    // Create test HIR
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

    // Create pipeline with multiple instances of same pass (should still work)
    let mut pipeline = OptimizationPipeline::new();
    pipeline.add_pass(Box::new(BoundaryEliminationPass::new()));
    pipeline.add_pass(Box::new(BoundaryEliminationPass::new())); // Idempotent

    let optimized = pipeline.run(hir)?;

    // Verify
    if let UnifiedHIR::Call { cross_mapping, .. } = optimized {
        let mapping = cross_mapping.expect("Should have mapping");
        assert!(mapping.boundary_eliminated);
    }

    Ok(())
}
