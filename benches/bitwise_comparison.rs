//! Benchmark comparing HashSet vs bitwise strategies for ticket generation.
//!
//! Run with: `cargo bench --bench bitwise_comparison`

use criterion::{Criterion, criterion_group, criterion_main};
use lotto_quick_pick::newtypes::{BallNumber, BallRange, PickCount};
use lotto_quick_pick::ticket::generate_ticket;
use lotto_quick_pick::ticket_bitwise::generate_ticketkey_bitwise;
use std::hint::black_box;

fn bench_hashset_mega_sena(c: &mut Criterion) {
    c.bench_function("hashset_mega_sena_6_picks", |b| {
        let range = BallRange::mega_sena();
        let count = PickCount::new(6, &range).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_ticket(black_box(&mut rng), black_box(&range), black_box(&count))
        });
    });
}

fn bench_bitwise_mega_sena(c: &mut Criterion) {
    c.bench_function("bitwise_mega_sena_6_picks", |b| {
        let range = BallRange::mega_sena();
        let count = PickCount::new(6, &range).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_ticketkey_bitwise(black_box(&range), black_box(count), black_box(&mut rng))
        });
    });
}

fn bench_hashset_lotomania(c: &mut Criterion) {
    c.bench_function("hashset_lotomania_50_picks", |b| {
        let range = BallRange::lotomania();
        let count = PickCount::new(50, &range).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_ticket(black_box(&mut rng), black_box(&range), black_box(&count))
        });
    });
}

fn bench_bitwise_lotomania(c: &mut Criterion) {
    c.bench_function("bitwise_lotomania_50_picks", |b| {
        let range = BallRange::lotomania();
        let count = PickCount::new(50, &range).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_ticketkey_bitwise(black_box(&range), black_box(count), black_box(&mut rng))
        });
    });
}

fn bench_hashset_large_range(c: &mut Criterion) {
    c.bench_function("hashset_range200_10_picks", |b| {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(200)).unwrap();
        let count = PickCount::new(10, &range).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_ticket(black_box(&mut rng), black_box(&range), black_box(&count))
        });
    });
}

fn bench_bitwise_large_range(c: &mut Criterion) {
    c.bench_function("bitwise_range200_10_picks", |b| {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(200)).unwrap();
        let count = PickCount::new(10, &range).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_ticketkey_bitwise(black_box(&range), black_box(count), black_box(&mut rng))
        });
    });
}

fn bench_hashset_small_picks(c: &mut Criterion) {
    c.bench_function("hashset_mega_sena_3_picks", |b| {
        let range = BallRange::mega_sena();
        let count = PickCount::new(3, &range).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_ticket(black_box(&mut rng), black_box(&range), black_box(&count))
        });
    });
}

fn bench_bitwise_small_picks(c: &mut Criterion) {
    c.bench_function("bitwise_mega_sena_3_picks", |b| {
        let range = BallRange::mega_sena();
        let count = PickCount::new(3, &range).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_ticketkey_bitwise(black_box(&range), black_box(count), black_box(&mut rng))
        });
    });
}

fn bench_hashset_powerball(c: &mut Criterion) {
    c.bench_function("hashset_powerball_5_picks", |b| {
        let range = BallRange::powerball();
        let count = PickCount::new(5, &range).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_ticket(black_box(&mut rng), black_box(&range), black_box(&count))
        });
    });
}

fn bench_bitwise_powerball(c: &mut Criterion) {
    c.bench_function("bitwise_powerball_5_picks", |b| {
        let range = BallRange::powerball();
        let count = PickCount::new(5, &range).unwrap();

        b.iter(|| {
            let mut rng = rand::rng();
            generate_ticketkey_bitwise(black_box(&range), black_box(count), black_box(&mut rng))
        });
    });
}

criterion_group!(
    benches,
    bench_hashset_mega_sena,
    bench_bitwise_mega_sena,
    bench_hashset_lotomania,
    bench_bitwise_lotomania,
    bench_hashset_large_range,
    bench_bitwise_large_range,
    bench_hashset_small_picks,
    bench_bitwise_small_picks,
    bench_hashset_powerball,
    bench_bitwise_powerball,
);

criterion_main!(benches);
