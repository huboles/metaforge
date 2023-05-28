use crate::{DirNode, MetaError};
use eyre::Result;
use rayon::prelude::*;
use std::fs;

impl<'a> DirNode<'a> {
    pub fn par_file(&mut self) -> Result<()> {
        self.files.par_iter_mut().for_each(|file| {
            file.merge(&self.global);
            match file.construct() {
                Ok(str) => {
                    fs::write(file.dest().unwrap(), str).unwrap();
                }
                Err(e) => {
                    // print a line to stderr about failure but continue with other files
                    if self.opts.force {
                        eprintln!("ignoring {}: {}", file.path.display(), e);
                    } else {
                        match *e {
                            MetaError::Ignored => {}
                            e => {
                                eprintln!("{}", file.path.display());
                                panic!("{}", e);
                            }
                        }
                    }
                }
            }
        });
        Ok(())
    }

    pub fn par_dir(&'a mut self) -> Result<()> {
        self.build_files()?;

        self.dirs.par_iter_mut().for_each(|dir| {
            dir.map(&self.global).unwrap();
            dir.build_dir().unwrap();
        });

        Ok(())
    }
}
