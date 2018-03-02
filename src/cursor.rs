use lmdb;
use lmdb::Cursor;

use types::{Key, Value};
use error::Error;

/// CursorOp provides the ability to specify the position of a cursor
pub enum CursorOp {
    /// Set the cursor to the first position
    First = 0,

    /// Get the key/value for the current position
    Current = 4,

    /// Set the cursor to the last position
    Last = 6,

    /// Move the cursor to the next position
    Next = 8,

    /// Move the cursor to the previous position
    Prev = 12,

    /// Set the cursor to the specified key
    Set = 16,

    /// Set the cursor to the first key greater than or equal to specified key
    SetRange = 17
}

impl CursorOp {
    fn flag(self) -> u32 {
        self as u32
    }
}

/// A readonly cursor
pub struct RoCursor<'a>(pub lmdb::RoCursor<'a>);

impl <'a> RoCursor<'a> {
    #[inline]
    /// Iterate over all key/value pairs
    pub fn iter(&mut self) -> lmdb::Iter<'a> {
        self.0.iter()
    }

    #[inline]
    /// Iterate over key/values pairs starting at `key`
    pub fn iter_from<K: Key>(&mut self, key: K) -> lmdb::Iter<'a> {
        self.0.iter_from(key)
    }

    #[inline]
    /// Get a value from the cursor
    pub fn get<K: Key, V: Value<'a>>(&self, key: Option<&[u8]>, value: Option<&[u8]>, op: CursorOp) -> Result<(Option<&'a [u8]>, &'a [u8]), Error> {
        Ok(self.0.get(key, value, op.flag())?)
    }
}

/// A writable cursor
pub struct RwCursor<'a>(pub lmdb::RwCursor<'a>);

impl <'a> RwCursor<'a> {
    #[inline]
    /// Iterate over all key/value pairs
    pub fn iter(&mut self) -> lmdb::Iter<'a> {
        self.0.iter()
    }

    #[inline]
    /// Iterate over key/values pairs starting at `key`
    pub fn iter_from<K: Key>(&mut self, key: K) -> lmdb::Iter<'a> {
        self.0.iter_from(key)
    }

    #[inline]
    /// Insert a value at the current position
    pub fn put<'b, K: Key, V: Value<'b>>(&mut self, key: K, value: V) -> Result<(), Error> {
        Ok(self.0.put(&key, &value, lmdb::WriteFlags::empty())?)
    }

    #[inline]
    /// Insert a value at the current position
    pub fn del<K: Key>(&mut self) -> Result<(), Error> {
        Ok(self.0.del(lmdb::WriteFlags::empty())?)
    }

    #[inline]
    /// Get a value from the cursor
    pub fn get<K: Key, V: Value<'a>>(&self, key: Option<&[u8]>, value: Option<&[u8]>, op: CursorOp) -> Result<(Option<&'a [u8]>, &'a [u8]), Error> {
        Ok(self.0.get(key, value, op.flag())?)
    }
}
