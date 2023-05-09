use crate::{metafile_to_string, parse_file, source, RootDirs, Source, Substitution};
use color_eyre::Result;
use pretty_assertions::assert_eq;
use std::{fs, path::PathBuf};

static SOURCE: &str = include_str!("../../tests/files/test_site/source/test_source.meta");
static PATTERN: &str = include_str!("../../tests/files//test_site/pattern/test/pattern.meta");

#[test]
fn test_metafile_gets() -> Result<()> {
    let source = parse_file(SOURCE)?;

    assert_eq!(source.get_var("var").unwrap(), "GOOD");
    assert_eq!(source.get_var("single_quotes").unwrap(), "GOOD");
    assert_eq!(source.get_var("blank"), None);
    assert_eq!(source.get_var("not_defined"), None);

    assert_eq!(source.get_arr("sub.array").unwrap(), ["GOOD", "GOOD"]);
    assert_eq!(source.get_arr("arr").unwrap(), ["GOOD", "GOOD", "GOOD"]);
    assert_eq!(
        source.get_arr("with_spaces").unwrap(),
        ["GOOD", "GOOD", "GOOD"]
    );
    assert_eq!(source.get_arr("not_defined"), None);

    assert_eq!(source.get_pat("test").unwrap(), "pattern");
    assert_eq!(source.get_pat("test.sub_pat"), None);
    assert_eq!(source.get_pat("blank_pat"), None);
    assert_eq!(source.get_pat("not_defined"), None);

    Ok(())
}

#[test]
fn parse_meta_file() -> Result<()> {
    let source = parse_file(SOURCE)?;

    assert_eq!(source.variables.get("var").unwrap(), &"GOOD");
    assert_eq!(source.variables.get("blank"), None);
    assert_eq!(source.variables.get("not_here"), None);

    assert_eq!(
        source.arrays.get("sub.array").unwrap(),
        &vec!["GOOD", "GOOD"]
    );
    assert_eq!(
        source.arrays.get("arr").unwrap(),
        &vec!["GOOD", "GOOD", "GOOD"]
    );
    assert_eq!(
        source.arrays.get("with_spaces").unwrap(),
        &vec!["GOOD", "GOOD", "GOOD"]
    );
    assert_eq!(source.arrays.get("not_defined"), None);

    assert_eq!(source.patterns.get("test").unwrap(), &"pattern");
    assert_eq!(source.patterns.get("test.sub_pat"), None);
    assert_eq!(source.patterns.get("blank_pat"), None);
    assert_eq!(source.patterns.get("not_defined"), None);

    Ok(())
}

#[ignore = "Need to rewrite pattern test file"]
#[test]
fn parse_pattern_file() -> Result<()> {
    let mut pattern_src = parse_file(PATTERN)?.source.into_iter();

    pattern_src.next();
    assert_eq!(pattern_src.next().unwrap(), source!(var("variable")));
    pattern_src.next();
    assert_eq!(pattern_src.next().unwrap(), source!(arr("array")));
    pattern_src.next();
    assert_eq!(pattern_src.next().unwrap(), source!(pat("pattern")));
    pattern_src.next();
    assert_eq!(pattern_src.next(), None);

    Ok(())
}
