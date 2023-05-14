use clap::Parser;
use color_eyre::Result;
use metaforge::{log, Options, Opts};

fn main() -> Result<()> {
    color_eyre::install()?;

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

    todo!("implement DirNode chain")
}
