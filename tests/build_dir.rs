use eyre::Result;

#[test]
fn build_test_site() -> Result<()> {
    let dir = std::path::PathBuf::from("files/test_site")
        .canonicalize()
        .unwrap();

    let mut opts = metaforge::Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");
    opts.clean = true;

    metaforge::build_site(&opts)?;

    assert!(opts.build.join("unit_tests").exists());
    assert!(opts
        .build
        .join("unit_tests/blank/blank_array.html")
        .exists());
    assert!(opts
        .build
        .join("unit_tests/expand/variable_in_source.html")
        .exists());
    assert!(opts
        .build
        .join("unit_tests/override/variable.html")
        .exists());
    assert!(opts.build.join("unit_tests/global/pattern.html").exists());

    Ok(())
}

#[test]
fn parallel_build_test_site() -> Result<()> {
    let dir = std::path::PathBuf::from("files/test_site")
        .canonicalize()
        .unwrap();

    let mut opts = metaforge::Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");
    opts.clean = true;
    opts.parallel = true;

    metaforge::build_site(&opts)?;

    assert!(opts.build.join("unit_tests").exists());
    assert!(opts
        .build
        .join("unit_tests/blank/blank_array.html")
        .exists());
    assert!(opts
        .build
        .join("unit_tests/expand/variable_in_source.html")
        .exists());
    assert!(opts
        .build
        .join("unit_tests/override/variable.html")
        .exists());
    assert!(opts.build.join("unit_tests/global/pattern.html").exists());

    Ok(())
}
