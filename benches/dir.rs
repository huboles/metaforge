use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn build_dir(c: &mut Criterion) {
    let dir = std::path::PathBuf::from("files/bench_site")
        .canonicalize()
        .unwrap();

    let mut opts = metaforge::Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");
    opts.clean = true;

    c.bench_function("build dir", |b| {
        if opts.build.exists() {
            std::fs::remove_dir_all(&opts.build).expect("clean build dir");
        }

        std::fs::create_dir(&opts.build).expect("create build dir");
        b.iter(|| metaforge::build_dir(black_box(&opts)).unwrap())
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets =  build_dir
}

criterion_main!(benches);
