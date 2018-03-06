use std::marker::PhantomData;

use lmdb;

use config::Config;
use error::Error;
use txn::Txn;
use std::collections::HashMap;
use types::{Integer, Key, Value};

/// A Store is used to keep data on disk using LMDB
pub struct Store {
    env: lmdb::Environment,
    buckets: HashMap<String, u32>,

    /// The `config` field stores the initial configuration values for the given store
    pub cfg: Config,
}

/// A Bucket represents a single database, or section of the Store
pub struct Bucket<'a, K: Key, V: 'a + Value<'a>>(lmdb::Database, PhantomData<K>, PhantomData<&'a V>);

impl <'a, K: Key, V: Value<'a>> Bucket<'a, K, V> {
    /// Provides access to the underlying LMDB dbi handle
    pub fn db(&self) -> lmdb::Database {
        self.0
    }
}

impl Store {
    pub(crate) fn wrap(env: lmdb::Environment, config: Config) -> Store {
        Store {
            env: env,
            buckets: HashMap::new(),
            cfg: config,
        }
    }

    /// Create a new store with the given configuration
    pub fn new(mut config: Config) -> Result<Store, Error> {
        let env = config.env()?;
        Ok(Self::wrap(env, config))
    }

    /// Get the default bucket
    pub fn default_bucket<'a, K: Key, V: Value<'a>>(&self) -> Result<Bucket<'a, K, V>, Error> {
        match self.buckets.get("default") {
            Some(flags) => {
                let f = lmdb::DatabaseFlags::from_bits(*flags).unwrap();
                Ok(Bucket(self.env.create_db(None, f)?, PhantomData, PhantomData))
            },
            None => Ok(Bucket(self.env.create_db(None, lmdb::DatabaseFlags::empty())?, PhantomData, PhantomData)),
        }
    }

    /// Get a named bucket
    pub fn bucket<'a, S: AsRef<str>, K: Key, V: Value<'a>>(&self, name: S) -> Result<Bucket<'a, K, V>, Error> {
        match self.buckets.get(name.as_ref()) {
            Some(flags) => {
                let f = lmdb::DatabaseFlags::from_bits(*flags).unwrap();
                Ok(Bucket(self.env.create_db(Some(name.as_ref()), f)?, PhantomData, PhantomData))
            },
            None => Err(Error::InvalidBucket),
        }
    }

    /// Get the default bucket
    pub fn default_int_bucket<'a, V: Value<'a>>(&self) -> Result<Bucket<'a, Integer, V>, Error> {
        match self.buckets.get("default") {
            Some(flags) => {
                let mut f = lmdb::DatabaseFlags::from_bits(*flags).unwrap();
                f.insert(lmdb::DatabaseFlags::INTEGER_KEY);
                Ok(Bucket(self.env.create_db(None, f)?, PhantomData, PhantomData))
            },
            None => Ok(Bucket(self.env.create_db(None, lmdb::DatabaseFlags::INTEGER_KEY)?, PhantomData, PhantomData)),
        }
    }

    /// Get a named bucket
    pub fn int_bucket<'a, S: AsRef<str>, V: Value<'a>>(&self, name: S) -> Result<Bucket<'a, Integer, V>, Error> {
        match self.buckets.get(name.as_ref()) {
            Some(flags) => {
                let mut f = lmdb::DatabaseFlags::from_bits(*flags).unwrap();
                f.insert(lmdb::DatabaseFlags::INTEGER_KEY);
                Ok(Bucket(self.env.create_db(Some(name.as_ref()), f)?, PhantomData, PhantomData))
            },
            None => Err(Error::InvalidBucket),
        }
    }

    #[inline]
    /// Open a readonly transaction
    pub fn read_txn<'env, K: Key, V: Value<'env>>(&'env self) -> Result<Txn<'env, K, V>, Error> {
        let txn = self.env.begin_ro_txn()?;
        Ok(Txn::read_only(txn))
    }

    #[inline]
    /// Open a writable transaction
    pub fn write_txn<'env, K: Key, V: Value<'env>>(&'env self) -> Result<Txn<'env, K, V>, Error> {
        if self.cfg.readonly {
            return Err(Error::ReadOnly);
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
