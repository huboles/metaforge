use eyre::Result;

#[test]
#[ignore = "generates README site"]
fn readme() -> Result<()> {
    let dir = std::path::PathBuf::from("files/README")
        .canonicalize()
        .unwrap();

    let mut opts = metaforge::Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");
    opts.clean = true;
    opts.parallel = true;

    metaforge::build_site(&opts)
}
