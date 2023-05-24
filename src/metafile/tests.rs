use crate::Options;
use eyre::Result;
use std::path::PathBuf;

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
    assert_eq!(pat1.name()?, "base.test");
    assert_eq!(pat2.name()?, "test.class.file");
    assert_eq!(pat1.class()?, "base");
    assert_eq!(pat2.class()?, "test.class");

    Ok(())
}
