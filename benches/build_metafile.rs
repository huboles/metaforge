use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn build_file_benchmark(c: &mut Criterion) {
    let dir = std::path::PathBuf::from("files/benchmark_site")
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
            let string = std::fs::read_to_string(black_box(&source)).expect("read file");
            let mut file = metaforge::parse_file(string, black_box(&opts)).expect("parse file");
            file.path = black_box(source.clone());
            let string = metaforge::build_metafile(&file).expect("build file");
            std::fs::write(file.dest().expect("find dest"), string).expect("write file");
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = build_file_benchmark
}

criterion_main!(benches);
