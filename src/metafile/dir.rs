use crate::{MetaError, Options};
use eyre::Result;
use std::{fs, path::PathBuf};

use super::*;

#[derive(Debug, Clone)]
pub struct DirNode<'a> {
    path: PathBuf,
    opts: &'a Options,
    global: MetaFile<'a>,
    files: Vec<MetaFile<'a>>,
    dirs: Vec<DirNode<'a>>,
}

impl<'a> DirNode<'a> {
    pub fn build(path: PathBuf, opts: &'a Options) -> Result<Self> {
        assert!(path.is_dir() && path.exists());

        // copy over directory structure from source dir
        let build_dir = opts.build.join(path.strip_prefix(&opts.source)?);
        if !build_dir.exists() {
            fs::create_dir_all(build_dir)?;
        }

        let files: Vec<MetaFile> = Vec::new();
        let dirs: Vec<DirNode> = Vec::new();
        let global = MetaFile::new(opts);

        Ok(Self {
            path,
            opts,
            global,
            files,
            dirs,
        })
    }

    // parses all contained files and directories and pushes
    // parsed structures into the files and directories vectors
    pub fn map(&mut self, global: &'a MetaFile) -> Result<()> {
        for f in fs::read_dir(&self.path)? {
            let file = f?.path();

            if file.is_dir() {
                let dir = DirNode::build(file, self.opts)?;
                self.dirs.push(dir);
            } else if file.file_name().and_then(|f| f.to_str()) == Some("default.meta") {
                let mut new_global = MetaFile::build(file, self.opts)?;
                new_global.merge(global);
                self.global = new_global;
            } else if file.extension().and_then(|f| f.to_str()) == Some("meta") {
                let file = MetaFile::build(file, self.opts)?;
                self.files.push(file)
            }
        }

        Ok(())
    }

    pub fn build_files(&mut self) -> Result<()> {
        for file in self.files.iter_mut() {
            file.merge(&self.global);
            match file.construct() {
                Ok(str) => {
                    fs::write(file.dest()?, str)?;
                }
                Err(e) => {
                    // print a line to stderr about failure but continue with other files
                    if self.opts.force {
                        eprintln!("ignoring {}: {}", file.path.display(), e);
                        continue;
                    } else {
                        // we raise an ignored error to quickly abort any file parsing
                        if let MetaError::Ignored = *e {
                            continue;
                        // anything else gets wrapped up and passed up the calling chain
                        } else {
                            return Err(e.into());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn build_dir(&'a mut self) -> Result<()> {
        self.build_files()?;

        for dir in self.dirs.iter_mut() {
            dir.map(&self.global)?;
            dir.build_dir()?;
        }

        Ok(())
    }
}
