use criterion::{black_box, criterion_group, criterion_main, Criterion};
use metaforge::Options;

pub fn build_site_benchmark(c: &mut Criterion) {
    let dir = std::path::PathBuf::from("files/site")
        .canonicalize()
        .unwrap();

    let mut opts = Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");
    opts.clean = true;

    todo!("implement with DirNode")
    // c.bench_function("build test site", |b| {
    //     b.iter(|| build_site(black_box(&opts)))
    // });
}

criterion_group!(benches, build_site_benchmark);
criterion_main!(benches);
