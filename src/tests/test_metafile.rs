use crate::parse_file;
use color_eyre::Result;

static SOURCE: &str = include_str!("test_source.meta");

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
