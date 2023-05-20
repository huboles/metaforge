mod array;
mod pattern;
mod source;
mod variable;

use pattern::*;
use source::*;
use variable::*;

#[cfg(test)]
mod tests;

use crate::{MetaError, MetaFile, Scope};
use eyre::Result;

pub fn build_metafile(file: &MetaFile) -> Result<String, Box<MetaError>> {
    if file.header.blank {
        return Ok(String::new());
    } else if file.header.ignore {
        return Err(Box::new(MetaError::Ignored));
    }

    let html = get_source_html(file).map_err(MetaError::from)?;

    let pattern = get_pattern("base", file).map_err(MetaError::from)?;
    let mut base = crate::parse_string(pattern, file.opts).map_err(MetaError::from)?;

    base.merge(file);
    base.patterns.insert(Scope::into_global("SOURCE"), html);

    let output = metafile_to_string(&base).map_err(MetaError::from)?;

    Ok(output)
}
