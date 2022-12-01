use criterion::{criterion_group, criterion_main, Criterion};
use aoc2022::d::d1::*;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("d1", |b| b.iter(d1));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
