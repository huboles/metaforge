#![allow(dead_code, unused)]
use crate::{metafile_to_string, parse_file, source, RootDirs, Source, Substitution};
use color_eyre::Result;
use std::path::PathBuf;

static SOURCE: &str = include_str!("./test_files/test_source.meta");
static PATTERN: &str = include_str!("./test_files/test_pattern.meta");
static PRE_EXPAND: &str = include_str!("./test_files/test_expand.meta");
static POST_EXPAND: &str = include_str!("./test_files/test_expanded");

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

#[ignore = "todo: build tmp directory"]
#[test]
fn test_metafile_to_str() -> Result<()> {
    let metafile = parse_file(PRE_EXPAND)?;
    let dirs = RootDirs {
        source: PathBuf::new(),
        build: PathBuf::new(),
        pattern: PathBuf::new(),
    };

    let file = metafile_to_string(&metafile, &dirs, None)?;

    assert_eq!(file, POST_EXPAND);

    Ok(())
}
