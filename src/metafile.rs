use crate::{build_metafile, parse_file, Options};
use color_eyre::{eyre::eyre, Result};
use std::collections::HashMap;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct MetaFile {
    pub path: PathBuf,
    pub variables: HashMap<String, String>,
    pub arrays: HashMap<String, Vec<String>>,
    pub patterns: HashMap<String, String>,
    pub source: Vec<Source>,
}

impl MetaFile {
    pub fn build(path: PathBuf) -> Result<Self> {
        let str = fs::read_to_string(&path)?;
        let mut metafile = MetaFile::build_from_string(str)?;
        metafile.path = path.to_path_buf();
        Ok(metafile)
    }

    fn build_from_string(string: String) -> Result<Self> {
        let metafile = parse_file(string)?;
        Ok(metafile)
    }

    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
            variables: HashMap::new(),
            arrays: HashMap::new(),
            patterns: HashMap::new(),
            source: Vec::new(),
        }
    }

    pub fn get_var(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn get_arr(&self, key: &str) -> Option<&[String]> {
        self.arrays.get(key).map(|a| &a[..])
    }

    pub fn get_pat(&self, key: &str) -> Option<&String> {
        self.patterns.get(key)
    }

    pub fn merge(&mut self, other: &Self) {
        for (key, val) in other.variables.iter() {
            match self.variables.get(key) {
                Some(_) => continue,
                None => self.variables.insert(key.to_string(), val.to_string()),
            };
        }
        for (key, val) in other.arrays.iter() {
            match self.arrays.get(key) {
                Some(_) => continue,
                None => self.arrays.insert(key.to_string(), val.to_vec()),
            };
        }
        for (key, val) in other.patterns.iter() {
            match self.patterns.get(key) {
                Some(_) => continue,
                None => self.patterns.insert(key.to_string(), val.to_string()),
            };
        }
    }
}

#[macro_export]
macro_rules! source (
    (var($s:expr)) => { Source::Sub(Substitution::Variable($s.to_string()))};
    (arr($s:expr)) => { Source::Sub(Substitution::Array($s.to_string()))};
    (pat($s:expr)) => { Source::Sub(Substitution::Pattern($s.to_string()))};
    ($s:expr) => { Source::Str($s.to_string())};
);

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    Str(String),
    Sub(Substitution),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Substitution {
    Variable(String),
    Array(String),
    Pattern(String),
}

pub struct DirNode<'a> {
    path: PathBuf,
    opts: &'a Options,
    global: MetaFile,
    files: Vec<MetaFile>,
    dirs: Vec<DirNode<'a>>,
}

impl<'a> DirNode<'a> {
    pub fn build(path: PathBuf, opts: &'a Options) -> Result<Self> {
        assert!(path.is_dir() && path.exists());

        fs::create_dir(opts.build.join(path.strip_prefix(&opts.source)?))?;

        let files: Vec<MetaFile> = Vec::new();
        let dirs: Vec<DirNode> = Vec::new();
        let global = MetaFile::new();

        Ok(Self {
            path: path.to_path_buf(),
            opts,
            global,
            files,
            dirs,
        })
    }

    // parses all contained files and directories and pushes
    // parsed structures into the files and directories vectors
    pub fn map(&mut self, global: &'a MetaFile) -> Result<()> {
        for f in fs::read_dir(&self.path)?.into_iter() {
            let file = f?.path();

            if file.is_dir() {
                let dir = DirNode::build(file, self.opts)?;
                self.dirs.push(dir);
            } else if file.file_name().and_then(|f| f.to_str()) == Some("default.meta") {
                let mut new_global = MetaFile::build(file)?;
                new_global.merge(global);
                self.global = new_global;
            } else if file.extension().and_then(|f| f.to_str()) == Some("meta") {
                let file = MetaFile::build(file)?;
                self.files.push(file)
            }
        }

        Ok(())
    }

    pub fn build_files(&mut self) -> Result<()> {
        for file in self.files.iter_mut() {
            file.merge(&self.global);
            if let Err(e) = build_metafile(file, self.opts) {
                if self.opts.force {
                    // print a line to stderr about failure but continue with other files
                    eprintln!("ignoring {}: {}", file.path.display(), e);
                    continue;
                } else {
                    return Err(e.wrap_err(eyre!("{}:", file.path.display())));
                }
            }
        }
        Ok(())
    }

    pub fn build_dir(&mut self) -> Result<()> {
        self.build_files()?;

        for dir in self.dirs.iter_mut() {
            dir.build_dir()?;
        }

        Ok(())
    }
}
