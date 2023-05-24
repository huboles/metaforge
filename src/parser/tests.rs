macro_rules! test_str (
    ($s: expr) => {
        let opts = crate::Options::new();
        let str = $s.to_string();
        crate::parse_string(str, &opts).unwrap();
    };
);

#[test]
fn no_spaces_in_def() {
    test_str!(r#"${v='v'}@{a=['a']}&{p='p'}"#);
}

#[test]
fn newlines_in_array() {
    test_str!("@{ a = [\n'stuff',\n'other stuff'\n] }");
}

#[test]
fn trailing_commas() {
    test_str!(r#"@{ a = ['stuff','other stuff',] }"#);
}

#[test]
fn just_source_string() {
    test_str!(r#"This is just a &{source} snippet"#);
}

#[test]
fn one_line() {
    test_str!(
        r#"${variable = 'var' } @{array = ['array']} &{ pattern = "pattern"} And some extra text"#
    );
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

#[test]
#[should_panic]
fn map_source_map() {
    test_str!(r#"${var='v'} Some text @{array = ['a']}"#);
}

#[test]
#[should_panic]
fn header_not_first() {
    test_str!(r#"${v='v'} #{ type = 'html'} @{a=['a']}"#);
}
