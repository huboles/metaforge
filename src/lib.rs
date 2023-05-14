extern crate pest;
#[macro_use]
extern crate pest_derive;

mod builder;
mod metafile;
mod options;
mod parser;

pub use builder::*;
pub use metafile::*;
pub use options::*;
pub use parser::*;

use clap::Parser;
use color_eyre::Result;

pub fn get_opts() -> Result<Options> {
    let opts = Options::try_from(Opts::parse())?;

    log!(
        opts,
        format!("cleaning build directory: {}", opts.build.display()),
        1
    );
    if opts.clean && opts.build.exists() {
        std::fs::remove_dir_all(&opts.build)?;
    }

    if !opts.build.exists() {
        std::fs::create_dir(&opts.build)?;
    }

    Ok(opts)
}

pub fn build_dir(opts: &Options) -> Result<()> {
    let mut source = DirNode::build(opts.source.clone(), opts)?;

    let global_init = MetaFile::new(&opts);

    source.map(&global_init)?;

    Ok(())
}
