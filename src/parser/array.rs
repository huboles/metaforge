use crate::{Rule, Scope};
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub fn parse_array_defs(pairs: Pairs<Rule>) -> HashMap<Scope, Vec<String>> {
    let mut map = HashMap::new();
    for pair in pairs {
        if Rule::assign == pair.as_rule() {
            let (key, val) = parse_assign_array(pair);
            map.insert(key, val);
        }
    }
    map
}

fn parse_assign_array(pair: Pair<Rule>) -> (Scope, Vec<String>) {
    let mut key = "";
    let mut val = Vec::default();
    let mut global = true;

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
            Rule::value => val = parse_array(pair.into_inner()),
            _ => unreachable!(),
        }
    }

    if global {
        (Scope::into_global(key), val)
    } else {
        (Scope::into_local(key), val)
    }
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
