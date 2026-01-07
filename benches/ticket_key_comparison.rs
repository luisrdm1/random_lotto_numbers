//! Benchmark comparing HashSet<Ticket> vs HashSet<TicketKey> for unique ticket generation.
//!
//! Run with: `cargo bench --bench ticket_key_comparison`

use criterion::{Criterion, criterion_group, criterion_main};
use lotto_quick_pick::newtypes::{BallRange, GameCount, PickCount};
use lotto_quick_pick::ticket::{
    generate_unique_tickets, generate_unique_tickets_with_ticket_hashset,
};
use std::hint::black_box;

fn bench_unique_tickets_mega_sena_small(c: &mut Criterion) {
    c.bench_function("ticketkey_mega_sena_10_games", |b| {
        let range = BallRange::mega_sena();
        let pick = PickCount::new(6, &range).unwrap();
        let count = GameCount::new(10).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_unique_tickets(
                black_box(&mut rng),
                black_box(&range),
                black_box(&pick),
                black_box(&count),
            )
        });
    });
}

fn bench_hashset_ticket_mega_sena_small(c: &mut Criterion) {
    c.bench_function("hashset_ticket_mega_sena_10_games", |b| {
        let range = BallRange::mega_sena();
        let pick = PickCount::new(6, &range).unwrap();
        let count = GameCount::new(10).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_unique_tickets_with_ticket_hashset(
                black_box(&mut rng),
                black_box(&range),
                black_box(&pick),
                black_box(&count),
            )
        });
    });
}

fn bench_unique_tickets_mega_sena_medium(c: &mut Criterion) {
    c.bench_function("ticketkey_mega_sena_100_games", |b| {
        let range = BallRange::mega_sena();
        let pick = PickCount::new(6, &range).unwrap();
        let count = GameCount::new(100).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_unique_tickets(
                black_box(&mut rng),
                black_box(&range),
                black_box(&pick),
                black_box(&count),
            )
        });
    });
}

fn bench_hashset_ticket_mega_sena_medium(c: &mut Criterion) {
    c.bench_function("hashset_ticket_mega_sena_100_games", |b| {
        let range = BallRange::mega_sena();
        let pick = PickCount::new(6, &range).unwrap();
        let count = GameCount::new(100).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_unique_tickets_with_ticket_hashset(
                black_box(&mut rng),
                black_box(&range),
                black_box(&pick),
                black_box(&count),
            )
        });
    });
}

fn bench_unique_tickets_mega_sena_large(c: &mut Criterion) {
    c.bench_function("ticketkey_mega_sena_1000_games", |b| {
        let range = BallRange::mega_sena();
        let pick = PickCount::new(6, &range).unwrap();
        let count = GameCount::new(1000).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_unique_tickets(
                black_box(&mut rng),
                black_box(&range),
                black_box(&pick),
                black_box(&count),
            )
        });
    });
}

fn bench_hashset_ticket_mega_sena_large(c: &mut Criterion) {
    c.bench_function("hashset_ticket_mega_sena_1000_games", |b| {
        let range = BallRange::mega_sena();
        let pick = PickCount::new(6, &range).unwrap();
        let count = GameCount::new(1000).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_unique_tickets_with_ticket_hashset(
                black_box(&mut rng),
                black_box(&range),
                black_box(&pick),
                black_box(&count),
            )
        });
    });
}

fn bench_unique_tickets_lotomania(c: &mut Criterion) {
    c.bench_function("ticketkey_lotomania_100_games", |b| {
        let range = BallRange::lotomania();
        let pick = PickCount::new(50, &range).unwrap();
        let count = GameCount::new(100).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_unique_tickets(
                black_box(&mut rng),
                black_box(&range),
                black_box(&pick),
                black_box(&count),
            )
        });
    });
}

fn bench_hashset_ticket_lotomania(c: &mut Criterion) {
    c.bench_function("hashset_ticket_lotomania_100_games", |b| {
        let range = BallRange::lotomania();
        let pick = PickCount::new(50, &range).unwrap();
        let count = GameCount::new(100).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_unique_tickets_with_ticket_hashset(
                black_box(&mut rng),
                black_box(&range),
                black_box(&pick),
                black_box(&count),
            )
        });
    });
}

criterion_group!(
    benches,
    bench_unique_tickets_mega_sena_small,
    bench_hashset_ticket_mega_sena_small,
    bench_unique_tickets_mega_sena_medium,
    bench_hashset_ticket_mega_sena_medium,
    bench_unique_tickets_mega_sena_large,
    bench_hashset_ticket_mega_sena_large,
    bench_unique_tickets_lotomania,
    bench_hashset_ticket_lotomania,
);
criterion_main!(benches);
