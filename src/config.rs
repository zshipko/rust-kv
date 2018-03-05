use lmdb;

use std::path::{Path, PathBuf};

/// Config is used to create a new store
pub struct Config {
    /// The `map_size` field determines the maximum number of bytes stored in the database
    pub map_size: usize,

    /// The `max_readers` field determines the maximum number of readers for a given database
    pub max_readers: u32,

    /// The `flags` field contains raw LMDB flags
    pub flags: lmdb::EnvironmentFlags,

    /// The `path` field determines where the database will be created
    pub path: PathBuf,

    /// The `buckets` field whitelists the named buckets
    pub buckets: Vec<String>,

    /// Readonly sets the MDB_RDONLY flag when opening the database
    pub readonly: bool,

    /// Flags used when creating a new Bucket
    pub database_flags: lmdb::DatabaseFlags,
}

impl Config {
    /// Create a default configuration object
    pub fn default<P: AsRef<Path>>(p: P) -> Config {
        Config {
            map_size: 1024 * 1024 * 1024,
            max_readers: 5,
            flags: lmdb::EnvironmentFlags::empty(),
            path: p.as_ref().to_path_buf(),
            buckets: Vec::new(),
            readonly: false,
            database_flags: lmdb::DatabaseFlags::empty(),
        }
    }

    /// Set `map_size` field
    pub fn set_map_size(&mut self, n: usize) -> &mut Config {
        self.map_size = n;
        self
    }

    /// Set `max_readers` field
    pub fn set_max_readers(&mut self, n: u32) -> &mut Config {
        self.max_readers = n;
        self
    }

    /// Set `flags` field
    pub fn set_flags(&mut self, f: lmdb::EnvironmentFlags) -> &mut Config {
        self.flags = f;
        self
    }

    /// Set `path` field
    pub fn set_path<P: AsRef<Path>>(&mut self, p: P) -> &mut Config {
        self.path = p.as_ref().to_path_buf();
        self
    }

    /// Add a bucket
    pub fn bucket<S: AsRef<str>>(&mut self, name: S) -> &mut Config {
        self.buckets.push(String::from(name.as_ref()));
        self
    }

    /// Set to readonly
    pub fn readonly(&mut self, readonly: bool) -> &mut Config {
        self.readonly = readonly;
        self
    }
}
