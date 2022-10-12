use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../src/filter.rs"]
mod filter;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("reduced-sample-size");
    group
        .sample_size(15)
        .measurement_time(std::time::Duration::from_secs(45));

    group.bench_function("Filter", |b| {
        b.iter(|| filter::filter_file("files/infile.txt", "files/cats.txt", 3, "files/outfile.txt"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
