extern crate pest;
#[macro_use]
extern crate pest_derive;

mod builder;
mod error;
mod metafile;
mod options;
mod parser;

pub use builder::*;
pub use error::*;
pub use metafile::*;
pub use options::*;
pub use parser::*;

use clap::Parser;
use eyre::Result;
use std::fs;

pub fn get_opts() -> Result<Options> {
    let opts = Options::try_from(Opts::parse())?;

    let exists = opts.build.exists();
    if exists && opts.clean {
        std::fs::remove_dir_all(&opts.build)?;
        std::fs::create_dir(&opts.build)?;
    } else if !exists {
        std::fs::create_dir(&opts.build)?;
    }

    Ok(opts)
}

pub fn build_dir(opts: &Options) -> Result<()> {
    let mut source = DirNode::build(opts.source.clone(), opts)?;

    let global_init = MetaFile::new(opts);

    source.map(&global_init)?;

    source.build_dir()
}

pub fn new_site(opts: &Options) -> Result<()> {
    macro_rules! exist_or_build(
        ($p:expr) => {
            if !$p.exists() {
                fs::create_dir_all(&$p)?;
            }
        };
    );

    macro_rules! write_default (
        ($p:expr, $m:literal) => {
            let path = opts.pattern.join($p).join("default.meta");
            fs::write(path, $m)?;
        };
    );

    exist_or_build!(opts.root);
    exist_or_build!(opts.source);
    exist_or_build!(opts.pattern);

    exist_or_build!(opts.pattern.join("base"));
    exist_or_build!(opts.pattern.join("body"));
    exist_or_build!(opts.pattern.join("head"));
    exist_or_build!(opts.pattern.join("foot"));

    write_default!("base", "<html>\n&{head}\n&{body}\n&{foot}\n</html>\n");
    write_default!("body", "<body>\n&{SOURCE}\n</body>\n");
    write_default!("head", "<head>\n<title>HELLO WORLD</title>\n</head>\n");
    write_default!("foot", "<foot>\n${footer}\n</foot>\n");

    let path = opts.source.join("hello_world.meta");
    fs::write(path, "${ footer = 'made using metaforge' }\n# it works\ncall `metaforge -h` for help, or read the [readme](https://huck.website/code/metaforge)\n")?;

    Ok(())
}
