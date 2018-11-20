use lmdb;
use lmdb::Transaction;

use cursor::Cursor;
use error::Error;
use store::Bucket;
use types::{Key, Value, ValueMut};

/// Access to the database
pub enum Txn<'env> {
    /// Readonly access
    ReadOnly(lmdb::RoTransaction<'env>),

    /// Read-write access
    ReadWrite(lmdb::RwTransaction<'env>),
}

impl<'env> Txn<'env> {
    /// Returns true when the transaction is ReadOnly
    pub fn is_read_only(&self) -> bool {
        match self {
            Txn::ReadOnly(_) => true,
            Txn::ReadWrite(_) => false,
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
            Txn::ReadOnly(txn) => txn.commit()?,
            Txn::ReadWrite(txn) => txn.commit()?,
        }
        Ok(())
    }

    /// Ends the transaction, discarding all changes
    pub fn abort(self) {
        match self {
            Txn::ReadOnly(txn) => txn.abort(),
            Txn::ReadWrite(txn) => txn.abort(),
        }
    }

    /// Gets the value associated with the given key
    pub fn get<K: Key, V: Value<'env>>(
        &'env self,
        bucket: &Bucket<'env, K, V>,
        key: K,
    ) -> Result<V, Error> {
        match self {
            Txn::ReadOnly(ref txn) => Ok(V::from_raw(txn.get(bucket.db(), &key.as_ref())?)),
            Txn::ReadWrite(ref txn) => Ok(V::from_raw(txn.get(bucket.db(), &key.as_ref())?)),
        }
    }

    /// Sets the value associated with the given key
    pub fn set<K: Key, V: Value<'env>, X: Into<V>>(
        &mut self,
        bucket: &Bucket<'env, K, V>,
        key: K,
        val: X,
    ) -> Result<(), Error> {
        match self {
            Txn::ReadOnly(_) => Err(Error::ReadOnly),
            Txn::ReadWrite(ref mut txn) => {
                txn.put(
                    bucket.db(),
                    &key.as_ref(),
                    &val.into(),
                    lmdb::WriteFlags::empty(),
                )?;
                Ok(())
            }
        }
    }

    /// Sets the value associated with the given key if it doesn't already exist
    pub fn set_no_overwrite<K: Key, V: Value<'env>, X: Into<V>>(
        &mut self,
        bucket: &Bucket<'env, K, V>,
        key: K,
        val: X,
    ) -> Result<(), Error> {
        match self {
            Txn::ReadOnly(_) => Err(Error::ReadOnly),
            Txn::ReadWrite(ref mut txn) => {
                txn.put(
                    bucket.db(),
                    &key.as_ref(),
                    &val.into(),
                    lmdb::WriteFlags::NO_OVERWRITE,
                )?;
                Ok(())
            }
        }
    }

    /// Deletes the key and value associated with `key` from the database
    pub fn del<K: Key, V: Value<'env>>(
        &mut self,
        bucket: &Bucket<'env, K, V>,
        key: K,
    ) -> Result<(), Error> {
        match self {
            Txn::ReadOnly(_) => Err(Error::ReadOnly),
            Txn::ReadWrite(ref mut txn) => {
                txn.del(bucket.db(), &key.as_ref(), None)?;
                Ok(())
            }
        }
    }

    /// Remove all items from the database
    pub fn clear_db<K: Key, V: Value<'env>>(
        &mut self,
        bucket: &Bucket<'env, K, V>,
    ) -> Result<(), Error> {
        match self {
            Txn::ReadOnly(_) => Err(Error::ReadOnly),
            Txn::ReadWrite(ref mut txn) => {
                txn.clear_db(bucket.db())?;
                Ok(())
            }
        }
    }

    /// Reserve a buffer
    pub fn reserve<K: Key, V: Value<'env>>(
        &'env mut self,
        bucket: &Bucket<'env, K, V>,
        key: K,
        len: usize,
    ) -> Result<ValueMut<'env>, Error> {
        match self {
            Txn::ReadOnly(_) => Err(Error::ReadOnly),
            Txn::ReadWrite(ref mut txn) => Ok(ValueMut::new(txn.reserve(
                bucket.db(),
                &key.as_ref(),
                len,
                lmdb::WriteFlags::empty(),
            )?)),
        }
    }

    /// Reserve a buffer with a unique key
    pub fn reserve_no_overwrite<K: Key, V: Value<'env>>(
        &'env mut self,
        bucket: &Bucket<'env, K, V>,
        key: K,
        len: usize,
    ) -> Result<ValueMut<'env>, Error> {
        match self {
            Txn::ReadOnly(_) => Err(Error::ReadOnly),
            Txn::ReadWrite(ref mut txn) => Ok(ValueMut::new(txn.reserve(
                bucket.db(),
                &key.as_ref(),
                len,
                lmdb::WriteFlags::NO_OVERWRITE,
            )?)),
        }
    }

    /// Open a new readonly cursor
    pub fn read_cursor<K: Key, V: Value<'env>>(
        &'env self,
        bucket: &Bucket<'env, K, V>,
    ) -> Result<Cursor<'env, K, V>, Error> {
        match self {
            Txn::ReadOnly(ref txn) => Ok(Cursor::read_only(txn.open_ro_cursor(bucket.db())?)),
            Txn::ReadWrite(ref txn) => Ok(Cursor::read_only(txn.open_ro_cursor(bucket.db())?)),
        }
    }

    /// Open a new writable cursor
    pub fn write_cursor<K: Key, V: Value<'env>>(
        &'env mut self,
        bucket: &Bucket<'env, K, V>,
    ) -> Result<Cursor<'env, K, V>, Error> {
        match self {
            Txn::ReadOnly(_) => Err(Error::ReadOnly),
            Txn::ReadWrite(ref mut txn) => Ok(Cursor::read_write(txn.open_rw_cursor(bucket.db())?)),
        }
    }

    /// Open a nested transaction
    /// NOTE: you must alread be in a read/write transaction otherwise an error will be returned
    pub fn txn(&mut self) -> Result<Txn, Error> {
        match self {
            Txn::ReadOnly(_) => Err(Error::ReadOnly),
            Txn::ReadWrite(ref mut txn) => Ok(Txn::ReadWrite(txn.begin_nested_txn()?)),
        }
    }
}
