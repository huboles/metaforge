use super::*;

impl<'a> MetaFile<'a> {
    pub fn to_html(&self) -> Result<String> {
        let string = self.get_source()?;

        if self.opts.no_pandoc || !self.header.pandoc || string.is_empty() {
            return Ok(string);
        }

        let input: InputFormat;
        let output: OutputFormat;
        if let Ok(io) = self.pandoc_io() {
            input = io.0;
            output = io.1;
        } else {
            // don't run pandoc if a filetype that isn't supported gets requested
            return Ok(string);
        }

        log!(self.opts, "calling pandoc", 3);

        let mut pandoc = Pandoc::new();
        pandoc
            .set_input(InputKind::Pipe(string))
            .set_output(OutputKind::Pipe)
            .set_input_format(input, vec![])
            .set_output_format(output, vec![]);

        if let pandoc::PandocOutput::ToBuffer(s) = pandoc.execute()? {
            Ok(s)
        } else {
            Err(MetaError::Pandoc { file: self.name()? }.into())
        }
    }

    fn pandoc_io(&self) -> Result<(pandoc::InputFormat, pandoc::OutputFormat), Box<MetaError>> {
        let mut source_type = "";
        if !self.header.source.is_empty() {
            source_type = &self.header.source;
        } else if !self.opts.input.is_empty() {
            source_type = &self.opts.input;
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
        if !self.header.filetype.is_empty() {
            filetype = &self.header.filetype;
        } else if !self.opts.input.is_empty() {
            filetype = &self.opts.output;
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

    pub fn get_source(&self) -> Result<String> {
        if self.header.blank {
            return Ok(String::new());
        }

        let mut output = String::default();
        let mut arrays = false;

        for section in self.source.iter() {
            let sec = match section {
                // concatenate any char sequences
                Src::Str(str) => str.to_string(),
                // expand all variables and recursively expand patterns
                Src::Var(key) => self.get_variable(key)?,
                Src::Pat(key) => self.get_pattern(key)?,
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
            self.expand_arrays(output)
        } else {
            Ok(output)
        }
    }
}
