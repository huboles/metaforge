use crate::{log, MetaError, MetaFile, Src};
use eyre::Result;
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc};

use super::array::*;
use super::*;

pub fn get_source_html(file: &MetaFile) -> Result<String> {
    let string = metafile_to_string(file)?;

    if file.opts.no_pandoc || !file.header.pandoc || string.is_empty() {
        return Ok(string);
    }

    let input: InputFormat;
    let output: OutputFormat;
    if let Ok(io) = get_pandoc_io(file) {
        input = io.0;
        output = io.1;
    } else {
        // don't run pandoc if a filetype that isn't supported gets requested
        return Ok(string);
    }

    log!(file.opts, "calling pandoc", 3);

    let mut pandoc = Pandoc::new();
    pandoc
        .set_input(InputKind::Pipe(string))
        .set_output(OutputKind::Pipe)
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
) -> Result<(pandoc::InputFormat, pandoc::OutputFormat), Box<MetaError>> {
    let mut source_type = "";
    if !file.header.source.is_empty() {
        source_type = &file.header.source;
    } else if !file.opts.input.is_empty() {
        source_type = &file.opts.input;
    }

    let input = match source_type {
        "markdown" => Ok(InputFormat::Markdown),
        "html" => Ok(InputFormat::Html),
        "org" => Ok(InputFormat::Org),
        "json" => Ok(InputFormat::Json),
        "latex" => Ok(InputFormat::Latex),
        _ => Err(Box::new(MetaError::Filetype)),
    }?;

    let mut filetype = "";
    if !file.header.filetype.is_empty() {
        filetype = &file.header.filetype;
    } else if !file.opts.input.is_empty() {
        filetype = &file.opts.output;
    }

    let output = match filetype {
        "html" => Ok(OutputFormat::Html),
        "markdown" => Ok(OutputFormat::Markdown),
        "man" => Ok(OutputFormat::Man),
        "txt" => Ok(OutputFormat::Plain),
        "org" => Ok(OutputFormat::Org),
        "json" => Ok(OutputFormat::Json),
        "latex" => Ok(OutputFormat::Latex),
        "asciidoc" => Ok(OutputFormat::Asciidoc),
        "pdf" => Ok(OutputFormat::Pdf),
        _ => Err(Box::new(MetaError::Filetype)),
    }?;

    Ok((input, output))
}
