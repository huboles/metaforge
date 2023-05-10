use color_eyre::{eyre::bail, Result};
use metaforge::{build_metafile, parse_opts};
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> Result<()> {
    color_eyre::install()?;

    let opts = parse_opts()?;

    let files: Vec<PathBuf> = WalkDir::new(&opts.source)
        .into_iter()
        .filter_map(|file| file.ok())
        .filter(|file| file.file_type().is_file())
        .map(|file| file.into_path())
        .collect();

    for file in files.iter() {
        match build_metafile(file, &opts) {
            Ok(_) => continue,
            Err(e) => {
                if opts.force {
                    // print a line to stderr about failure but continue with other files
                    eprintln!("error in {}: {}", file.to_string_lossy(), e);
                    continue;
                } else {
                    bail!("error in {}: {}", file.to_string_lossy(), e);
                }
            }
        }
    }

    Ok(())
}
