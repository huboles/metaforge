use eyre::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub root: PathBuf,
    pub source: PathBuf,
    pub build: PathBuf,
    pub pattern: PathBuf,
    pub file: Option<PathBuf>,
    pub input: String,
    pub output: String,
    pub verbose: u8,
    pub quiet: bool,
    pub force: bool,
    pub undefined: bool,
    pub clean: bool,
    pub no_pandoc: bool,
    pub new: bool,
}

impl Options {
    pub fn new() -> Self {
        Self {
            root: PathBuf::new(),
            source: PathBuf::new(),
            build: PathBuf::new(),
            pattern: PathBuf::new(),
            file: None,
            input: String::default(),
            output: String::default(),
            verbose: 0,
            quiet: false,
            force: false,
            undefined: false,
            clean: false,
            no_pandoc: false,
            new: false,
        }
    }
}

impl TryFrom<crate::Opts> for Options {
    type Error = eyre::Error;
    fn try_from(value: crate::Opts) -> Result<Self, Self::Error> {
        let mut opts = Options::new();

        opts.verbose = value.verbose;
        opts.quiet = value.quiet;
        opts.force = value.force;
        opts.undefined = value.undefined;
        opts.clean = value.clean;
        opts.no_pandoc = value.no_pandoc;
        opts.new = value.new;

        if let Some(root) = value.root.as_deref() {
            opts.root = PathBuf::from(root).canonicalize()?;
        } else {
            opts.root = std::env::current_dir()?;
        }

        if let Some(source) = value.source.as_deref() {
            opts.source = PathBuf::from(source).canonicalize()?;
        } else {
            opts.source = opts.root.join("source");
        }

        if let Some(build) = value.build.as_deref() {
            opts.build = PathBuf::from(build).canonicalize()?;
        } else {
            opts.build = opts.root.join("build");
        }

        if let Some(pattern) = value.pattern.as_deref() {
            opts.pattern = PathBuf::from(pattern).canonicalize()?;
        } else {
            opts.pattern = opts.root.join("pattern");
        }

        if let Some(file) = value.file.as_deref() {
            opts.file = Some(PathBuf::from(file).canonicalize()?);
        }

        if let Some(input) = value.input {
            opts.input = input;
        } else {
            opts.input = String::from("html");
        }

        if let Some(output) = value.output {
            opts.output = output;
        } else {
            opts.output = String::from("markdown");
        }

        Ok(opts)
    }
}
