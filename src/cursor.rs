use lmdb;
use lmdb::Cursor;

use types::Key;

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
}
