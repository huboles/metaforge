use color_eyre::Result;

#[test]
fn build_test_site() -> Result<()> {
    let dir = std::path::PathBuf::from("files/site")
        .canonicalize()
        .unwrap();

    let mut opts = metaforge::Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");
    opts.clean = true;

    metaforge::build_dir(&opts)?;

    assert!(opts.build.join("benchmark.html").exists());
    assert!(opts.build.join("dir1/sub_dir1/deep1/deep.html").exists());
    assert_eq!(
        std::fs::read_to_string(opts.build.join("root.html"))?,
        "<p>TEST</p>\n"
    );

    Ok(())
}
