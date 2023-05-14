use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn build_file_benchmark(c: &mut Criterion) {
    let dir = std::path::PathBuf::from("files/site")
        .canonicalize()
        .unwrap();

    let mut opts = metaforge::Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");
    opts.clean = true;

    let source = opts.source.join("benchmark.meta");

    c.bench_function("build benchmark file", |b| {
        b.iter(|| {
            let string = std::fs::read_to_string(black_box(&source)).unwrap();
            let file = metaforge::parse_file(string, black_box(&opts)).unwrap();
            let mut path = opts
                .build
                .join(source.strip_prefix(black_box(&opts.source)).unwrap());
            path.set_extension("html");
            std::fs::write(path, metaforge::build_metafile(&file).unwrap()).unwrap();
        })
    });
}

criterion_group!(benches, build_file_benchmark);
criterion_main!(benches);
