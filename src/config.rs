use lmdb;

use std::path::{PathBuf, Path};

pub struct Config {
    pub map_size: usize,
    pub max_readers: u32,
    pub flags: lmdb::EnvironmentFlags,
    pub path: PathBuf,
    pub buckets: Vec<String>,
    pub readonly: bool
}

impl Config {
    pub fn default<P: AsRef<Path>>(p: P) -> Config {
        Config {
            map_size: 1024 * 1024 * 1024,
            max_readers: 5,
            flags: lmdb::EnvironmentFlags::empty(),
            path: p.as_ref().to_path_buf(),
            buckets: Vec::new(),
            readonly: false
        }
    }

    pub fn set_map_size(&mut self, n: usize) {
        self.map_size = n
    }

    pub fn set_max_readers(&mut self, n: u32) {
        self.max_readers = n
    }

    pub fn set_flags(&mut self, f: lmdb::EnvironmentFlags) {
        self.flags = f
    }

    pub fn set_path<P: AsRef<Path>>(&mut self, p: P) {
        self.path = p.as_ref().to_path_buf()
    }

    pub fn bucket<S: AsRef<str>>(&mut self, name: S) {
        self.buckets.push(String::from(name.as_ref()));
    }

    pub fn readonly(&mut self, readonly: bool) {
        self.readonly = readonly;
    }
}

