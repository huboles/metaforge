mod node;
mod parallel;

pub use node::*;
pub use parallel::*;

use crate::Options;
use std::path::PathBuf;

use super::*;

#[derive(Debug, Clone)]
pub struct DirNode<'a> {
    pub path: PathBuf,
    pub opts: &'a Options,
    pub global: MetaFile<'a>,
    pub files: Vec<MetaFile<'a>>,
    pub dirs: Vec<DirNode<'a>>,
}
