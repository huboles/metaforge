use crate::{MetaError, MetaFile, Options};
use eyre::Result;
use std::{fs, path::PathBuf};

macro_rules! unit_test (
    ($name:ident, $file:expr,$test:literal) => {
        #[test]
        fn $name() -> Result<()> {
            let dir = PathBuf::from("files/test_site").canonicalize()?;

            let mut opts = Options::new();
            opts.root = dir.clone();
            opts.source = dir.join("source");
            opts.build = dir.join("build");
            opts.pattern = dir.join("pattern");

            let test_dir = opts.source.join("unit_tests");
            let mut path = test_dir.join($file);
            path.set_extension("meta");
            let mut file = MetaFile::build(path, &opts)?;

            let str = match file.construct() {
                Ok(f) => f,
                Err(e) => match *e {
                    MetaError::Ignored => return Ok(()),
                    e => return Err(e.into())
                }

            };

            assert_eq!(str, $test);
            Ok(())
        }
    };
);

macro_rules! panic_test (
    ($name:ident, $file:expr,$test:literal) => {
        #[test]
        #[should_panic]
        fn $name() {
            let dir = PathBuf::from("files/test_site").canonicalize().unwrap_or_default();

            let mut opts = Options::new();
            opts.root = dir.clone();
            opts.source = dir.join("source");
            opts.build = dir.join("build");
            opts.pattern = dir.join("pattern");

            let test_dir = opts.source.join("unit_tests");
            let mut path = test_dir.join($file);
            path.set_extension("meta");
            let mut file = MetaFile::build(path, &opts).unwrap();
            assert_eq!(file.construct().unwrap(), $test);
        }
    };
);

unit_test!(blank_pattern, "blank/blank_pattern", "");
unit_test!(
    blank_variable,
    "blank/blank_variable",
    "<html>\n\n\n</html>\n"
);
unit_test!(blank_array, "blank/blank_array", "<html>\n\n\n</html>\n");
unit_test!(blank_comment, "blank/comment", "<html>\n\n\n\n</html>\n");
unit_test!(
    inline_comment,
    "blank/inline_comment",
    "<html>\n<p>inline comment</p>\n\n\n\n</html>\n"
);
unit_test!(
    expand_var_in_src,
    "expand/variable_in_source",
    "<html>\n<p>GOOD</p>\n\n\n\n</html>\n"
);
unit_test!(
    expand_var_in_pat,
    "expand/variable_in_pattern",
    "<html>\nGOOD\n\n\n</html>\n"
);
unit_test!(
    expand_arr_in_src,
    "expand/array_in_source",
    "<html>\n<p>1 2 3 4 5</p>\n\n\n\n</html>\n"
);
unit_test!(
    expand_arr_in_pat,
    "expand/array_in_pattern",
    "<html>\n1\n2\n3\n4\n5\n\n\n</html>\n"
);
unit_test!(
    expand_pat_in_src,
    "expand/pattern_in_source",
    "<p>GOOD</p>\n\n"
);
unit_test!(
    expand_pat_in_pat,
    "expand/pattern_in_pattern",
    "<html>\nGOOD\nGOOD\n\n\n\n</html>\n"
);
unit_test!(
    override_var,
    "override/variable",
    "<html>\n<p>GOOD</p>\n\n\n\n</html>\n"
);
unit_test!(
    override_pat,
    "override/pattern",
    "<html>\nGOOD\n GOOD\n\n\n\n</html>\n"
);
unit_test!(
    header_no_pandoc,
    "header/pandoc",
    "# This should not become html\n\n"
);

unit_test!(header_blank, "header/blank", "");

unit_test!(
    pat_file,
    "expand/file.meta",
    "<html>\n<p>GOOD</p>\n\n\n\n</html>\n"
);

unit_test!(
    direct_call,
    "expand/direct_call",
    "<html>\n<p>a b c d</p>\n\n\n\n</html>\n"
);

unit_test!(
    expand_spaces,
    "expand/spaces",
    "<html>\n<p>GOOD GOOD</p>\n\n\n\n</html>\n"
);

unit_test!(
    copy_header,
    "header/copy",
    r#"variable: ${this} should get copied verbatim"#
);

unit_test!(
    expandoc,
    "header/expandoc",
    "<html>\n<h1 id=\"good\">GOOD</h1>\n\n\n</html>\n"
);

unit_test!(
    include_source,
    "expand/source",
    "<html>\n<p>GOOD GOOD</p>\n\n\n\n</html>\n"
);

panic_test!(ignore, "ignore.meta", "");

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
        opts.build.join("unit_tests/header/filetype.rss")
    );

    Ok(())
}

#[test]
fn test_global() -> Result<()> {
    let dir = PathBuf::from("files/test_site").canonicalize()?;

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
        fs::read_to_string(dir.join("build/unit_tests/global/pattern.html"))?,
        "<p>GOOD</p><p>GOOD</p>"
    );

    assert_eq!(
        fs::read_to_string(dir.join("build/unit_tests/global/variable.html"))?,
        "<p>GOOD GOOD</p>"
    );

    Ok(())
}
