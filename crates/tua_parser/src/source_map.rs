use crate::syntax_pos::BytePos;
use once_cell::sync::Lazy;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};
use url::Url;

static CURRENT_DIR: Lazy<Option<PathBuf>> = Lazy::new(|| env::current_dir().ok());

/// An abstraction over the fs operations used by the Parser.
pub trait FileLoader {
    /// Query the existence of a file.
    fn file_exists(&self, path: &Path) -> bool;

    /// Return an absolute path to a file, if possible.
    fn abs_path(&self, path: &Path) -> Option<PathBuf>;

    /// Read the contents of an UTF-8 file into memory.
    fn read_file(&self, path: &Path) -> io::Result<String>;
}

pub struct RealFileLoader;

impl FileLoader for RealFileLoader {
    fn file_exists(&self, path: &Path) -> bool {
        fs::metadata(path).is_ok()
    }

    fn abs_path(&self, path: &Path) -> Option<PathBuf> {
        if path.is_absolute() {
            Some(path.to_path_buf())
        } else {
            CURRENT_DIR.as_ref().map(|cwd| cwd.join(path))
        }
    }

    fn read_file(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub enum FileName {
    Real(PathBuf),
    /// Command line
    Anon,
    Url(Url),
    Internal(String),
}

#[derive(Clone)]
pub struct SourceFile {
    /// The name of the file that the source came from. Source that doesn't
    /// originate from files has names between angle brackets by convention,
    /// e.g. `<anon>`
    pub name: FileName,
    /// Indicates which crate this `SourceFile` was imported from.
    pub crate_of_origin: u32,
    /// The complete source code
    pub src: Lrc<String>,
    /// The source code's hash
    pub src_hash: u128,
    /// The start position of this source in the `SourceMap`
    pub start_pos: BytePos,
    /// The end position of this source in the `SourceMap`
    pub end_pos: BytePos,
    /// Locations of lines beginnings in the source code
    pub lines: Vec<BytePos>,
    /// A hash of the filename, used for speeding up the incr. comp. hashing.
    pub name_hash: u128,
}
