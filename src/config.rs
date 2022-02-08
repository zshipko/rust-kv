use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::error::Error;
use serde::{Deserialize, Serialize};

/// Config is used to create a new store
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// The `path` field determines where the database will be created
    pub path: PathBuf,

    /// The `temporary` field specifies if the database will be destroyed on close
    #[serde(default)]
    pub temporary: bool,

    /// Enable compression by setting `use_compression` to true
    #[serde(default)]
    pub use_compression: bool,

    /// Specify the flush frequency
    #[serde(default)]
    pub flush_every_ms: Option<u64>,

    /// Specify the cache capacity in bytes
    #[serde(default)]
    pub cache_capacity: Option<u64>,

    /// Specify the segment size for compatibility
    #[serde(default)]
    pub segment_size: Option<usize>,
}

impl Config {
    /// Create a default configuration object
    pub fn new<P: AsRef<Path>>(p: P) -> Config {
        Config {
            path: p.as_ref().to_path_buf(),
            temporary: false,
            use_compression: false,
            flush_every_ms: None,
            cache_capacity: None,
            segment_size: None,
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

    /// Set compression field
    pub fn use_compression(mut self, use_compression: bool) -> Config {
        self.use_compression = use_compression;
        self
    }

    /// Toggle `temporary` value
    pub fn temporary(mut self, temporary: bool) -> Config {
        self.temporary = temporary;
        self
    }

    /// Set flush frequency
    pub fn flush_every_ms(mut self, ms: u64) -> Config {
        self.flush_every_ms = Some(ms);
        self
    }

    /// Set cache capacity
    pub fn cache_capacity(mut self, bytes: u64) -> Config {
        self.cache_capacity = Some(bytes);
        self
    }

    /// Set cache capacity
    pub fn segment_size(mut self, kb: usize) -> Config {
        self.segment_size = Some(kb);
        self
    }

    pub(crate) fn open(&mut self) -> Result<sled::Db, Error> {
        let config = sled::Config::new()
            .path(&self.path)
            .temporary(self.temporary)
            .flush_every_ms(self.flush_every_ms)
            .use_compression(self.use_compression);
        let config = if let Some(cache_capacity) = self.cache_capacity {
            config.cache_capacity(cache_capacity)
        } else {
            config
        };
        let config = if let Some(segment_size) = self.segment_size {
            // allow old database to work
            config.segment_size(segment_size)
        } else {
            config
        };
        let db = config.open()?;
        Ok(db)
    }
}
