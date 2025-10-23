//! Optimizer Performance Benchmarks
//!
//! Measures the effectiveness of the cross-layer optimizer, specifically:
//! - Boundary elimination pass performance
//! - Pipeline execution overhead
//! - Optimization of different patterns (len, append, dict.get)

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use spydecy_hir::{
    metadata::Metadata,
    types::Type,
    unified::{CrossMapping, UnificationPattern, UnifiedHIR},
    Language, NodeId,
};
use spydecy_optimizer::{BoundaryEliminationPass, OptimizationPipeline, Pass};

/// Create a sample UnifiedHIR for benchmarking
fn create_sample_hir(pattern: UnificationPattern) -> UnifiedHIR {
    let callee = match pattern {
        UnificationPattern::LenPattern => "Vec::len",
        UnificationPattern::AppendPattern => "Vec::push",
        UnificationPattern::DictGetPattern => "HashMap::get",
        UnificationPattern::ReversePattern => "Vec::reverse",
        UnificationPattern::ClearPattern => "Vec::clear",
        UnificationPattern::Custom => "custom",
    };

    UnifiedHIR::Call {
        id: NodeId::new(1),
        target_language: Language::Python,
        callee: callee.to_owned(),
        args: vec![],
        inferred_type: Type::Unknown,
        source_language: Language::Python,
        cross_mapping: Some(CrossMapping {
            python_node: None,
            c_node: None,
            pattern,
            boundary_eliminated: false,
        }),
        meta: Metadata::new(),
    }
}

/// Benchmark boundary elimination pass on different patterns
fn benchmark_boundary_elimination(c: &mut Criterion) {
    let mut group = c.benchmark_group("boundary_elimination");

    for pattern in [
        UnificationPattern::LenPattern,
        UnificationPattern::AppendPattern,
        UnificationPattern::DictGetPattern,
    ] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{pattern:?}")),
            &pattern,
            |b, &pattern| {
                let pass = BoundaryEliminationPass::new();
                b.iter(|| {
                    let hir = create_sample_hir(pattern);
                    black_box(pass.run(hir).expect("Pass should succeed"))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark the full optimization pipeline
fn benchmark_optimization_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_pipeline");

    // Single pass pipeline
    group.bench_function("single_pass", |b| {
        let pipeline = OptimizationPipeline::standard();
        b.iter(|| {
            let hir = create_sample_hir(UnificationPattern::LenPattern);
            black_box(pipeline.run(hir).expect("Pipeline should succeed"))
        });
    });

    // Multiple passes pipeline (idempotent)
    group.bench_function("multi_pass_idempotent", |b| {
        let mut pipeline = OptimizationPipeline::new();
        pipeline.add_pass(Box::new(BoundaryEliminationPass::new()));
        pipeline.add_pass(Box::new(BoundaryEliminationPass::new()));
        pipeline.add_pass(Box::new(BoundaryEliminationPass::new()));

        b.iter(|| {
            let hir = create_sample_hir(UnificationPattern::LenPattern);
            black_box(pipeline.run(hir).expect("Pipeline should succeed"))
        });
    });

    group.finish();
}

/// Benchmark HIR cloning overhead (used in optimization)
fn benchmark_hir_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("hir_operations");

    group.bench_function("clone_simple_call", |b| {
        let hir = create_sample_hir(UnificationPattern::LenPattern);
        b.iter(|| black_box(hir.clone()));
    });

    // Benchmark with nested HIR (more complex)
    group.bench_function("clone_nested_call", |b| {
        let inner = create_sample_hir(UnificationPattern::LenPattern);
        let hir = UnifiedHIR::Call {
            id: NodeId::new(2),
            target_language: Language::Rust,
            callee: "outer".to_owned(),
            args: vec![inner],
            inferred_type: Type::Unknown,
            source_language: Language::Python,
            cross_mapping: None,
            meta: Metadata::new(),
        };
        b.iter(|| black_box(hir.clone()));
    });

    group.finish();
}

/// Benchmark pattern matching in optimizer
fn benchmark_pattern_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_detection");

    for pattern in [
        UnificationPattern::LenPattern,
        UnificationPattern::AppendPattern,
        UnificationPattern::DictGetPattern,
    ] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{pattern:?}")),
            &pattern,
            |b, &pattern| {
                let hir = create_sample_hir(pattern);
                b.iter(|| {
                    // Simulate pattern detection (checking cross_mapping)
                    if let UnifiedHIR::Call { cross_mapping, .. } = &hir {
                        black_box(cross_mapping.as_ref().map(|m| m.pattern))
                    } else {
                        None
                    }
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_boundary_elimination,
    benchmark_optimization_pipeline,
    benchmark_hir_clone,
    benchmark_pattern_detection
);
criterion_main!(benches);
