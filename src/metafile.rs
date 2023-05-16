use crate::{build_metafile, parse_file, Options};
use color_eyre::{
    eyre::{bail, eyre},
    Result,
};
use std::collections::HashMap;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Default)]
pub struct Header {
    pub blank: bool,
    pub panic_default: bool,
    pub panic_undefined: bool,
    pub filetype: String,
    pub pandoc: bool,
}

impl Header {
    pub fn new() -> Self {
        Self {
            blank: false,
            panic_default: false,
            panic_undefined: false,
            filetype: String::from("html"),
            pandoc: true,
        }
    }
}

impl From<HashMap<String, String>> for Header {
    fn from(value: HashMap<String, String>) -> Self {
        let mut header = Header::new();
        for (key, val) in value.iter() {
            match &key[..] {
                "blank" => header.blank = val == "true",
                "panic_default" => header.panic_default = val == "true",
                "panic_undefined" => header.panic_undefined = val == "true",
                "pandoc" => header.pandoc = val == "true",
                "filetype" => header.filetype = val.to_string(),
                _ => continue,
            }
        }
        header
    }
}

#[derive(Debug, Clone)]
pub struct MetaFile<'a> {
    pub opts: &'a Options,
    pub path: PathBuf,
    pub header: Header,
    pub variables: HashMap<String, String>,
    pub arrays: HashMap<String, Vec<String>>,
    pub patterns: HashMap<String, String>,
    pub source: Vec<Src>,
}

impl<'a> MetaFile<'a> {
    pub fn build(path: PathBuf, opts: &'a Options) -> Result<Self> {
        let str = match fs::read_to_string(&path) {
            Ok(str) => str,
            Err(_) => bail!("{} does not exist", path.display()),
        };
        let mut metafile = parse_file(str, opts)?;
        metafile.path = path;
        Ok(metafile)
    }

    pub fn new(opts: &'a Options) -> Self {
        Self {
            opts,
            path: PathBuf::new(),
            header: Header::new(),
            variables: HashMap::new(),
            arrays: HashMap::new(),
            patterns: HashMap::new(),
            source: Vec::new(),
        }
    }

    pub fn dest(&self) -> Result<PathBuf> {
        let mut path = self
            .opts
            .build
            .join(self.path.strip_prefix(&self.opts.source)?);
        path.set_extension(&self.header.filetype);

        Ok(path)
    }

    pub fn name(&self) -> Result<String> {
        if self.path.starts_with(&self.opts.source) {
            // in source dir, we want the file name without the '.meta' extension
            let name: String = self
                .path
                .strip_prefix(&self.opts.source)?
                .components()
                .map(|x| {
                    x.as_os_str()
                        .to_string_lossy()
                        .to_string()
                        .replace(".meta", "")
                })
                .collect::<Vec<String>>()
                .join(".");
            Ok(name)
        } else if self.path.starts_with(&self.opts.pattern) {
            // in pattern dir, we want the parent dir
            let name = self.path.strip_prefix(&self.opts.pattern)?;
            let name = name
                .parent()
                .map(|s| s.to_string_lossy().to_string().replace('/', "."))
                .unwrap_or_default();
            Ok(name)
        } else {
            color_eyre::eyre::bail!("could not get name from: {}", self.path.display());
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
                    fs::write(file.dest()?, str)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_name() -> Result<()> {
        let mut opts = Options::new();

        opts.source = "/tmp/source".into();
        opts.build = "/tmp/build".into();
        opts.pattern = "/tmp/pattern".into();

        let src_path = PathBuf::from("/tmp/source/test/file.meta");
        let pat1_path = PathBuf::from("/tmp/pattern/base/test.meta");
        let pat2_path = PathBuf::from("/tmp/pattern/test/class/file.meta");

        let mut src = MetaFile::new(&opts);
        src.path = src_path;
        let mut pat1 = MetaFile::new(&opts);
        pat1.path = pat1_path;
        let mut pat2 = MetaFile::new(&opts);
        pat2.path = pat2_path;

        assert_eq!(src.name()?, "test.file");
        assert_eq!(pat1.name()?, "base");
        assert_eq!(pat2.name()?, "test.class");

        Ok(())
    }
}
