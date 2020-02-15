use std::path::Path;

use crate::{Bucket, Config, Error, Key, Value};

/// Store is used to read/write data to disk using `sled`
pub struct Store {
    config: Config,
    db: sled::Db,
}

impl Store {
    /// Create a new store from the given config
    pub fn new(mut config: Config) -> Result<Store, Error> {
        Ok(Store {
            db: config.open()?,
            config,
        })
    }

    /// Get the store's path
    pub fn path(&self) -> Result<&Path, Error> {
        Ok(self.config.path.as_path())
    }

    /// Open a new bucket
    pub fn bucket<'a, K: Key<'a>, V: Value>(
        &self,
        name: Option<&str>,
    ) -> Result<Bucket<'a, K, V>, Error> {
        let t = self.db.open_tree(name.unwrap_or("__sled__default"))?;
        Ok(Bucket::new(t))
    }

    /// Remove a bucket from the store
    pub fn drop_bucket<S: AsRef<str>>(&self, name: S) -> Result<(), Error> {
        self.db.drop_tree(name.as_ref().as_bytes())?;
        Ok(())
    }

    /// Returns the size on disk in bytes
    pub fn size_on_disk(&self) -> Result<u64, Error> {
        let i = self.db.size_on_disk()?;
        Ok(i)
    }
}
