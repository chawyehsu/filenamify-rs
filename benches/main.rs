use criterion::{black_box, criterion_group, criterion_main, Criterion};
use filenamify::filenamify;

// basic benchmark
fn benchmark_filenamify_1(c: &mut Criterion) {
    c.bench_function("filenamify", |b| {
        b.iter(|| filenamify(black_box("foo/bar")))
    });
}

// windows reserved characters
fn benchmark_filenamify_2(c: &mut Criterion) {
    c.bench_function("filenamify", |b| b.iter(|| filenamify(black_box(":nul|"))));
}

// outer periods
fn benchmark_filenamify_3(c: &mut Criterion) {
    c.bench_function("filenamify", |b| {
        b.iter(|| filenamify(black_box("../../foo/bar")))
    });
}

criterion_group!(
    benches,
    benchmark_filenamify_1,
    benchmark_filenamify_2,
    benchmark_filenamify_3
);
criterion_main!(benches);
