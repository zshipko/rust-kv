use lmdb;
use lmdb::Transaction;

use error::Error;
use store::Bucket;
use cursor::Cursor;
use types::{Key, Value};

/// Provices access to the database
pub enum Txn<'env> {
    /// Read only transaction
    ReadOnly(lmdb::RoTransaction<'env>),

    /// Writable transaction
    ReadWrite(lmdb::RwTransaction<'env>)
}

impl <'env> Txn<'env> {
    /// Returns true when the transaction is ReadOnly
    pub fn is_read_only(&self) -> bool {
        match self {
            &Txn::ReadOnly(_) => true,
            &Txn::ReadWrite(_) => false
        }
    }

    pub(crate) fn read_only(t: lmdb::RoTransaction<'env>) -> Txn<'env> {
        Txn::ReadOnly(t)
    }

    pub(crate) fn read_write(t: lmdb::RwTransaction<'env>) -> Txn<'env> {
        Txn::ReadWrite(t)
    }

    /// Ends the transaction, saving all changes
    pub fn commit(self) -> Result<(), Error> {
        match self {
            Txn::ReadOnly(txn) => Ok(txn.commit()?),
            Txn::ReadWrite(txn) => Ok(txn.commit()?)
        }
    }

    /// Ends the transaction, discarding all changes
    pub fn abort(self) {
        match self {
            Txn::ReadOnly(txn) => txn.abort(),
            Txn::ReadWrite(txn) => txn.abort()
        }
    }

    /// Gets the value associated with the given key
    pub fn get<K: Key, V: Value<'env>>(&'env self, bucket: &Bucket, key: K) -> Result<V, Error> {
        match self {
            &Txn::ReadOnly(ref txn) => Ok(V::from_raw(txn.get(bucket.db(), &key)?)),
            &Txn::ReadWrite(ref txn) => Ok(V::from_raw(txn.get(bucket.db(), &key)?))
        }
    }

    /// Sets the value associated with the given key
    pub fn set<'a, K: Key, V: Value<'a>>(&mut self, bucket: &Bucket, key: K, val: V) -> Result<(), Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(txn.put(bucket.db(), &key, &val, lmdb::WriteFlags::empty())?),
        }
    }

    /// Sets the value associated with the given key if it doesn't already exist
    pub fn set_unique<'a, K: Key, V: Value<'a>>(&mut self, bucket: &Bucket, key: K, val: V) -> Result<(), Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(txn.put(bucket.db(), &key, &val, lmdb::WriteFlags::NO_OVERWRITE)?),
        }
    }

    /// Deletes the key and value associated with `key` from the database
    pub fn del<K: Key>(&mut self, bucket: &Bucket, key: K) -> Result<(), Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(txn.del(bucket.db(), &key, None)?),
        }
    }

    /// Open a new readonly cursor
    pub fn read_cursor(&self, bucket: &Bucket) -> Result<Cursor, Error> {
        match self {
            &Txn::ReadOnly(ref txn) => Ok(Cursor::read_only(txn.open_ro_cursor(bucket.db())?)),
            &Txn::ReadWrite(ref txn) => Ok(Cursor::read_only(txn.open_ro_cursor(bucket.db())?))
        }
    }

    /// Open a new writable cursor
    pub fn write_cursor(&mut self, bucket: &Bucket) -> Result<Cursor, Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(Cursor::read_write(txn.open_rw_cursor(bucket.db())?))
        }
    }
}
