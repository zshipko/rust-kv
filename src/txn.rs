use std::marker::PhantomData;

use lmdb;
use lmdb::Transaction;

use error::Error;
use store::Bucket;
use cursor::Cursor;
use types::{Key, Value};

pub enum Txn<'env, V> {
    ReadOnly(lmdb::RoTransaction<'env>),
    ReadWrite(lmdb::RwTransaction<'env>),
    Phantom(PhantomData<V>)
}

impl <'env, V: Value<'env>> Txn<'env, V> {
    /// Returns true when the transaction is ReadOnly
    pub fn is_read_only(&self) -> bool {
        match self {
            &Txn::ReadOnly(_) => true,
            &Txn::ReadWrite(_) => false,
            &Txn::Phantom(_) => unreachable!()
        }
    }

    pub(crate) fn read_only(t: lmdb::RoTransaction<'env>) -> Txn<'env, V> {
        Txn::ReadOnly(t)
    }

    pub(crate) fn read_write(t: lmdb::RwTransaction<'env>) -> Txn<'env, V> {
        Txn::ReadWrite(t)
    }

    /// Ends the transaction, saving all changes
    pub fn commit(self) -> Result<(), Error> {
        match self {
            Txn::ReadOnly(txn) => Ok(txn.commit()?),
            Txn::ReadWrite(txn) => Ok(txn.commit()?),
            Txn::Phantom(_) => unreachable!()
        }
    }

    /// Ends the transaction, discarding all changes
    pub fn abort(self) {
        match self {
            Txn::ReadOnly(txn) => txn.abort(),
            Txn::ReadWrite(txn) => txn.abort(),
            Txn::Phantom(_) => unreachable!()
        }
    }

    /// Gets the value associated with the given key
    pub fn get<K: Key>(&'env self, bucket: &Bucket<K>, key: K) -> Result<V, Error> {
        match self {
            &Txn::ReadOnly(ref txn) => Ok(V::from_raw(txn.get(bucket.db(), &key)?)),
            &Txn::ReadWrite(ref txn) => Ok(V::from_raw(txn.get(bucket.db(), &key)?)),
            &Txn::Phantom(_) => unreachable!()
        }
    }

    /// Sets the value associated with the given key
    pub fn set<'a, K: Key>(&mut self, bucket: &Bucket<K>, key: K, val: V) -> Result<(), Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(txn.put(bucket.db(), &key, &val, lmdb::WriteFlags::empty())?),
            &mut Txn::Phantom(_) => unreachable!()
        }
    }

    /// Sets the value associated with the given key if it doesn't already exist
    pub fn set_no_overwrite<'a, K: Key>(&mut self, bucket: &Bucket<K>, key: K, val: V) -> Result<(), Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(txn.put(bucket.db(), &key, &val, lmdb::WriteFlags::NO_OVERWRITE)?),
            &mut Txn::Phantom(_) => unreachable!()
        }
    }

    /// Deletes the key and value associated with `key` from the database
    pub fn del<K: Key>(&mut self, bucket: &Bucket<K>, key: K) -> Result<(), Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(txn.del(bucket.db(), &key, None)?),
            &mut Txn::Phantom(_) => unreachable!()
        }
    }

    /// Open a new readonly cursor
    pub fn read_cursor<K: Key>(&'env self, bucket: &Bucket<K>) -> Result<Cursor<'env, K, V>, Error> {
        match self {
            &Txn::ReadOnly(ref txn,) => Ok(Cursor::read_only(txn.open_ro_cursor(bucket.db())?)),
            &Txn::ReadWrite(ref txn) => Ok(Cursor::read_only(txn.open_ro_cursor(bucket.db())?)),
            &Txn::Phantom(_) => unreachable!()
        }
    }

    /// Open a new writable cursor
    pub fn write_cursor<K: Key>(&'env mut self, bucket: &Bucket<K>) -> Result<Cursor<'env, K, V>, Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(Cursor::read_write(txn.open_rw_cursor(bucket.db())?)),
            &mut Txn::Phantom(_) => unreachable!()
        }
    }
}
