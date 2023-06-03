use crate::{error::*, Options};
use eyre::Result;
use minify_html::{minify, Cfg};
use std::{fs, path::PathBuf};

use super::*;

const HTML_CFG: Cfg = Cfg {
    do_not_minify_doctype: false,
    ensure_spec_compliant_unquoted_attribute_values: false,
    keep_closing_tags: true,
    keep_html_and_head_opening_tags: true,
    keep_spaces_between_attributes: false,
    keep_comments: false,
    minify_css: true,
    minify_css_level_1: false,
    minify_css_level_2: true,
    minify_css_level_3: false,
    minify_js: true,
    remove_bangs: true,
    remove_processing_instructions: true,
};

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

            if self.global.header.copy_only {
                let dest = self.global.dest()?;
                fs::copy(file, &dest.parent().unwrap_or(&self.opts.build))?;
                continue;
            }

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
                    if file.header.minify && self.opts.minify {
                        fs::write(file.dest()?, minify(str.as_bytes(), &HTML_CFG))?;
                    } else {
                        fs::write(file.dest()?, str)?;
                    }
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
