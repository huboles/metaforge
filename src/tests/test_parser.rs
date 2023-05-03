use crate::parse_file;
use color_eyre::Result;

static SOURCE: &str = include_str!("test_source.meta");

#[test]
fn build_meta_file() -> Result<()> {
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
    assert_eq!(source.arrays.get("blank"), None);

    assert_eq!(source.patterns.get("pat").unwrap(), &"pattern");
    assert_eq!(source.patterns.get("pat.sub_pat"), None);
    assert_eq!(source.patterns.get("blank_pat"), None);
    assert_eq!(source.patterns.get("not_defined"), None);

    Ok(())
}
