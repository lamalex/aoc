use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use aoc_2025;

fn day2_bench(c: &mut Criterion) {
    let input = include_str!("../data/day2.txt");
    c.bench_function("day2 part 1", |b| {
        b.iter(|| aoc_2025::day2::part1::sum_invalid(black_box(input)))
    });

    c.bench_function("day2 part 2", |b| {
        b.iter(|| aoc_2025::day2::part2::sum_invalid(black_box(input)))
    });
}

criterion_group!(benches, day2_bench);
criterion_main!(benches);
