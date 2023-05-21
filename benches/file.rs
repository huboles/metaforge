use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn build_file(c: &mut Criterion) {
    let dir = std::path::PathBuf::from("files/bench_site")
        .canonicalize()
        .unwrap();

    let mut opts = metaforge::Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");

    let mut file: metaforge::MetaFile = metaforge::MetaFile::new(&opts);
    let path = opts.source.join("bench.meta");

    c.bench_function("build file", |b| {
        b.iter(|| {
            file = metaforge::MetaFile::build(black_box(path.clone()), black_box(&opts)).unwrap();
            file.construct().unwrap();
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets =  build_file
}

criterion_main!(benches);
