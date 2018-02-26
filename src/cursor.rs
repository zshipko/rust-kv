use lmdb;
use lmdb::Cursor;

use types::Key;

pub struct RoCursor<'a>(pub lmdb::RoCursor<'a>);

impl <'a> RoCursor<'a> {
    #[inline]
    pub fn iter(&mut self) -> lmdb::Iter<'a> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_from<K: Key>(&mut self, key: K) -> lmdb::Iter<'a> {
        self.0.iter_from(key)
    }
}

pub struct RwCursor<'a>(pub lmdb::RwCursor<'a>);

impl <'a> RwCursor<'a> {
    #[inline]
    pub fn iter(&mut self) -> lmdb::Iter<'a> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_from<K: Key>(&mut self, key: K) -> lmdb::Iter<'a> {
        self.0.iter_from(key)
    }
}
