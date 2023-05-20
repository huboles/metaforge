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

use crate::{log, Header, MetaError, MetaFile, Options};
use eyre::Result;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

#[derive(Parser)]
#[grammar = "parser/meta.pest"]
pub struct MetaParser;

pub fn parse_string(file: String, opts: &Options) -> Result<MetaFile> {
    log!(opts, "parsing file", 3);

    let pair = MetaParser::parse(Rule::file, &file)?.next().unwrap();

    let mut meta_file = MetaFile::new(opts);

    if Rule::file == pair.as_rule() {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::source => meta_file.source = parse_source(pair.into_inner()),
                Rule::header => {
                    meta_file.header = Header::from(parse_header_defs(pair.into_inner()))
                }
                Rule::var_def => meta_file.variables = parse_defs(pair.into_inner())?,
                Rule::arr_def => meta_file.arrays = parse_array_defs(pair.into_inner())?,
                Rule::pat_def => meta_file.patterns = parse_defs(pair.into_inner())?,
                // do nothing on end of file
                Rule::EOI => continue,
                // anything else is either hidden or children of previous nodes and will be dealt with
                // in respective parse functions
                _ => {
                    return Err(MetaError::UnreachableRule {
                        input: pair.to_string(),
                    }
                    .into())
                }
            }
        }
    }

    Ok(meta_file)
}
