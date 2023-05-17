use crate::Rule;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub fn parse_array_defs(pairs: Pairs<Rule>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for pair in pairs {
        if Rule::assign == pair.as_rule() {
            let (key, val) = parse_assign_array(pair);
            map.insert(key.to_string(), val);
        }
    }
    map
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
