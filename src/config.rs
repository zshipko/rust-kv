use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

use lmdb;
use toml;

/// Database Flags
pub use lmdb::DatabaseFlags;

use error::Error;

/// Bucket flag, used when creating/opening a bucket
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Flag {
    Default,
    ReverseKey,
    IntegerKey,
}

impl Flag {
    pub(crate) fn database_flags(&self) -> lmdb::DatabaseFlags {
        match self {
            Flag::ReverseKey => DatabaseFlags::REVERSE_KEY,
            Flag::IntegerKey => DatabaseFlags::INTEGER_KEY,
            Flag::Default => DatabaseFlags::empty(),
        }
    }
}

/// Config is used to create a new store
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// The `map_size` field determines the maximum number of bytes stored in the database
    pub map_size: usize,

    /// The `max_readers` field determines the maximum number of readers for a given database
    pub max_readers: u32,

    flags: u32,

    /// The `path` field determines where the database will be created
    pub path: PathBuf,

    /// Readonly sets the MDB_RDONLY flag when opening the database
    pub readonly: bool,

    /// Whitelisted buckets and DatabaseFlags
    pub buckets: HashMap<String, Flag>,
}

impl Config {
    /// Create a default configuration object
    pub fn default<P: AsRef<Path>>(p: P) -> Config {
        Config {
            map_size: 1024 * 1024 * 1024,
            max_readers: 5,
            flags: lmdb::EnvironmentFlags::empty().bits(),
            path: p.as_ref().to_path_buf(),
            readonly: false,
            buckets: HashMap::new(),
        }
    }

    /// Save Config to an io::Write
    pub fn save_to<W: io::Write>(&self, mut w: W) -> Result<(), Error> {
        let s = match toml::to_string(self) {
            Ok(s) => s,
            Err(_) => return Err(Error::InvalidConfiguration),
        };
        w.write_all(s.as_ref())?;
        Ok(())
    }

    /// Save Config to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let file = fs::File::create(path.as_ref())?;
        self.save_to(file)
    }

    /// Load configuration from an io::Read
    pub fn load_from<R: io::Read>(mut r: R) -> Result<Config, Error> {
        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;
        match toml::from_slice(buf.as_ref()) {
            Ok(cfg) => Ok(cfg),
            Err(_) => Err(Error::InvalidConfiguration),
        }
    }

    /// Load configuration to a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let file = fs::File::open(path.as_ref())?;
        Self::load_from(file)
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

    /// Get `flags` field (DatabaseFlags)
    pub fn flags(&self) -> lmdb::EnvironmentFlags {
        lmdb::EnvironmentFlags::from_bits(self.flags).unwrap()
    }

    /// Set `flags` field (DatabaseFlags)
    pub fn flag(&mut self, f: lmdb::EnvironmentFlags) -> &mut Config {
        let mut flags = self.flags();
        flags.insert(f);
        self.flags = f.bits();
        self
    }

    /// Set `path` field
    pub fn set_path<P: AsRef<Path>>(&mut self, p: P) -> &mut Config {
        self.path = p.as_ref().to_path_buf();
        self
    }

    /// Add a bucket
    pub fn bucket<S: AsRef<str>>(&mut self, name: S, f: Option<Flag>) -> &mut Config {
        self.buckets
            .insert(String::from(name.as_ref()), f.unwrap_or(Flag::Default));
        self
    }

    /// Set to readonly
    pub fn readonly(&mut self, readonly: bool) -> &mut Config {
        self.readonly = readonly;
        self
    }

    pub(crate) fn env(&mut self) -> Result<lmdb::Environment, Error> {
        let mut builder = lmdb::Environment::new();

        let mut flags = self.flags();

        if self.readonly {
            flags.insert(lmdb::EnvironmentFlags::READ_ONLY)
        }

        let _ = fs::create_dir_all(&self.path);

        match builder
            .set_flags(flags)
            .set_max_readers(self.max_readers)
            .set_max_dbs((self.buckets.len() + 1) as u32)
            .set_map_size(self.map_size)
            .open(self.path.as_path())
        {
            Ok(db) => Ok(db),
            Err(e) => {
                let _ = fs::remove_dir_all(&self.path);
                Err(Error::LMDB(e))
            }
        }
    }
}
