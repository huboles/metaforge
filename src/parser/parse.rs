use crate::{source, MetaFile, Source, Substitution};
use color_eyre::{eyre::WrapErr, Result};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "parser/meta.pest"]
pub struct MetaParser;

pub fn parse_file(file: &str) -> Result<MetaFile> {
    let meta_source = MetaParser::parse(Rule::file, file)
        .wrap_err("parser error")?
        .next()
        .unwrap();

    Ok(parse_pair(meta_source))
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

fn parse_defs(pairs: Pairs<Rule>) -> HashMap<&'_ str, &'_ str> {
    let mut map = HashMap::new();
    for pair in pairs {
        if Rule::assign == pair.as_rule() {
            let (key, val) = parse_assign(pair);
            map.insert(key, val);
        }
    }
    map
}

fn parse_array_defs(pairs: Pairs<Rule>) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();
    for pair in pairs {
        if Rule::assign == pair.as_rule() {
            let (key, val) = parse_assign_array(pair);
            map.insert(key, val);
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
            Rule::char_seq => vec.push(source!(pair.as_str())),
            // anything that isn't a substitution is a char_seq inside source
            _ => unreachable!(),
        }
    }

    vec
}

fn parse_sub(pair: Pair<Rule>) -> &'_ str {
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

fn parse_assign(pair: Pair<Rule>) -> (&'_ str, &'_ str) {
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

fn parse_assign_array(pair: Pair<Rule>) -> (&str, Vec<&str>) {
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

    (key, val)
}

fn parse_array(pairs: Pairs<Rule>) -> Vec<&str> {
    let mut vec: Vec<&str> = Vec::default();

    for pair in pairs {
        if Rule::string == pair.as_rule() {
            let tmp = pair.as_str();
            // remove surrounding quotes from values
            // see parse_assign() for reasoning
            let val = &tmp[1..tmp.len() - 1];
            vec.push(val);
        }
    }

    vec
}
