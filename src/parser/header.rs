use crate::Rule;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub fn parse_header_defs(pairs: Pairs<Rule>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in pairs {
        if Rule::header_assign == pair.as_rule() {
            let (key, val) = parse_header_assign(pair);
            map.insert(key.to_string(), val.to_string());
        }
    }
    map
}

fn parse_header_assign(pair: Pair<Rule>) -> (&str, &str) {
    let mut key = "";
    let mut val = "";

    for pair in pair.into_inner() {
        if Rule::key == pair.as_rule() {
            key = pair.as_str();
        }
        if Rule::header_value == pair.as_rule() {
            let tmp = pair.as_str();
            // blank and default shoud be handled by whoever is getting the value
            if tmp == "BLANK" || tmp == "true" || tmp == "false" {
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
