use crate::{metafile_to_string, parse_file, source, RootDirs, Source, Substitution};
use color_eyre::Result;
use pretty_assertions::assert_eq;
use std::path::PathBuf;

static SOURCE: &str = include_str!("./test_files/test_site/source/test_source.meta");
static PATTERN: &str = include_str!("./test_files/test_site/pattern/test/pattern.meta");
static PRE_EXPAND: &str = include_str!("./test_files/test_site/source/expand.meta");
static POST_EXPAND: &str = include_str!("./test_files/test_expanded");

// builds a tmp_dir then runs multiple tests on it, then deletes the tmpdir
// so we don't have to rebuild entire tmpdir every test
fn test_on_tmp_dir(tests: Vec<fn(dirs: &RootDirs) -> Result<()>>) -> Result<()> {
    let tmp_dir = std::env::temp_dir();

    let dirs = RootDirs {
        source: tmp_dir.join("source"),
        build: tmp_dir.join("site"),
        pattern: tmp_dir.join("pattern"),
    };

    for test in tests.iter() {
        std::fs::remove_dir_all(&dirs.build)?;
        std::fs::create_dir(&dirs.build)?;
        test(&dirs)?;
    }

    std::fs::remove_dir_all(tmp_dir)?;
    Ok(())
}

#[test]
fn test_metafile_gets() -> Result<()> {
    let source = parse_file(SOURCE)?;

    assert_eq!(source.get_var("var").unwrap(), "good");
    assert_eq!(source.get_var("single").unwrap(), "quotes");
    assert_eq!(source.get_var("blank"), None);
    assert_eq!(source.get_var("not_defined"), None);

    assert_eq!(source.get_arr("sub.array").unwrap(), ["sub", "value"]);
    assert_eq!(source.get_arr("arr").unwrap(), ["split", "up", "values"]);
    assert_eq!(
        source.get_arr("with_spaces").unwrap(),
        ["stuff", "with", "spaces"]
    );
    assert_eq!(source.get_arr("not_defined"), None);

    assert_eq!(source.get_pat("pat").unwrap(), "pattern");
    assert_eq!(source.get_pat("pat.sub_pat"), None);
    assert_eq!(source.get_pat("blank_pat"), None);
    assert_eq!(source.get_pat("not_defined"), None);

    Ok(())
}

#[test]
fn parse_meta_file() -> Result<()> {
    let source = parse_file(SOURCE)?;

    assert_eq!(source.variables.get("var").unwrap(), &"good");
    assert_eq!(source.variables.get("blank"), None);
    assert_eq!(source.variables.get("not_here"), None);

    assert_eq!(
        source.arrays.get("sub.array").unwrap(),
        &vec!["sub", "value"]
    );
    assert_eq!(
        source.arrays.get("arr").unwrap(),
        &vec!["split", "up", "values"]
    );
    assert_eq!(
        source.arrays.get("with_spaces").unwrap(),
        &vec!["stuff", "with", "spaces"]
    );
    assert_eq!(source.arrays.get("not_defined"), None);

    assert_eq!(source.patterns.get("pat").unwrap(), &"pattern");
    assert_eq!(source.patterns.get("pat.sub_pat"), None);
    assert_eq!(source.patterns.get("blank_pat"), None);
    assert_eq!(source.patterns.get("not_defined"), None);

    Ok(())
}

#[test]
fn parse_pattern_file() -> Result<()> {
    let mut pattern_src = parse_file(PATTERN)?.source.into_iter();

    pattern_src.next();
    assert_eq!(pattern_src.next().unwrap(), source!(var("var")));
    pattern_src.next();
    assert_eq!(pattern_src.next().unwrap(), source!(pat("pat")));
    assert_eq!(pattern_src.next().unwrap(), source!(arr("array")));
    pattern_src.next();
    assert_eq!(pattern_src.next().unwrap(), source!(var("blank")));

    Ok(())
}

#[test]
fn builder_tests() -> Result<()> {
    // vector of tests to perform on tmp_dir
    let mut tests: Vec<fn(dirs: &RootDirs) -> Result<()>> = Vec::default();
    tests.push(test_metafile_to_str);
    test_on_tmp_dir(tests)?;
    Ok(())
}

fn test_metafile_to_str(dirs: &RootDirs) -> Result<()> {
    let metafile = parse_file(PRE_EXPAND)?;

    let file = metafile_to_string(&metafile, dirs, None)?;

    assert_eq!(file, POST_EXPAND);

    Ok(())
}
