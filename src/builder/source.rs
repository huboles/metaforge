use crate::{MetaError, MetaFile, Src};
use eyre::Result;

use super::array::*;
use super::*;

pub fn get_source_html(file: &MetaFile) -> Result<String> {
    let string = metafile_to_string(file)?;

    if file.opts.no_pandoc || !file.header.pandoc || string == "" {
        return Ok(string);
    }

    let input: pandoc::InputFormat;
    let output: pandoc::OutputFormat;
    if let Ok(io) = get_pandoc_io(&file) {
        input = io.0;
        output = io.1;
    } else {
        // don't run pandoc if a filetype that isn't supported gets requested
        return Ok(string);
    }

    let mut pandoc = pandoc::Pandoc::new();
    pandoc
        .set_input(pandoc::InputKind::Pipe(string))
        .set_output(pandoc::OutputKind::Pipe)
        .set_input_format(input, vec![])
        .set_output_format(output, vec![]);

    if let pandoc::PandocOutput::ToBuffer(s) = pandoc.execute()? {
        Ok(s)
    } else {
        Err(MetaError::Pandoc { file: file.name()? }.into())
    }
}

pub fn metafile_to_string(file: &MetaFile) -> Result<String> {
    if file.header.blank {
        return Ok(String::new());
    }

    let mut output = String::default();
    let mut arrays = false;

    for section in file.source.iter() {
        let sec = match section {
            // concatenate any char sequences
            Src::Str(str) => str.to_string(),
            // expand all variables and recursively expand patterns
            Src::Var(key) => get_variable(key, file)?,
            Src::Pat(key) => get_pattern(key, file)?,
            Src::Arr(key) => {
                arrays = true;
                // comments have already been removed at this point,
                // so we use them to mark keys for array substitution
                format!("-{{{key}}}")
            }
        };

        output.push_str(&sec);
    }

    if arrays {
        expand_arrays(output, file)
    } else {
        Ok(output)
    }
}

fn get_pandoc_io(
    file: &MetaFile,
) -> Result<(pandoc::InputFormat, pandoc::OutputFormat), MetaError> {
    let input: pandoc::InputFormat;
    let output: pandoc::OutputFormat;

    let mut source_type = "";
    if !file.header.source.is_empty() {
        source_type = &file.header.source;
    } else if !file.opts.input.is_empty() {
        source_type = &file.opts.input;
    }

    match source_type {
        "markdown" => input = pandoc::InputFormat::Markdown,
        "html" => input = pandoc::InputFormat::Html,
        "org" => input = pandoc::InputFormat::Org,
        "json" => input = pandoc::InputFormat::Json,
        "latex" => input = pandoc::InputFormat::Latex,
        _ => return Err(MetaError::Filetype.into()),
    }

    let mut filetype = "";
    if !file.header.filetype.is_empty() {
        filetype = &file.header.filetype;
    } else if !file.opts.input.is_empty() {
        filetype = &file.opts.output;
    }

    match filetype {
        "html" => output = pandoc::OutputFormat::Html,
        "markdown" => output = pandoc::OutputFormat::Markdown,
        "man" => output = pandoc::OutputFormat::Man,
        "txt" => output = pandoc::OutputFormat::Plain,
        "org" => output = pandoc::OutputFormat::Org,
        "json" => output = pandoc::OutputFormat::Json,
        "latex" => output = pandoc::OutputFormat::Latex,
        "asciidoc" => output = pandoc::OutputFormat::Asciidoc,
        "pdf" => output = pandoc::OutputFormat::Pdf,
        _ => return Err(MetaError::Filetype.into()),
    };

    Ok((input, output))
}
