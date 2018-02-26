use lmdb;

use config::Config;
use error::Error;
use txn::{RoTxn, RwTxn};
use std::fs;
use std::collections::HashMap;

pub struct Store {
    env: lmdb::Environment,
    buckets: HashMap<String, Bucket>,
    pub cfg: Config
}

pub struct Bucket(lmdb::Database);

impl Bucket {
    pub fn db(&self) -> lmdb::Database {
        self.0
    }
}

impl Store {
    pub fn new(config: Config) -> Result<Store, Error> {
        let _ = fs::create_dir_all(&config.path);
        let mut builder = lmdb::Environment::new();
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

    pub fn default(&self) -> Result<&Bucket, Error> {
        self.bucket("default")
    }

    pub fn bucket<S: AsRef<str>>(&self, name: S) -> Result<&Bucket, Error> {
        let s = String::from(name.as_ref());
        match self.buckets.get(&s) {
            Some(ref bucket) => Ok(bucket),
            None => Err(Error::InvalidBucket)
        }
    }


    #[inline]
    pub fn ro_txn<'env>(&'env self) -> Result<RoTxn<'env>, Error> {
        let txn = self.env.begin_ro_txn()?;
        Ok(RoTxn::new(txn))
    }

    #[inline]
    pub fn rw_txn<'env>(&'env self) -> Result<RwTxn<'env>, Error> {
        let txn = self.env.begin_rw_txn()?;
        Ok(RwTxn::new(txn))
    }

    #[inline]
    pub fn sync(&self, force: bool) -> Result<(), Error> {
        Ok(self.env.sync(force)?)
    }
}


