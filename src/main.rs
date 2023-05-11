use color_eyre::{eyre::eyre, Result};
use metaforge::{build_metafile, log, parse_opts};
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> Result<()> {
    color_eyre::install()?;

    let opts = parse_opts()?;

    log!(opts, "finding files", 2);
    let files: Vec<PathBuf> = WalkDir::new(&opts.source)
        .into_iter()
        .filter_map(|file| {
            if file.as_ref().ok()?.file_type().is_dir() {
                // need to create directories in build dir
                let path = file.unwrap().into_path();
                let path = path.strip_prefix(&opts.source).ok()?;
                let path = opts.build.join(path);
                log!(opts, format!("\tcreating dir: {}", path.display()), 3);
                std::fs::create_dir(path).ok()?;
                // don't need them for any further operations so we filter them out
                None
            } else {
                if let Ok(file) = file {
                    log!(opts, format!("\tadding file: {}", file.path().display()), 3);
                    Some(file.into_path())
                } else {
                    None
                }
            }
        })
        .collect();

    log!(opts, "building files", 2);
    for file in files.iter() {
        match build_metafile(file, &opts) {
            Ok(_) => continue,
            Err(e) => {
                if opts.force {
                    // print a line to stderr about failure but continue with other files
                    eprintln!("{}: {}", file.display(), e);
                    continue;
                } else {
                    return Err(e.wrap_err(eyre!("{}:", file.display())));
                }
            }
        }
    }

    Ok(())
}
