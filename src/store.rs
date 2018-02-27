use lmdb;

use config::Config;
use error::Error;
use txn::Txn;
use std::fs;
use std::collections::HashMap;

/// A Store is used to keep data on disk using LMDB
pub struct Store {
    env: lmdb::Environment,
    buckets: HashMap<String, Bucket>,

    /// The `config` field stores the initial configuration values for the given store
    pub cfg: Config
}

/// A Bucket represents a single database, or section of the Store
pub struct Bucket(lmdb::Database);

impl Bucket {
    /// Provides access to the underlying LMDB dbi handle
    pub fn db(&self) -> lmdb::Database {
        self.0
    }
}

impl Store {
    /// Create a new store with the given configuration
    pub fn new(mut config: Config) -> Result<Store, Error> {
        let _ = fs::create_dir_all(&config.path);
        let mut builder = lmdb::Environment::new();

        if config.readonly {
            config.flags.insert(lmdb::READ_ONLY)
        }

        let env = builder
            .set_flags(config.flags)
            .set_max_readers(config.max_readers)
            .set_max_dbs((config.buckets.len() + 1) as u32)
            .set_map_size(config.map_size)
            .open(config.path.as_path())?;

        let mut store = Store {
            env: env,
            buckets: HashMap::new(),
            cfg: config
        };

        for bucket in &store.cfg.buckets {
            let b = store.env.open_db(Some(bucket.as_ref()))?;
            store.buckets.insert(bucket.clone(), Bucket(b));
        }

        let default = store.env.open_db(None)?;
        store.buckets.insert(String::from("default"), Bucket(default));

        Ok(store)
    }

    /// Get the default bucket
    pub fn default(&self) -> Result<&Bucket, Error> {
        self.bucket("default")
    }

    /// Get a named bucket
    pub fn bucket<S: AsRef<str>>(&self, name: S) -> Result<&Bucket, Error> {
        let s = String::from(name.as_ref());
        match self.buckets.get(&s) {
            Some(ref bucket) => Ok(bucket),
            None => Err(Error::InvalidBucket)
        }
    }

    #[inline]
    /// Open a readonly transaction
    pub fn read_txn<'env>(&'env self) -> Result<Txn<'env>, Error> {
        let txn = self.env.begin_ro_txn()?;
        Ok(Txn::read_only(txn))
    }

    #[inline]
    /// Open a writable transaction
    pub fn write_txn<'env>(&'env self) -> Result<Txn<'env>, Error> {
        if self.cfg.readonly {
            return Err(Error::ReadOnly)
        }

        let txn = self.env.begin_rw_txn()?;
        Ok(Txn::read_write(txn))
    }

    #[inline]
    /// Sync data to disk
    pub fn sync(&self, force: bool) -> Result<(), Error> {
        Ok(self.env.sync(force)?)
    }
}


