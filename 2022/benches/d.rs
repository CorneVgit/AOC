use aoc2022::d::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("d1", |b| b.iter(d1::d1));
    c.bench_function("d2", |b| b.iter(d2::d2));
    c.bench_function("d3", |b| b.iter(d3::d3));
    c.bench_function("d4", |b| b.iter(d4::d4));
    c.bench_function("d5", |b| b.iter(d5::d5));
    c.bench_function("d6_slow", |b| b.iter(d6::d6_slow));
    c.bench_function("d6_fast", |b| b.iter(d6::d6_fast));
    c.bench_function("d7", |b| b.iter(d7::d7));
    c.bench_function("d8", |b| b.iter(d8::d8));
    c.bench_function("d9", |b| b.iter(d9::d9));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
