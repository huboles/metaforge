use clap::Parser;
use color_eyre::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "Huck Boles")]
#[command(version = "0.1.1")]
#[command(about = "A customizable template driven static site generator")]
#[command(long_about = None)]
pub struct Opts {
    /// Root directory [CURRENT_DIR]
    #[arg(short, long, value_name = "ROOT_DIR")]
    pub root: Option<String>,
    /// Source file directory [CURRENT_DIR/source]
    #[arg(short, long, value_name = "SOURCE_DIR")]
    source: Option<String>,
    /// Build directory [CURRENT_DIR/build]
    #[arg(short, long, value_name = "BUILD_DIR")]
    build: Option<String>,
    /// Pattern directory [CURRENT_DIR/pattern]
    #[arg(short, long, value_name = "PATTERN_DIR")]
    pattern: Option<String>,
    /// Enable extra output.
    /// Repeated flags give more info
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    /// Minimal output
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
    /// Don't stop on file failure [FALSE]
    #[arg(long, default_value_t = false)]
    force: bool,
    /// Stop on undefined variables, patterns, and arrays [FALSE]
    #[arg(long, default_value_t = false)]
    undefined: bool,
    /// Clean build directory before building site [FALSE]
    #[arg(long, default_value_t = false)]
    clean: bool,
}

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
        }
    }
}

#[macro_export]
macro_rules! log {
    ($opts:ident, $string:expr, $level:expr) => {
        if $opts.verbose >= $level && !$opts.quiet {
            println!("{}", $string);
        }
    };
}

pub fn parse_opts() -> Result<Options> {
    let opts = Opts::parse();

    let mut options = Options::new();
    options.verbose = opts.verbose;
    options.quiet = opts.quiet;
    options.force = opts.force;
    options.undefined = opts.undefined;
    options.clean = opts.clean;

    if let Some(root) = opts.root.as_deref() {
        options.root = PathBuf::from(root).canonicalize()?;
    } else {
        options.root = std::env::current_dir()?;
    }

    if let Some(source) = opts.source.as_deref() {
        options.source = PathBuf::from(source).canonicalize()?;
    } else {
        options.source = options.root.join("source");
    }

    if let Some(build) = opts.build.as_deref() {
        options.build = PathBuf::from(build).canonicalize()?;
    } else {
        options.build = options.root.join("build");
    }

    if let Some(pattern) = opts.pattern.as_deref() {
        options.pattern = PathBuf::from(pattern).canonicalize()?;
    } else {
        options.pattern = options.root.join("pattern");
    }

    Ok(options)
}
