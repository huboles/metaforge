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

    eprintln!("{}", file);
    assert_eq!(file + "\n", POST_EXPAND);

    Ok(())
}

fn build_rootdir() -> Result<RootDirs> {
    let dir = PathBuf::from("./test_site");

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
