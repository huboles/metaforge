use crate::{source, MetaFile, Source, Substitution};
use color_eyre::{eyre::WrapErr, Result};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "meta.pest"]
pub struct MetaParser;

pub fn parse_file<'a>(file: String) -> Result<MetaFile> {
    let meta_source = MetaParser::parse(Rule::file, &file)
        .wrap_err("parser error")?
        .next()
        .unwrap();

    let metafile = parse_pair(meta_source);
    Ok(metafile)
}

fn parse_pair(pair: Pair<Rule>) -> MetaFile {
    let mut meta_file = MetaFile::new();

    if Rule::file == pair.as_rule() {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::source => meta_file.source = parse_source(pair.into_inner()),
                Rule::var_def => meta_file.variables = parse_defs(pair.into_inner()),
                Rule::arr_def => meta_file.arrays = parse_array_defs(pair.into_inner()),
                Rule::pat_def => meta_file.patterns = parse_defs(pair.into_inner()),
                // do nothing on end of file
                Rule::EOI => continue,
                // anything else is either hidden or children of previous nodes and will be dealt with
                // in respective parse functions
                _ => unreachable!(),
            }
        }
    }

    meta_file
}

fn parse_defs(pairs: Pairs<Rule>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in pairs {
        if Rule::assign == pair.as_rule() {
            let (key, val) = parse_assign(pair);
            map.insert(key.to_string(), val.to_string());
        }
    }
    map
}

fn parse_array_defs(pairs: Pairs<Rule>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for pair in pairs {
        if Rule::assign == pair.as_rule() {
            let (key, val) = parse_assign_array(pair);
            map.insert(key.to_string(), val);
        }
    }
    map
}

fn parse_source(pairs: Pairs<Rule>) -> Vec<Source> {
    let mut vec = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::var_sub => vec.push(source!(var(parse_sub(pair)))),
            Rule::arr_sub => vec.push(source!(arr(parse_sub(pair)))),
            Rule::pat_sub => vec.push(source!(pat(parse_sub(pair)))),
            Rule::char_seq => vec.push(source!(pair)),
            // anything that isn't a substitution is a char_seq inside source
            _ => unreachable!(),
        }
    }

    vec
}

fn parse_sub(pair: Pair<Rule>) -> &str {
    match pair.as_rule() {
        Rule::var_sub | Rule::arr_sub | Rule::pat_sub => {
            let str = pair.as_str();
            // return the value as the inner string for substitution
            // all substitutions have the format of
            //      *{ ... }
            // we return everything except:
            //      first two chars (sigil and preceding brace)
            //      last char (trailing brace)
            &str[2..str.len() - 1]
        }
        // this function only gets called to parse substituiton patterns
        // so anything else should never be called
        _ => unreachable!(),
    }
}

fn parse_assign(pair: Pair<Rule>) -> (&str, &str) {
    let mut key = "";
    let mut val = "";

    for pair in pair.into_inner() {
        if Rule::key == pair.as_rule() {
            key = pair.as_str();
        }
        if Rule::value == pair.as_rule() {
            let tmp = pair.as_str();
            // blank and default shoud be handled by whoever is getting the value
            if tmp == "BLANK" || tmp == "DEFAULT" {
                return (key, tmp);
            }
            // remove surrounding quotes from values by returning
            // everything except first and last characters
            // a string is defined as " ... " or ' ... '
            // so it's safe to strip these characters
            val = &tmp[1..tmp.len() - 1];
        }
    }

    (key, val)
}

fn parse_assign_array(pair: Pair<Rule>) -> (String, Vec<String>) {
    let mut key = "";
    let mut val = Vec::default();

    for pair in pair.into_inner() {
        if Rule::key == pair.as_rule() {
            key = pair.as_str();
        }
        if Rule::value == pair.as_rule() {
            val = parse_array(pair.into_inner());
        }
    }

    (key.to_string(), val)
}

fn parse_array(pairs: Pairs<Rule>) -> Vec<String> {
    let mut vec: Vec<String> = Vec::default();

    for pair in pairs {
        if Rule::string == pair.as_rule() {
            let tmp = pair.as_str();
            // remove surrounding quotes from values
            // see parse_assign() for reasoning
            let val = &tmp[1..tmp.len() - 1];
            vec.push(val.to_string());
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_str (
        ($s: expr) => {
            let str = $s.to_string();
            parse_file(str).unwrap();
        };
    );

    #[test]
    fn no_spaces_in_def() {
        test_str!(r#"${v='v'}@{a=['a']}&{p='p'}"#);
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
}
