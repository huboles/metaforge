use crate::{build_metafile, MetaFile, Options};
use color_eyre::{eyre::WrapErr, Result};
use std::path::PathBuf;

fn unit_test(test: (&str, &str)) -> Result<()> {
    let dir = PathBuf::from("files/test_site").canonicalize()?;

    let mut opts = Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");
    opts.clean = true;

    let test_dir = opts.source.join("unit_tests");
    let mut file_path = test_dir.join(test.0);
    file_path.set_extension("meta");
    let file = MetaFile::build(file_path, &opts)?;

    let output = build_metafile(&file).wrap_err_with(|| test.0.to_string())?;

    assert_eq!(output, test.1);

    Ok(())
}

#[test]
fn builder_tests() -> Result<()> {
    let mut tests: Vec<(&str, &str)> = Vec::new();
    tests.push(("find_dest", "<html>\n\n</html>\n"));
    tests.push(("blank/blank_pattern", ""));
    tests.push(("blank/blank_variable", "<html>\n</html>\n"));
    tests.push(("blank/blank_array", "<html>\n</html>\n"));
    tests.push(("blank/comment", "<html>\n\n</html>\n"));
    tests.push((
        "blank/inline_comment",
        "<html>\n<p>inline comment</p>\n</html>\n",
    ));
    tests.push((
        "expand/variable_in_source",
        "<html>\n<p>GOOD</p>\n</html>\n",
    ));
    tests.push(("expand/variable_in_pattern", "<html>\nGOOD</html>\n"));
    tests.push(("expand/array_in_source", "<html>\n<p>12345</p>\n</html>\n"));
    tests.push(("expand/array_in_pattern", "<html>\n12345</html>\n"));
    tests.push(("expand/pattern_in_source", "<p>GOOD</p>\n"));
    tests.push(("expand/pattern_in_pattern", "<html>\nGOOD\nGOOD\n</html>\n"));
    tests.push(("override/variable", "<html>\n<p>GOOD</p>\n</html>\n"));
    tests.push(("override/pattern", "<html>\nGOOD\nGOOD\n</html>\n"));
    tests.push(("header/pandoc", "# This should not become html\n"));
    tests.push(("header/blank", ""));

    for test in tests.iter() {
        unit_test(*test)?;
    }

    Ok(())
}

#[test]
fn test_filetype_header() -> Result<()> {
    let dir = PathBuf::from("files/test_site").canonicalize()?;

    let mut opts = Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");

    let path = opts.source.join("unit_tests/header/filetype.meta");
    let file = MetaFile::build(path, &opts)?;

    assert_eq!(
        file.dest()?,
        PathBuf::from(
            "/home/huck/repos/metaforge/files/test_site/build/unit_tests/header/filetype.rss"
        )
    );

    Ok(())
}

#[test]
fn test_global() -> Result<()> {
    let dir = PathBuf::from("files/test_site/").canonicalize()?;

    let mut opts = Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");

    let mut dir_node = crate::DirNode::build(dir.join("source/unit_tests/global"), &opts)?;
    let global = MetaFile::build(dir.join("source/default.meta"), &opts)?;
    dir_node.map(&global)?;
    dir_node.build_dir()?;

    assert_eq!(
        std::fs::read_to_string(dir.join("build/unit_tests/global/pattern.html"))?,
        "<p>GOOD GOOD</p>\n"
    );

    assert_eq!(
        std::fs::read_to_string(dir.join("build/unit_tests/global/variable.html"))?,
        "<p>GOODGOOD</p>\n"
    );

    Ok(())
}
