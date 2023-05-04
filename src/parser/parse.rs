use crate::{MetaFile, Source, Substitution};
use color_eyre::Result;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "parser/meta.pest"]
pub struct MetaParser;

pub fn parse_file(file: &str) -> Result<MetaFile> {
    let meta_source = MetaParser::parse(Rule::file, file)?.next().unwrap();

    Ok(parse_pair(meta_source))
}

pub fn parse_pair(pair: Pair<Rule>) -> MetaFile {
    let mut meta_file = MetaFile::new();

    if Rule::file == pair.as_rule() {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::source => meta_file.source = parse_source(pair.into_inner()),
                Rule::var_def => meta_file.variables = parse_defs(pair.into_inner()),
                Rule::arr_def => meta_file.arrays = parse_array_defs(pair.into_inner()),
                Rule::pat_def => meta_file.patterns = parse_defs(pair.into_inner()),
                Rule::EOI => (),
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

fn parse_array_defs(pairs: Pairs<Rule>) -> HashMap<&'_ str, Vec<&'_ str>> {
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
    let mut vec: Vec<Source> = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::var_sub => vec.push(Source::Sub(Substitution::Variable(pair.as_str()))),
            Rule::arr_sub => vec.push(Source::Sub(Substitution::Array(pair.as_str()))),
            Rule::pat_sub => vec.push(Source::Sub(Substitution::Pattern(pair.as_str()))),
            Rule::char_seq => vec.push(Source::Str(pair.as_str())),
            // anything that isn't a substitution is a char_seq inside source
            _ => unreachable!(),
        }
    }

    vec
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
            // set it to empty strings do remove it from the HashMap
            if tmp == "BLANK" || tmp == "DEFAULT" {
                return ("", "");
            }
            // remove surrounding quotes from values
            val = &tmp[1..tmp.len() - 1];
        }
    }

    (key, val)
}

fn parse_assign_array(pair: Pair<Rule>) -> (&'_ str, Vec<&'_ str>) {
    let mut key = "";
    let mut val: Vec<&str> = Vec::default();
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

fn parse_array(pairs: Pairs<Rule>) -> Vec<&'_ str> {
    let mut vec: Vec<&str> = Vec::default();

    for pair in pairs {
        if Rule::string == pair.as_rule() {
            let tmp = pair.as_str();
            // remove surrounding quotes from values
            let val = &tmp[1..tmp.len() - 1];
            vec.push(val);
        }
    }
    vec
}
