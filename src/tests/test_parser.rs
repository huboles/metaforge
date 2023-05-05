use crate::parse_file;

macro_rules! test_str (
    ($s: expr) => {
        let str = $s;
        parse_file(str).unwrap();
    };
);

#[test]
fn no_spaces_def() {
    test_str!(r#"${v='v'}@{a=['a']}&{p='p'}"#);
}

#[test]
fn just_source() {
    test_str!(r#"This is just a &{source} snippet"#);
}

#[test]
#[should_panic]
fn key_with_spaces() {
    test_str!(r#"${ key with spaces = "value" }"#);
}

#[test]
#[should_panic]
fn value_missing_quote() {
    test_str!(r#"${ key = "value missing quote }"#);
}

#[test]
#[should_panic]
fn mixed_quotes() {
    test_str!(r#"${ key = "value mixing quotes' }"#);
}

#[test]
#[should_panic]
fn spaces_in_substitution() {
    test_str!(r#"This ${variable is not allowed}"#);
}

#[test]
#[should_panic]
fn missing_closing_brace() {
    test_str!(r#"${ key = "value" "#);
}

#[test]
#[should_panic]
fn map_in_source() {
    test_str!(r#"This map: ${ is = "invalid" }"#);
}
