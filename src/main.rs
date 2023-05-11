use color_eyre::Result;
use metaforge::{build_site, log, parse_opts};

fn main() -> Result<()> {
    color_eyre::install()?;

    let opts = parse_opts()?;

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

    build_site(&opts)
}
