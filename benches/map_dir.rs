use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn map_dir_benchmark(c: &mut Criterion) {
    let dir = std::path::PathBuf::from("files/site")
        .canonicalize()
        .unwrap();

    let mut opts = metaforge::Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");
    opts.clean = true;

    c.bench_function("build benchmark file", |b| {
        b.iter(|| {
            let mut dir =
                metaforge::DirNode::build(black_box(opts.source.clone()), black_box(&opts))
                    .unwrap();
            let tmp = metaforge::MetaFile::new(black_box(&opts));
            dir.map(&tmp).unwrap();
        })
    });
}

criterion_group!(benches, map_dir_benchmark);
criterion_main!(benches);
