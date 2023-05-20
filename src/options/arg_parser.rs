use clap::Parser;

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
