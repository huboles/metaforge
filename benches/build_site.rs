use criterion::{black_box, criterion_group, criterion_main, Criterion};
use metaforge::{build_site, Options};

pub fn build_site_benchmark(c: &mut Criterion) {
    let dir = std::path::PathBuf::from("files/site")
        .canonicalize()
        .unwrap();

    let opts = Options {
        root: dir.clone(),
        source: dir.join("source"),
        build: dir.join("build"),
        pattern: dir.join("pattern"),
        verbose: 0,
        quiet: false,
        force: false,
        undefined: false,
        clean: true,
    };

    c.bench_function("build test site", |b| {
        b.iter(|| build_site(black_box(&opts)))
    });
}

criterion_group!(benches, build_site_benchmark);
criterion_main!(benches);
