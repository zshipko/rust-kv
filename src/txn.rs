use lmdb;
use lmdb::Transaction;

use error::Error;
use store::Bucket;
use cursor::{RoCursor, RwCursor};
use types::{Key, Value};


pub struct RoTxn<'env> {
    ro: lmdb::RoTransaction<'env>
}

impl <'env> RoTxn<'env> {
    pub fn new(t: lmdb::RoTransaction<'env>) -> RoTxn<'env> {
        RoTxn {
            ro: t
        }
    }

    #[inline]
    pub fn commit(self) -> Result<(), Error> {
        Ok(self.ro.commit()?)
    }

    #[inline]
    pub fn abort(self) {
        self.ro.abort()
    }

    #[inline]
    pub fn get<K: AsRef<[u8]>>(&self, bucket: &Bucket, key: &K) -> Result<&[u8], Error> {
        Ok(self.ro.get(bucket.db(), key)?)
    }

    #[inline]
    pub fn cursor<'txn>(&'txn self, bucket: &Bucket) -> Result<RoCursor, Error> {
        Ok(RoCursor(self.ro.open_ro_cursor(bucket.db())?))
    }
}

pub struct RwTxn<'env> {
    rw: lmdb::RwTransaction<'env>
}

impl <'env> RwTxn<'env> {
    pub fn new(t: lmdb::RwTransaction<'env>) -> RwTxn<'env> {
        RwTxn {
            rw: t
        }
    }

    #[inline]
    pub fn commit(self) -> Result<(), Error> {
        Ok(self.rw.commit()?)
    }

    #[inline]
    pub fn abort(self) {
        self.rw.abort()
    }

    #[inline]
    pub fn reserve<'txn, K: Key>(&'txn mut self, bucket: &Bucket, key: K, n: usize) -> Result<&'txn mut [u8], Error> {

        Ok(self.rw.reserve(bucket.db(), &key, n, lmdb::WriteFlags::empty())?)
    }

    #[inline]
    pub fn get<K: Key>(&self, bucket: &Bucket, key: &K) -> Result<&[u8], Error> {
        Ok(self.rw.get(bucket.db(), key)?)
    }

    #[inline]
    pub fn set<K: Key, V: Value>(&mut self, bucket: &Bucket, key: &K, val: &V) -> Result<(), Error> {
        Ok(self.rw.put(bucket.db(), key, val, lmdb::WriteFlags::empty())?)
    }

    //#[inline]
    //pub fn put<V: Value>(&mut self, bucket: &Bucket, val: V) -> Result<Token, Error> {
    //    let token = Token::generate(val.as_ref());
    //    self.set(bucket, token.as_string(), &val)?;
    //    Ok(token)
    //}

    #[inline]
    pub fn del<K: AsRef<[u8]>>(&mut self, bucket: &Bucket, key: &K) -> Result<(), Error> {
        Ok(self.rw.del(bucket.db(), key, None)?)
    }

    #[inline]
    pub fn clear_bucket(&mut self, bucket: &Bucket) -> Result<(), Error> {
        Ok(self.rw.clear_db(bucket.db())?)
    }

    #[inline]
    pub fn cursor(&mut self, bucket: &Bucket) -> Result<RwCursor, Error>{
        Ok(RwCursor(self.rw.open_rw_cursor(bucket.db())?))
    }

}

