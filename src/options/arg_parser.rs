use clap::Parser;

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
    pub source: Option<String>,
    /// Build directory [CURRENT_DIR/build]
    #[arg(short, long, value_name = "BUILD_DIR")]
    pub build: Option<String>,
    /// Pattern directory [CURRENT_DIR/pattern]
    #[arg(short, long, value_name = "PATTERN_DIR")]
    pub pattern: Option<String>,
    /// Enable extra output.
    /// Repeated flags give more info
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Minimal output
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,
    /// Don't stop on file failure [FALSE]
    #[arg(long, default_value_t = false)]
    pub force: bool,
    /// Stop on undefined variables and arrays [FALSE]
    #[arg(long, default_value_t = false)]
    pub undefined: bool,
    /// Clean build directory before building site [FALSE]
    #[arg(long, default_value_t = false)]
    pub clean: bool,
    /// Don't convert markdown to html.
    /// Runs even if pandoc isn't installed [FALSE]
    #[arg(long, default_value_t = false)]
    pub no_pandoc: bool,
}
