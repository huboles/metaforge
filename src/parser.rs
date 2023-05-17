use crate::{Header, MetaFile, Options};
use color_eyre::{eyre::WrapErr, Result};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

mod array;
mod def_block;
mod header;
mod source;

use array::*;
use def_block::*;
use header::*;
use source::*;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "parser/meta.pest"]
pub struct MetaParser;

pub fn parse_string(file: String, opts: &Options) -> Result<MetaFile> {
    let meta_source = MetaParser::parse(Rule::file, &file)
        .wrap_err("parser error")?
        .next()
        .unwrap();

    let metafile = parse_file(meta_source, opts);
    Ok(metafile)
}

fn parse_file<'a>(pair: Pair<Rule>, opts: &'a Options) -> MetaFile<'a> {
    let mut meta_file = MetaFile::new(opts);

    if Rule::file == pair.as_rule() {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::source => meta_file.source = parse_source(pair.into_inner()),
                Rule::header => {
                    meta_file.header = Header::from(parse_header_defs(pair.into_inner()))
                }
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
