use clap::Parser;
use eyre::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "Huck Boles")]
#[command(version = "0.1.1")]
#[command(about = "A customizable template driven static site generator")]
#[command(long_about = None)]
pub struct Opts {
    /// root directory [current_dir]
    #[arg(short, long, value_name = "ROOT_DIR")]
    pub root: Option<String>,
    /// source file directory [current_dir/source]
    #[arg(short, long, value_name = "SOURCE_DIR")]
    pub source: Option<String>,
    /// build directory [current_dir/build]
    #[arg(short, long, value_name = "BUILD_DIR")]
    pub build: Option<String>,
    /// pattern directory [current_dir/pattern]
    #[arg(short, long, value_name = "PATTERN_DIR")]
    pub pattern: Option<String>,
    /// only build a single file
    #[arg(short, long, value_name = "FILENAME")]
    pub file: Option<String>,
    /// create a new skeleton directory
    #[arg(long, default_value_t = false)]
    pub new: bool,
    /// enable extra output. repeated flags give more info
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// minimal output
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,
    /// don't stop on file failure [false]
    #[arg(long, default_value_t = false)]
    pub force: bool,
    /// stop on undefined variables and arrays [false]
    #[arg(long, default_value_t = false)]
    pub undefined: bool,
    /// clean build directory before building site [false]
    #[arg(long, default_value_t = false)]
    pub clean: bool,
    /// don't call pandoc on source files
    #[arg(long, default_value_t = false)]
    pub no_pandoc: bool,
    /// output filetype [html]
    #[arg(short, long, value_name = "OUTPUT_FILETYPE")]
    pub output: Option<String>,
    /// input filetype [markdown]
    #[arg(short, long, value_name = "INPUT_FILETYPE")]
    pub input: Option<String>,
}

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
