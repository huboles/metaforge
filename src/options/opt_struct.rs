use color_eyre::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub root: PathBuf,
    pub source: PathBuf,
    pub build: PathBuf,
    pub pattern: PathBuf,
    pub verbose: u8,
    pub quiet: bool,
    pub force: bool,
    pub undefined: bool,
    pub clean: bool,
    pub no_pandoc: bool,
}

impl Options {
    pub fn new() -> Self {
        Self {
            root: PathBuf::new(),
            source: PathBuf::new(),
            build: PathBuf::new(),
            pattern: PathBuf::new(),
            verbose: 0,
            quiet: false,
            force: false,
            undefined: false,
            clean: false,
            no_pandoc: false,
        }
    }
}

impl TryFrom<crate::Opts> for Options {
    type Error = color_eyre::eyre::Error;
    fn try_from(value: crate::Opts) -> Result<Self, Self::Error> {
        let mut options = Options::new();

        options.verbose = value.verbose;
        options.quiet = value.quiet;
        options.force = value.force;
        options.undefined = value.undefined;
        options.clean = value.clean;
        options.no_pandoc = value.no_pandoc;

        if let Some(root) = value.root.as_deref() {
            options.root = PathBuf::from(root).canonicalize()?;
        } else {
            options.root = std::env::current_dir()?;
        }

        if let Some(source) = value.source.as_deref() {
            options.source = PathBuf::from(source).canonicalize()?;
        } else {
            options.source = options.root.join("source");
        }

        if let Some(build) = value.build.as_deref() {
            options.build = PathBuf::from(build).canonicalize()?;
        } else {
            options.build = options.root.join("build");
        }

        if let Some(pattern) = value.pattern.as_deref() {
            options.pattern = PathBuf::from(pattern).canonicalize()?;
        } else {
            options.pattern = options.root.join("pattern");
        }

        Ok(options)
    }
}
