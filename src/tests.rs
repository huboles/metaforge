use crate::{MetaFile, Options};
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
            let file = MetaFile::build(path, &opts)?;
            assert_eq!(file.construct()?, $test);
            Ok(())
        }
    };
);

unit_test!(blank_pattern, "blank/blank_pattern", "");
unit_test!(blank_variable, "blank/blank_variable", "<html>\n</html>\n");
unit_test!(blank_array, "blank/blank_array", "<html>\n</html>\n");
unit_test!(blank_comment, "blank/comment", "<html>\n</html>\n");
unit_test!(
    inline_comment,
    "blank/inline_comment",
    "<html>\n<p>inline comment</p>\n</html>\n"
);
unit_test!(
    expand_var_in_src,
    "expand/variable_in_source",
    "<html>\n<p>GOOD</p>\n</html>\n"
);
unit_test!(
    expand_var_in_pat,
    "expand/variable_in_pattern",
    "<html>\nGOOD</html>\n"
);
unit_test!(
    expand_arr_in_src,
    "expand/array_in_source",
    "<html>\n<p>12345</p>\n</html>\n"
);
unit_test!(
    expand_arr_in_pat,
    "expand/array_in_pattern",
    "<html>\n12345</html>\n"
);
unit_test!(
    expand_pat_in_src,
    "expand/pattern_in_source",
    "<p>GOOD</p>\n"
);
unit_test!(
    expand_pat_in_pat,
    "expand/pattern_in_pattern",
    "<html>\nGOOD\nGOOD\n</html>\n"
);
unit_test!(
    override_var,
    "override/variable",
    "<html>\n<p>GOOD</p>\n</html>\n"
);
unit_test!(
    override_pat,
    "override/pattern",
    "<html>\nGOOD\nGOOD\n</html>\n"
);
unit_test!(
    header_no_pandoc,
    "header/pandoc",
    "# This should not become html\n"
);

unit_test!(header_blank, "header/blank", "");

unit_test!(
    pat_file,
    "expand/file.meta",
    "<html>\n<p>GOOD</p>\n</html>\n"
);

#[test]
fn test_filetype_header() -> Result<()> {
    let dir = PathBuf::from("files/test_site").canonicalize()?;

    let mut opts = Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");

    let path = opts.source.join("unit_tests/header/filetype.meta");
    let file = MetaFile::build(path, &opts)?;

    if file.dest()? == opts.build.join("unit_tests/header/filetype.rss") {
        Ok(())
    } else {
        let err = eyre::eyre!("filetype - failed");
        eprintln!("{:?}", err);
        eprintln!(
            "\nTEST:\n{}\nOUTPUT:\n{}",
            opts.build.join("unit_tests/header/filetype.rss").display(),
            file.dest()?.display()
        );
        Err(err)
    }
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
        fs::read_to_string(dir.join("build/unit_tests/global/pattern.html"))?,
        "<p>GOOD GOOD</p>\n"
    );

    assert_eq!(
        fs::read_to_string(dir.join("build/unit_tests/global/variable.html"))?,
        "<p>GOODGOOD</p>\n"
    );

    Ok(())
}
