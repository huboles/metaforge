use crate::MetaFile;
use color_eyre::Result;

mod array;
mod pattern;
mod source;
mod variable;

use pattern::*;
use source::*;

#[cfg(test)]
mod tests;

pub fn build_metafile(file: &MetaFile) -> Result<String> {
    if file.header.blank {
        return Ok(String::new());
    }

    let html = get_source_html(file)?;

    let pattern = get_pattern("base", file)?;
    let mut base = crate::parse_string(pattern, file.opts)?;

    base.merge(file);
    base.patterns.insert("SOURCE".to_string(), html);

    let output = metafile_to_string(&base)?;

    Ok(output)
}
