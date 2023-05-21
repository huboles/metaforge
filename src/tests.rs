use crate::{MetaFile, Options};
use eyre::{Result, WrapErr};
use std::{error::Error, fs, path::PathBuf};

fn unit_test(test: (&str, &str)) -> Result<()> {
    let dir = PathBuf::from("files/test_site").canonicalize()?;

    let mut opts = Options::new();
    opts.root = dir.clone();
    opts.source = dir.join("source");
    opts.build = dir.join("build");
    opts.pattern = dir.join("pattern");
    opts.clean = true;
    opts.undefined = true;

    let test_dir = opts.source.join("unit_tests");
    let mut file_path = test_dir.join(test.0);
    file_path.set_extension("meta");
    let file = MetaFile::build(file_path, &opts)?;

    let output = file.construct().wrap_err_with(|| test.0.to_string())?;

    if output == test.1 {
        Ok(())
    } else {
        let err = eyre::eyre!("{} - failed", test.0);
        eprintln!("{:?}", err);
        eprintln!("\nTEST:\n{}\nOUTPUT:\n{}", test.1, output);
        Err(err)
    }
}

fn clean_build_dir() -> Result<()> {
    let build = PathBuf::from("files/test_site")
        .canonicalize()?
        .join("build");

    if build.exists() {
        fs::remove_dir_all(&build)?;
    }

    fs::create_dir_all(&build)?;
    Ok(())
}

#[test]
fn builder_tests() -> Result<()> {
    clean_build_dir()?;

    let tests: Vec<(&str, &str)> = vec![
        ("find_dest", "<html>\n</html>\n"),
        ("blank/blank_pattern", ""),
        ("blank/blank_variable", "<html>\n</html>\n"),
        ("blank/blank_array", "<html>\n</html>\n"),
        ("blank/comment", "<html>\n</html>\n"),
        (
            "blank/inline_comment",
            "<html>\n<p>inline comment</p>\n</html>\n",
        ),
        (
            "expand/variable_in_source",
            "<html>\n<p>GOOD</p>\n</html>\n",
        ),
        ("expand/variable_in_pattern", "<html>\nGOOD</html>\n"),
        ("expand/array_in_source", "<html>\n<p>12345</p>\n</html>\n"),
        ("expand/array_in_pattern", "<html>\n12345</html>\n"),
        ("expand/pattern_in_source", "<p>GOOD</p>\n"),
        ("expand/pattern_in_pattern", "<html>\nGOOD\nGOOD\n</html>\n"),
        ("override/variable", "<html>\n<p>GOOD</p>\n</html>\n"),
        ("override/pattern", "<html>\nGOOD\nGOOD\n</html>\n"),
        ("header/pandoc", "# This should not become html\n"),
        ("header/blank", ""),
    ];

    let mut err = false;
    let mut errs: Vec<Box<dyn Error>> = Vec::new();
    for test in tests.iter() {
        match unit_test(*test) {
            Ok(_) => continue,
            Err(e) => {
                err = true;
                errs.push(e.into());
            }
        }
    }

    if let Err(e) = test_filetype_header() {
        errs.push(e.into());
    }

    if let Err(e) = test_global() {
        errs.push(e.into());
    }

    if err {
        for e in errs.iter() {
            eprintln!("{}", e);
        }
        return Err(eyre::eyre!("failed tests"));
    }

    Ok(())
}

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
