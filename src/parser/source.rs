use crate::{
    parser::{Pair, Pairs},
    Rule, Src,
};

pub fn parse_source(pairs: Pairs<Rule>) -> Vec<Src> {
    let mut vec = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::var_sub => vec.push(Src::to_var(parse_sub(pair))),
            Rule::arr_sub => vec.push(Src::to_arr(parse_sub(pair))),
            Rule::pat_sub => vec.push(Src::to_pat(parse_sub(pair))),
            Rule::char_seq => vec.push(Src::to_str(pair.as_str())),
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
