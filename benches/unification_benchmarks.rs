//! Unification Performance Benchmarks
//!
//! Measures the performance of unifying Python and C HIR into Unified HIR.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use spydecy_hir::{
    c::{StorageClass, CHIR},
    metadata::Metadata,
    python::PythonHIR,
    types::{CType, Type},
    unified::Unifier,
    NodeId, Visibility,
};

/// Create Python len() call HIR
fn create_python_len_call() -> PythonHIR {
    PythonHIR::Call {
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
    }
}

/// Create C list_length() function HIR
fn create_c_list_length() -> CHIR {
    CHIR::Function {
        id: NodeId::new(3),
        name: "list_length".to_owned(),
        return_type: Type::C(CType::SizeT),
        params: vec![],
        body: vec![],
        storage_class: StorageClass::Static,
        visibility: Visibility::Private,
        meta: Metadata::new(),
    }
}

/// Create Python append() call HIR
fn create_python_append_call() -> PythonHIR {
    PythonHIR::Call {
        id: NodeId::new(1),
        callee: Box::new(PythonHIR::Variable {
            id: NodeId::new(2),
            name: "append".to_owned(),
            inferred_type: None,
            meta: Metadata::new(),
        }),
        args: vec![],
        kwargs: vec![],
        inferred_type: None,
        meta: Metadata::new(),
    }
}

/// Create C PyList_Append() function HIR
fn create_c_pylist_append() -> CHIR {
    CHIR::Function {
        id: NodeId::new(3),
        name: "PyList_Append".to_owned(),
        return_type: Type::C(CType::Int),
        params: vec![],
        body: vec![],
        storage_class: StorageClass::Static,
        visibility: Visibility::Private,
        meta: Metadata::new(),
    }
}

/// Create Python dict.get() call HIR
fn create_python_dict_get_call() -> PythonHIR {
    PythonHIR::Call {
        id: NodeId::new(1),
        callee: Box::new(PythonHIR::Variable {
            id: NodeId::new(2),
            name: "get".to_owned(),
            inferred_type: None,
            meta: Metadata::new(),
        }),
        args: vec![],
        kwargs: vec![],
        inferred_type: None,
        meta: Metadata::new(),
    }
}

/// Create C PyDict_GetItem() function HIR
fn create_c_pydict_getitem() -> CHIR {
    CHIR::Function {
        id: NodeId::new(3),
        name: "PyDict_GetItem".to_owned(),
        return_type: Type::C(CType::Pointer(Box::new(CType::Void))),
        params: vec![],
        body: vec![],
        storage_class: StorageClass::Static,
        visibility: Visibility::Private,
        meta: Metadata::new(),
    }
}

/// Benchmark unification of len pattern
fn benchmark_unify_len_pattern(c: &mut Criterion) {
    c.bench_function("unify_len_pattern", |b| {
        b.iter(|| {
            let mut unifier = Unifier::new();
            let python = create_python_len_call();
            let c_hir = create_c_list_length();
            black_box(unifier.unify(&python, &c_hir).expect("Should unify"))
        });
    });
}

/// Benchmark unification of append pattern
fn benchmark_unify_append_pattern(c: &mut Criterion) {
    c.bench_function("unify_append_pattern", |b| {
        b.iter(|| {
            let mut unifier = Unifier::new();
            let python = create_python_append_call();
            let c_hir = create_c_pylist_append();
            black_box(unifier.unify(&python, &c_hir).expect("Should unify"))
        });
    });
}

/// Benchmark unification of dict.get pattern
fn benchmark_unify_dict_get_pattern(c: &mut Criterion) {
    c.bench_function("unify_dict_get_pattern", |b| {
        b.iter(|| {
            let mut unifier = Unifier::new();
            let python = create_python_dict_get_call();
            let c_hir = create_c_pydict_getitem();
            black_box(unifier.unify(&python, &c_hir).expect("Should unify"))
        });
    });
}

/// Benchmark all patterns in a group
fn benchmark_all_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("unification_patterns");

    let patterns = vec![
        ("len", create_python_len_call(), create_c_list_length()),
        (
            "append",
            create_python_append_call(),
            create_c_pylist_append(),
        ),
        (
            "dict_get",
            create_python_dict_get_call(),
            create_c_pydict_getitem(),
        ),
    ];

    for (name, python, c_hir) in patterns {
        group.bench_with_input(BenchmarkId::from_parameter(name), &name, |b, _| {
            let python = python.clone();
            let c_hir = c_hir.clone();
            b.iter(|| {
                let mut unifier = Unifier::new();
                black_box(unifier.unify(&python, &c_hir).expect("Should unify"))
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_unify_len_pattern,
    benchmark_unify_append_pattern,
    benchmark_unify_dict_get_pattern,
    benchmark_all_patterns
);
criterion_main!(benches);
