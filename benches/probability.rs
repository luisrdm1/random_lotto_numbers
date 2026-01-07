use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use lotto_quick_pick::probability::{calculate_probability, combination};
use std::hint::black_box;

fn bench_combination(c: &mut Criterion) {
    let mut group = c.benchmark_group("combination");
    
    let scenarios = vec![
        ("C(60, 6) - Mega-Sena", 60, 6),
        ("C(100, 10) - Large", 100, 10),
        ("C(100, 50) - Half", 100, 50),
        ("C(20, 10) - Medium", 20, 10),
        ("C(10, 5) - Small", 10, 5),
    ];

    for (name, n, k) in scenarios {
        group.bench_with_input(BenchmarkId::from_parameter(name), &(n, k), 
            |b, &(n, k)| {
                b.iter(|| {
                    combination(black_box(n), black_box(k)).unwrap()
                });
            }
        );
    }
    
    group.finish();
}

fn bench_probability(c: &mut Criterion) {
    let mut group = c.benchmark_group("probability");
    
    let scenarios = vec![
        ("Mega-Sena match 6", 60, 6, 6),
        ("Mega-Sena match 5", 60, 6, 5),
        ("Mega-Sena match 4", 60, 6, 4),
        ("Lotomania match 20", 100, 50, 20),
        ("Lotomania match 15", 100, 50, 15),
    ];

    for (name, total, pick, match_count) in scenarios {
        group.bench_with_input(BenchmarkId::from_parameter(name), 
            &(total, pick, match_count), 
            |b, &(t, p, m)| {
                b.iter(|| {
                    calculate_probability(black_box(t), black_box(p), black_box(m)).unwrap()
                });
            }
        );
    }
    
    group.finish();
}

fn bench_combination_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("combination_edge_cases");
    
    // Test symmetry optimization: C(n, k) = C(n, n-k)
    group.bench_function("C(100, 5) - small k", |b| {
        b.iter(|| {
            combination(black_box(100), black_box(5)).unwrap()
        });
    });
    
    group.bench_function("C(100, 95) - large k (uses symmetry)", |b| {
        b.iter(|| {
            combination(black_box(100), black_box(95)).unwrap()
        });
    });
    
    // Test edge cases
    group.bench_function("C(1000, 1) - trivial", |b| {
        b.iter(|| {
            combination(black_box(1000), black_box(1)).unwrap()
        });
    });
    
    group.bench_function("C(100, 0) - zero", |b| {
        b.iter(|| {
            combination(black_box(100), black_box(0)).unwrap()
        });
    });
    
    group.finish();
}

criterion_group!(benches, bench_combination, bench_probability, bench_combination_edge_cases);
criterion_main!(benches);
