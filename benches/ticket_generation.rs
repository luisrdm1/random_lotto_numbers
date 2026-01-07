use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use lotto_quick_pick::{
    newtypes::{BallNumber, BallRange, GameCount, PickCount},
    ticket::{generate_ticket, generate_unique_tickets},
};
use std::hint::black_box;

fn bench_single_ticket(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_ticket");

    let scenarios = vec![
        ("Mega-Sena (6 from 60)", 1, 60, 6),
        ("Lotomania (50 from 100)", 0, 99, 50),
        ("Small (3 from 10)", 1, 10, 3),
        ("Large (10 from 100)", 1, 100, 10),
    ];

    for (name, start, end, pick) in scenarios {
        let range = BallRange::new(BallNumber::new(start), BallNumber::new(end)).unwrap();
        let pick_count = PickCount::new(pick, &range).unwrap();

        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &(range, pick_count),
            |b, (range, pick)| {
                let mut rng = rand::rng();
                b.iter(|| generate_ticket(black_box(&mut rng), black_box(range), black_box(pick)));
            },
        );
    }

    group.finish();
}

fn bench_unique_tickets(c: &mut Criterion) {
    let mut group = c.benchmark_group("unique_tickets");

    let scenarios = vec![
        ("10 Mega-Sena tickets", 1, 60, 6, 10),
        ("100 Mega-Sena tickets", 1, 60, 6, 100),
        ("10 small tickets", 1, 20, 5, 10),
        ("50 medium tickets", 1, 50, 10, 50),
    ];

    for (name, start, end, pick, count) in scenarios {
        let range = BallRange::new(BallNumber::new(start), BallNumber::new(end)).unwrap();
        let pick_count = PickCount::new(pick, &range).unwrap();
        let game_count = GameCount::new(count).unwrap();

        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &(range, pick_count, game_count),
            |b, (range, pick, count)| {
                let mut rng = rand::rng();
                b.iter(|| {
                    generate_unique_tickets(
                        black_box(&mut rng),
                        black_box(range),
                        black_box(pick),
                        black_box(count),
                    )
                    .unwrap()
                });
            },
        );
    }

    group.finish();
}

fn bench_ticket_strategies(c: &mut Criterion) {
    let mut group = c.benchmark_group("ticket_strategies");

    // Test insertion strategy (picking few from many)
    let range_insertion = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
    let pick_insertion = PickCount::new(6, &range_insertion).unwrap();

    group.bench_function("insertion_strategy_6_from_60", |b| {
        let mut rng = rand::rng();
        b.iter(|| {
            generate_ticket(
                black_box(&mut rng),
                black_box(&range_insertion),
                black_box(&pick_insertion),
            )
        });
    });

    // Test exclusion strategy (picking many from few more)
    let range_exclusion = BallRange::new(BallNumber::new(0), BallNumber::new(99)).unwrap();
    let pick_exclusion = PickCount::new(50, &range_exclusion).unwrap();

    group.bench_function("exclusion_strategy_50_from_100", |b| {
        let mut rng = rand::rng();
        b.iter(|| {
            generate_ticket(
                black_box(&mut rng),
                black_box(&range_exclusion),
                black_box(&pick_exclusion),
            )
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_single_ticket,
    bench_unique_tickets,
    bench_ticket_strategies
);
criterion_main!(benches);
