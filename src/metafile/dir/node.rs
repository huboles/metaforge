use crate::{error::*, Options};
use eyre::Result;
use std::{fs, path::PathBuf};

use super::*;

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
        if self.path.join("default.meta").exists() {
            if let Some(mut new_global) = check_ignore(MetaFile::build(
                self.path.clone().join("default.meta"),
                self.opts,
            ))? {
                new_global.merge(global);
                self.global = new_global;
            }
        }

        for f in fs::read_dir(&self.path)? {
            let file = f?.path();

            if file.is_dir() {
                let dir = DirNode::build(file, self.opts)?;
                self.dirs.push(dir);
            } else if file.file_name().and_then(|f| f.to_str()) == Some("default.meta") {
                continue;
            } else if file.extension().and_then(|f| f.to_str()) == Some("meta") {
                if let Some(file) = check_ignore(MetaFile::build(file, self.opts))? {
                    self.files.push(file)
                }
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
                        match *e {
                            MetaError::Ignored => continue,
                            e => {
                                eprintln!("{}", file.path.display());
                                return Err(e.into());
                            }
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
