use crate::{MetaFile, Src, Sub};
use color_eyre::{eyre::bail, Result};

use super::array::*;
use super::*;

pub fn get_source_html(file: &MetaFile) -> Result<String> {
    let string = metafile_to_string(file)?;

    if file.opts.no_pandoc || !file.header.pandoc {
        return Ok(string);
    }

    let mut pandoc = pandoc::Pandoc::new();
    pandoc
        .set_input(pandoc::InputKind::Pipe(string))
        .set_output(pandoc::OutputKind::Pipe)
        .set_input_format(pandoc::InputFormat::Markdown, vec![])
        .set_output_format(pandoc::OutputFormat::Html, vec![]);

    if let Ok(pandoc::PandocOutput::ToBuffer(html)) = pandoc.execute() {
        Ok(html)
    } else {
        bail!("pandoc could not write to buffer")
    }
}

pub fn metafile_to_string(file: &MetaFile) -> Result<String> {
    if file.header.blank {
        return Ok(String::new());
    }

    let mut output = String::default();
    let mut arrays = false;

    for section in file.source.iter() {
        match section {
            // concatenate any char sequences
            Src::Str(str) => {
                output.push_str(str);
            }
            // expand all variables and recursively expand patterns
            Src::Sub(sub) => {
                let expanded = match sub {
                    Sub::Var(key) => super::variable::get_variable(key, file)?,
                    Sub::Pat(key) => get_pattern(key, file)?,
                    Sub::Arr(key) => {
                        arrays = true;
                        // comments have already been removed at this point,
                        // so we use them to mark keys for array substitution
                        format!("-{{{key}}}")
                    }
                };
                output.push_str(&expanded);
            }
        }
    }

    if arrays {
        expand_arrays(output, file)
    } else {
        Ok(output)
    }
}
