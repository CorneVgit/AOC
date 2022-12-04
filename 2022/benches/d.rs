use aoc2022::d::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("d1", |b| b.iter(d1::d1));
    c.bench_function("d2", |b| b.iter(d2::d2));
    c.bench_function("d3", |b| b.iter(d3::d3));
    c.bench_function("d4", |b| b.iter(d4::d4));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
