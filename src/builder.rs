mod array;
mod pattern;
mod source;
mod variable;

use pattern::*;
use source::*;
use variable::*;

#[cfg(test)]
mod tests;

use crate::{MetaFile, Scope};
use color_eyre::Result;

pub fn build_metafile(file: &MetaFile) -> Result<String> {
    if file.header.blank {
        return Ok(String::new());
    }

    let html = get_source_html(file)?;

    let pattern = get_pattern("base", file)?;
    let mut base = crate::parse_string(pattern, file.opts)?;

    base.merge(file);
    base.patterns.insert(Scope::into_global("SOURCE"), html);

    let output = metafile_to_string(&base)?;

    Ok(output)
}
