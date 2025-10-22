//! Cross-Layer Optimizer - Sprint 4
//!
//! This module implements cross-layer optimization passes for the Unified HIR.
//! The optimizer eliminates Python→C boundaries and performs optimizations
//! that are only possible with unified Python+C knowledge.
//!
//! # Architecture
//!
//! The optimizer is built around a **pass-based architecture**:
//!
//! ```text
//! UnifiedHIR → Pass 1 → Pass 2 → Pass N → Optimized HIR
//! ```
//!
//! # Core Passes
//!
//! - **Boundary Elimination**: Removes Python→C FFI boundaries
//! - **Dead Code Elimination**: Removes unreachable code
//! - **Inlining** (future): Inlines small functions
//! - **Constant Folding** (future): Evaluates constants at compile time
//!
//! # Usage
//!
//! ```no_run
//! use spydecy_optimizer::{OptimizationPipeline, BoundaryEliminationPass};
//! use spydecy_hir::unified::UnifiedHIR;
//!
//! # fn example(unified_hir: UnifiedHIR) -> anyhow::Result<()> {
//! let mut pipeline = OptimizationPipeline::new();
//! pipeline.add_pass(Box::new(BoundaryEliminationPass::new()));
//!
//! let optimized = pipeline.run(unified_hir)?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

use anyhow::Result;
use spydecy_hir::unified::UnifiedHIR;

/// Optimization pass trait
///
/// All optimization passes implement this trait.
pub trait Pass: Send + Sync {
    /// Get the name of this pass
    fn name(&self) -> &'static str;

    /// Run the optimization pass on the HIR
    ///
    /// # Errors
    ///
    /// Returns an error if the optimization pass fails
    fn run(&self, hir: UnifiedHIR) -> Result<UnifiedHIR>;
}

/// Boundary elimination pass
///
/// Eliminates Python→C FFI boundaries by converting cross-language
/// calls into pure Rust calls.
///
/// This is the **core optimization** validated by Sprint 0.
pub struct BoundaryEliminationPass;

impl BoundaryEliminationPass {
    /// Create a new boundary elimination pass
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for BoundaryEliminationPass {
    fn default() -> Self {
        Self::new()
    }
}

impl Pass for BoundaryEliminationPass {
    fn name(&self) -> &'static str {
        "BoundaryElimination"
    }

    fn run(&self, hir: UnifiedHIR) -> Result<UnifiedHIR> {
        // Use the eliminate_boundary method already implemented in UnifiedHIR
        Ok(hir.eliminate_boundary())
    }
}

/// Optimization pipeline
///
/// Orchestrates running multiple optimization passes in sequence.
pub struct OptimizationPipeline {
    passes: Vec<Box<dyn Pass>>,
}

impl OptimizationPipeline {
    /// Create a new empty optimization pipeline
    #[must_use]
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    /// Create a pipeline with standard optimization passes
    #[must_use]
    pub fn standard() -> Self {
        let mut pipeline = Self::new();
        pipeline.add_pass(Box::new(BoundaryEliminationPass::new()));
        pipeline
    }

    /// Add a pass to the pipeline
    pub fn add_pass(&mut self, pass: Box<dyn Pass>) {
        self.passes.push(pass);
    }

    /// Run all passes in the pipeline
    ///
    /// # Errors
    ///
    /// Returns an error if any pass fails
    pub fn run(&self, mut hir: UnifiedHIR) -> Result<UnifiedHIR> {
        for pass in &self.passes {
            hir = pass.run(hir)?;
        }
        Ok(hir)
    }

    /// Get the number of passes in the pipeline
    #[must_use]
    pub fn pass_count(&self) -> usize {
        self.passes.len()
    }
}

impl Default for OptimizationPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use spydecy_hir::{
        metadata::Metadata,
        types::Type,
        unified::{CrossMapping, UnificationPattern},
        Language, NodeId,
    };

    #[test]
    fn test_boundary_elimination_pass() {
        // Create a UnifiedHIR with boundary not eliminated
        let hir = UnifiedHIR::Call {
            id: NodeId::new(1),
            target_language: Language::Python,
            callee: "len".to_owned(),
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

        // Run boundary elimination pass
        let pass = BoundaryEliminationPass::new();
        let optimized = pass.run(hir).expect("Pass should succeed");

        // Verify boundary was eliminated
        if let UnifiedHIR::Call { cross_mapping, .. } = optimized {
            let mapping = cross_mapping.expect("Mapping should exist");
            assert!(mapping.boundary_eliminated, "Boundary should be eliminated");
        } else {
            panic!("Expected UnifiedHIR::Call");
        }
    }

    #[test]
    fn test_pipeline_creation() {
        let pipeline = OptimizationPipeline::new();
        assert_eq!(pipeline.pass_count(), 0);
    }

    #[test]
    fn test_pipeline_add_pass() {
        let mut pipeline = OptimizationPipeline::new();
        pipeline.add_pass(Box::new(BoundaryEliminationPass::new()));
        assert_eq!(pipeline.pass_count(), 1);
    }

    #[test]
    fn test_standard_pipeline() {
        let pipeline = OptimizationPipeline::standard();
        assert_eq!(
            pipeline.pass_count(),
            1,
            "Standard pipeline should have 1 pass"
        );
    }

    #[test]
    fn test_pipeline_run() {
        // Create test HIR
        let hir = UnifiedHIR::Call {
            id: NodeId::new(1),
            target_language: Language::Python,
            callee: "len".to_owned(),
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

        // Run pipeline
        let pipeline = OptimizationPipeline::standard();
        let optimized = pipeline.run(hir).expect("Pipeline should succeed");

        // Verify optimization occurred
        if let UnifiedHIR::Call { cross_mapping, .. } = optimized {
            let mapping = cross_mapping.expect("Mapping should exist");
            assert!(
                mapping.boundary_eliminated,
                "Pipeline should eliminate boundary"
            );
        }
    }
}
