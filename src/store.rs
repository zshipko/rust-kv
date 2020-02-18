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

    /// Generate monotonic ID
    pub fn generate_id(&self) -> Result<u64, Error> {
        let id = self.db.generate_id()?;
        Ok(id)
    }

    /// Get a list of bucket names
    pub fn buckets(&self) -> Vec<String> {
        self.db
            .tree_names()
            .into_iter()
            .map(|x| String::from_utf8(x.to_vec()))
            .filter_map(|x| match x {
                Ok(x) => Some(x),
                Err(_) => None,
            })
            .collect()
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

    /// Export entire database
    pub fn export(&self) -> Vec<(Vec<u8>, Vec<u8>, impl Iterator<Item = Vec<Vec<u8>>>)> {
        self.db.export()
    }

    /// Import from database export
    pub fn import(&self, export: Vec<(Vec<u8>, Vec<u8>, impl Iterator<Item = Vec<Vec<u8>>>)>) {
        self.db.import(export)
    }
}
