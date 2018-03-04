use std::marker::PhantomData;

use lmdb;

use config::Config;
use error::Error;
use txn::Txn;
use std::fs;
use std::collections::HashMap;
use types::{Key, Value};

/// A Store is used to keep data on disk using LMDB
pub struct Store<K> {
    env: lmdb::Environment,
    buckets: HashMap<String, Bucket<K>>,

    /// The `config` field stores the initial configuration values for the given store
    pub cfg: Config,

    _key: PhantomData<K>
}

/// A Bucket represents a single database, or section of the Store
pub struct Bucket<K>(lmdb::Database, PhantomData<K>);

impl <K: Key> Bucket<K> {
    /// Provides access to the underlying LMDB dbi handle
    pub fn db(&self) -> lmdb::Database {
        self.0
    }
}

impl <K: Key> Store<K> {
    /// Create a new store with the given configuration
    pub fn new(mut config: Config) -> Result<Store<K>, Error> {
        let _ = fs::create_dir_all(&config.path);
        let mut builder = lmdb::Environment::new();

        if config.readonly {
            config.flags.insert(lmdb::EnvironmentFlags::READ_ONLY)
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
            cfg: config,
            _key: PhantomData
        };

        for bucket in &store.cfg.buckets {
            let b = store.env.open_db(Some(bucket.as_ref()))?;
            store.buckets.insert(bucket.clone(), Bucket(b, PhantomData));
        }

        let default = store.env.open_db(None)?;
        store.buckets.insert(String::from("default"), Bucket(default, PhantomData));

        Ok(store)
    }

    /// Get the default bucket
    pub fn default(&self) -> Result<&Bucket<K>, Error> {
        self.bucket("default")
    }

    /// Get a named bucket
    pub fn bucket<S: AsRef<str>>(&self, name: S) -> Result<&Bucket<K>, Error> {
        let s = String::from(name.as_ref());
        match self.buckets.get(&s) {
            Some(ref bucket) => Ok(bucket),
            None => Err(Error::InvalidBucket)
        }
    }

    #[inline]
    /// Open a readonly transaction
    pub fn read_txn<'env, V: Value<'env>>(&'env self) -> Result<Txn<'env, V>, Error> {
        let txn = self.env.begin_ro_txn()?;
        Ok(Txn::read_only(txn))
    }

    #[inline]
    /// Open a writable transaction
    pub fn write_txn<'env, V: Value<'env>>(&'env self) -> Result<Txn<'env, V>, Error> {
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


