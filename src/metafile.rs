use crate::{build_metafile, parse_file, write_file, Options};
use color_eyre::{eyre::eyre, Result};
use std::collections::HashMap;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct MetaFile<'a> {
    pub opts: &'a Options,
    pub path: PathBuf,
    pub header: HashMap<String, String>,
    pub variables: HashMap<String, String>,
    pub arrays: HashMap<String, Vec<String>>,
    pub patterns: HashMap<String, String>,
    pub source: Vec<Src>,
}

impl<'a> MetaFile<'a> {
    pub fn build(path: PathBuf, opts: &'a Options) -> Result<Self> {
        let str = fs::read_to_string(&path)?;
        let mut metafile = parse_file(str, opts)?;
        metafile.path = path;
        Ok(metafile)
    }

    pub fn new(opts: &'a Options) -> Self {
        Self {
            opts,
            path: PathBuf::new(),
            header: HashMap::new(),
            variables: HashMap::new(),
            arrays: HashMap::new(),
            patterns: HashMap::new(),
            source: Vec::new(),
        }
    }

    pub fn name(&self) -> Result<String> {
        if self.path.starts_with(&self.opts.source) {
            // in source dir, we want the file name
            let name = self.path.strip_prefix(&self.opts.source)?;
            let name = name.to_string_lossy().to_string().replace('/', ".");
            Ok(name)
        } else {
            // in pattern dir, we want the parent dir
            let name = self.path.strip_prefix(&self.opts.pattern)?;
            let name = name
                .parent()
                .unwrap()
                .to_string_lossy()
                .to_string()
                .replace('/', ".");
            Ok(name)
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
    (var($s:expr)) => { Src::Sub(Sub::Var($s.to_string()))};
    (arr($s:expr)) => { Src::Sub(Sub::Arr($s.to_string()))};
    (pat($s:expr)) => { Src::Sub(Sub::Pat($s.to_string()))};
    ($s:expr) => { Src::Str($s.to_string())};
);

#[derive(Debug, Clone, PartialEq)]
pub enum Src {
    Str(String),
    Sub(Sub),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sub {
    Var(String),
    Arr(String),
    Pat(String),
}

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

        let build_dir = opts.build.join(path.strip_prefix(&opts.source)?);
        if !build_dir.exists() {
            fs::create_dir(build_dir)?;
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
            match build_metafile(file) {
                Ok(str) => {
                    write_file(&file.path, str, file.opts)?;
                }
                Err(e) => {
                    if self.opts.force {
                        // print a line to stderr about failure but continue with other files
                        eprintln!("ignoring {}: {}", file.path.display(), e);
                        continue;
                    } else {
                        return Err(e.wrap_err(eyre!("{}:", file.path.display())));
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
