use color_eyre::Result;
use metaforge::*;
use pretty_assertions::assert_eq;
use std::{fs, path::PathBuf};

static PRE_EXPAND: &str = include_str!("../test_site/source/expand.meta");
static POST_EXPAND: &str = include_str!("files/expanded");

#[test]
fn test_metafile_to_str() -> Result<()> {
    let metafile = parse_file(PRE_EXPAND)?;
    let dirs = build_options()?;

    let file = metafile_to_string(&metafile, &dirs, None)?;

    // requires newline to match with files ending newline
    assert_eq!(file + "\n", POST_EXPAND);

    Ok(())
}

#[test]
fn test_metafile_builder() -> Result<()> {
    let dirs = build_options()?;
    let path = PathBuf::from("test_site/source/expand_html.meta");
    build_metafile(&path, &dirs)?;
    let source = fs::read_to_string("test_site/build/expand_html.html")?;
    let html = fs::read_to_string("tests/files/expanded_html")?;

    assert_eq!(source, html);

    Ok(())
}

fn build_options() -> Result<Options> {
    let dir = PathBuf::from("./test_site").canonicalize()?;

    let opts = Options {
        root: dir.clone(),
        source: dir.join("source"),
        build: dir.join("build"),
        pattern: dir.join("pattern"),
        verbose: 0,
        quiet: false,
        force: false,
        undefined: false,
    };

    if !opts.build.exists() {
        fs::create_dir(&opts.build)?;
    }

    Ok(opts)
}
