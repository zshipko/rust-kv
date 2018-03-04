use std::marker::PhantomData;

use lmdb;
use lmdb::Transaction;

use error::Error;
use store::Bucket;
use cursor::Cursor;
use types::{Key, Value};

pub struct Hidden<A, B>(PhantomData<A>, PhantomData<B>);

/// Access to the database
pub enum Txn<'env, K, V> {
    /// Readonly access
    ReadOnly(lmdb::RoTransaction<'env>),

    /// Read-write access
    ReadWrite(lmdb::RwTransaction<'env>),

    /// Type information
    Phantom(Hidden<K, V>)
}

impl <'env, K: Key, V: Value<'env>> Txn<'env, K, V> {
    /// Returns true when the transaction is ReadOnly
    pub fn is_read_only(&self) -> bool {
        match self {
            &Txn::ReadOnly(_) => true,
            &Txn::ReadWrite(_) => false,
            &Txn::Phantom(_) => unreachable!()
        }
    }

    pub(crate) fn read_only(t: lmdb::RoTransaction<'env>) -> Txn<'env, K, V> {
        Txn::ReadOnly(t)
    }

    pub(crate) fn read_write(t: lmdb::RwTransaction<'env>) -> Txn<'env, K, V> {
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
    pub fn get(&'env self, bucket: &Bucket, key: K) -> Result<V, Error> {
        match self {
            &Txn::ReadOnly(ref txn) => Ok(V::from_raw(txn.get(bucket.db(), &key.as_ref())?)),
            &Txn::ReadWrite(ref txn) => Ok(V::from_raw(txn.get(bucket.db(), &key.as_ref())?)),
            &Txn::Phantom(_) => unreachable!()
        }
    }

    /// Sets the value associated with the given key
    pub fn set<'b, V0: Value<'b>>(&mut self, bucket: &Bucket, key: K, val: V0) -> Result<(), Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(txn.put(bucket.db(), &key.as_ref(), &val, lmdb::WriteFlags::empty())?),
            &mut Txn::Phantom(_) => unreachable!()
        }
    }

    /// Sets the value associated with the given key if it doesn't already exist
    pub fn set_no_overwrite<'b, V0: Value<'b>>(&mut self, bucket: &Bucket, key: K, val: V0) -> Result<(), Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(txn.put(bucket.db(), &key.as_ref(), &val, lmdb::WriteFlags::NO_OVERWRITE)?),
            &mut Txn::Phantom(_) => unreachable!()
        }
    }

    /// Deletes the key and value associated with `key` from the database
    pub fn del(&mut self, bucket: &Bucket, key: K) -> Result<(), Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(txn.del(bucket.db(), &key.as_ref(), None)?),
            &mut Txn::Phantom(_) => unreachable!()
        }
    }

    /// Open a new readonly cursor
    pub fn read_cursor(&'env self, bucket: &Bucket) -> Result<Cursor<'env, K, V>, Error> {
        match self {
            &Txn::ReadOnly(ref txn,) => Ok(Cursor::read_only(txn.open_ro_cursor(bucket.db())?)),
            &Txn::ReadWrite(ref txn) => Ok(Cursor::read_only(txn.open_ro_cursor(bucket.db())?)),
            &Txn::Phantom(_) => unreachable!()
        }
    }

    /// Open a new writable cursor
    pub fn write_cursor(&'env mut self, bucket: &Bucket) -> Result<Cursor<'env, K, V>, Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(Cursor::read_write(txn.open_rw_cursor(bucket.db())?)),
            &mut Txn::Phantom(_) => unreachable!()
        }
    }
}
