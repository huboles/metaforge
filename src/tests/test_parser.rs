use crate::{parse_file, source, Source, Substitution};
use color_eyre::Result;

static SOURCE: &str = include_str!("test_source.meta");
static PATTERN: &str = include_str!("test_pattern.meta");

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
