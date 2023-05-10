use color_eyre::Result;
use metaforge::*;
use pretty_assertions::assert_eq;
use std::{fs, path::PathBuf};

static PRE_EXPAND: &str = include_str!("../test_site/source/expand.meta");
static POST_EXPAND: &str = include_str!("./files/expanded");

#[test]
fn test_metafile_to_str() -> Result<()> {
    let metafile = parse_file(PRE_EXPAND)?;
    let dirs = build_rootdir()?;

    let file = metafile_to_string(&metafile, &dirs, None)?;

    // requires newline to match with files ending newline
    assert_eq!(file + "\n", POST_EXPAND);

    Ok(())
}

#[test]
#[ignore = "use different source file than expanded"]
fn test_metafile_builder() -> Result<()> {
    let dirs = build_rootdir()?;
    let path = PathBuf::from("test_site/source/expand_html.meta");
    build_metafile(&path, &dirs)?;
    let source = fs::read_to_string("../test_site/site/expand_html.html")?;
    let html = fs::read_to_string("./files/expand_html")?;

    assert_eq!(source, html);

    Ok(())
}

fn build_rootdir() -> Result<RootDirs> {
    let dir = PathBuf::from("./test_site").canonicalize()?;

    let dirs = RootDirs {
        source: dir.join("source"),
        build: dir.join("site"),
        pattern: dir.join("pattern"),
    };

    if dirs.build.exists() {
        fs::remove_dir(&dirs.build)?;
    }
    fs::create_dir(&dirs.build)?;

    Ok(dirs)
}
