use crate::{Rule, Scope};
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub fn parse_defs(pairs: Pairs<Rule>) -> HashMap<Scope, String> {
    let mut map = HashMap::new();
    for pair in pairs {
        if Rule::assign == pair.as_rule() {
            let (key, val) = parse_assign(pair);
            map.insert(key, val.to_string());
        }
    }
    map
}

fn parse_assign(pair: Pair<Rule>) -> (Scope, &str) {
    let mut key = "";
    let mut val = "";
    let mut global = true;
    let mut trim = true;

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::scope => {
                if pair.as_str() == "*" {
                    global = false;
                } else {
                    global = true;
                }
            }
            Rule::key => key = pair.as_str(),
            Rule::value => {
                val = pair.as_str();
                if val == "BLANK" || val == "DEFAULT" {
                    trim = false;
                }
            }
            // nothing else is an acceptable assignment
            _ => unreachable!(),
        }
    }

    if trim {
        // remove surrounding quotes from values by returning
        // everything except first and last characters
        // a string is defined as " ... " or ' ... '
        // so it's safe to strip these characters
        val = &val[1..val.len() - 1];
    }

    if global {
        (Scope::into_global(key), val)
    } else {
        (Scope::into_local(key), val)
    }
}
