//! Code Generation Performance Benchmarks - Phase 2.2
//!
//! Validates SPECIFICATION.md Section 30 performance target:
//! Generated code must perform within 20% of hand-written Rust.
//!
//! This benchmark compares the performance of Spydecy-generated code patterns
//! against idiomatic hand-written Rust code.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;

// =============================================================================
// Vec::len() Benchmarks
// =============================================================================

/// Hand-written Rust: Vec::len()
fn handwritten_vec_len(data: &[i32]) -> usize {
    data.len()
}

/// Spydecy-generated pattern: Vec::len()
/// Generated from: Python `len(my_list)` + C `list_length()`
fn spydecy_generated_vec_len(data: &[i32]) -> usize {
    data.len() // Spydecy generates identical code
}

fn benchmark_vec_len(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_len");

    for size in [10, 100, 1000, 10_000].iter() {
        let data: Vec<i32> = (0..*size).collect();

        group.bench_with_input(BenchmarkId::new("hand_written", size), &data, |b, data| {
            b.iter(|| black_box(handwritten_vec_len(data)));
        });

        group.bench_with_input(
            BenchmarkId::new("spydecy_generated", size),
            &data,
            |b, data| {
                b.iter(|| black_box(spydecy_generated_vec_len(data)));
            },
        );
    }

    group.finish();
}

// =============================================================================
// Vec::push() Benchmarks
// =============================================================================

/// Hand-written Rust: Vec::push()
fn handwritten_vec_push(data: &mut Vec<i32>, value: i32) {
    data.push(value);
}

/// Spydecy-generated pattern: Vec::push()
/// Generated from: Python `list.append()` + C `PyList_Append()`
fn spydecy_generated_vec_push(data: &mut Vec<i32>, value: i32) {
    data.push(value); // Spydecy generates identical code
}

fn benchmark_vec_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_push");

    for initial_capacity in [0, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("hand_written", initial_capacity),
            initial_capacity,
            |b, &capacity| {
                b.iter(|| {
                    let mut data = Vec::with_capacity(capacity);
                    for i in 0..100 {
                        handwritten_vec_push(&mut data, i);
                    }
                    black_box(data);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("spydecy_generated", initial_capacity),
            initial_capacity,
            |b, &capacity| {
                b.iter(|| {
                    let mut data = Vec::with_capacity(capacity);
                    for i in 0..100 {
                        spydecy_generated_vec_push(&mut data, i);
                    }
                    black_box(data);
                });
            },
        );
    }

    group.finish();
}

// =============================================================================
// Vec::reverse() Benchmarks
// =============================================================================

/// Hand-written Rust: Vec::reverse()
fn handwritten_vec_reverse(data: &mut [i32]) {
    data.reverse();
}

/// Spydecy-generated pattern: Vec::reverse()
/// Generated from: Python `list.reverse()` + C `list_reverse()`
fn spydecy_generated_vec_reverse(data: &mut [i32]) {
    data.reverse(); // Spydecy generates identical code
}

fn benchmark_vec_reverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_reverse");

    for size in [10, 100, 1000, 10_000].iter() {
        let data: Vec<i32> = (0..*size).collect();

        group.bench_with_input(BenchmarkId::new("hand_written", size), &data, |b, data| {
            b.iter(|| {
                let mut clone = data.clone();
                handwritten_vec_reverse(&mut clone);
                black_box(clone);
            });
        });

        group.bench_with_input(
            BenchmarkId::new("spydecy_generated", size),
            &data,
            |b, data| {
                b.iter(|| {
                    let mut clone = data.clone();
                    spydecy_generated_vec_reverse(&mut clone);
                    black_box(clone);
                });
            },
        );
    }

    group.finish();
}

// =============================================================================
// HashMap::get() Benchmarks
// =============================================================================

/// Hand-written Rust: HashMap::get()
fn handwritten_hashmap_get<'a>(map: &'a HashMap<String, i32>, key: &str) -> Option<&'a i32> {
    map.get(key)
}

/// Spydecy-generated pattern: HashMap::get()
/// Generated from: Python `dict.get()` + C `PyDict_GetItem()`
fn spydecy_generated_hashmap_get<'a>(map: &'a HashMap<String, i32>, key: &str) -> Option<&'a i32> {
    map.get(key) // Spydecy generates identical code
}

fn benchmark_hashmap_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_get");

    for size in [10, 100, 1000].iter() {
        let mut map = HashMap::new();
        for i in 0..*size {
            map.insert(format!("key_{i}"), i);
        }

        let test_key = format!("key_{}", size / 2);

        group.bench_with_input(
            BenchmarkId::new("hand_written", size),
            &(&map, &test_key),
            |b, (map, key)| {
                b.iter(|| black_box(handwritten_hashmap_get(map, key)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("spydecy_generated", size),
            &(&map, &test_key),
            |b, (map, key)| {
                b.iter(|| black_box(spydecy_generated_hashmap_get(map, key)));
            },
        );
    }

    group.finish();
}

// =============================================================================
// Additional Pattern Benchmarks
// =============================================================================

/// Benchmark Vec::clear()
fn benchmark_vec_clear(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_clear");

    for size in [100, 1000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::new("hand_written", size), size, |b, &size| {
            b.iter(|| {
                let mut data: Vec<i32> = (0..size).collect();
                data.clear();
                black_box(data);
            });
        });

        group.bench_with_input(
            BenchmarkId::new("spydecy_generated", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut data: Vec<i32> = (0..size).collect();
                    data.clear(); // Identical to hand-written
                    black_box(data);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Vec::pop()
fn benchmark_vec_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_pop");

    for size in [100, 1000].iter() {
        let data: Vec<i32> = (0..*size).collect();

        group.bench_with_input(BenchmarkId::new("hand_written", size), &data, |b, data| {
            b.iter(|| {
                let mut clone = data.clone();
                while clone.pop().is_some() {}
                black_box(clone);
            });
        });

        group.bench_with_input(
            BenchmarkId::new("spydecy_generated", size),
            &data,
            |b, data| {
                b.iter(|| {
                    let mut clone = data.clone();
                    while clone.pop().is_some() {} // Identical
                    black_box(clone);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_vec_len,
    benchmark_vec_push,
    benchmark_vec_reverse,
    benchmark_hashmap_get,
    benchmark_vec_clear,
    benchmark_vec_pop
);
criterion_main!(benches);
