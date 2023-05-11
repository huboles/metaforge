use color_eyre::Result;
use metaforge::{build_site, parse_opts};

fn main() -> Result<()> {
    color_eyre::install()?;

    let opts = parse_opts()?;

    build_site(&opts)
}
