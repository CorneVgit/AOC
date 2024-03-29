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
    c.bench_function("d10", |b| b.iter(d10::d10));
    c.bench_function("d11", |b| b.iter(d11::d11));
    c.bench_function("d12", |b| b.iter(d12::d12));
    c.bench_function("d13", |b| b.iter(d13::d13));
    c.bench_function("d14", |b| b.iter(d14::d14));
    c.bench_function("d15", |b| b.iter(d15::d15));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
