use lmdb;
use lmdb::Cursor as LMDBCursor;

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

/// Abstracts over both readonly and writable cursors
pub enum Cursor<'a> {
    /// A read-only cursor
    ReadOnly(lmdb::RoCursor<'a>),

    /// A writable cursor
    ReadWrite(lmdb::RwCursor<'a>)
}

impl <'a> Cursor<'a> {
        /// Returns true when the transaction is ReadOnly
    pub fn is_read_only(&self) -> bool {
        match self {
            &Cursor::ReadOnly(_) => true,
            &Cursor::ReadWrite(_) => false
        }
    }


    pub(crate) fn read_only(cursor: lmdb::RoCursor<'a>) -> Cursor<'a> {
        Cursor::ReadOnly(cursor)
    }

    pub(crate) fn read_write(cursor: lmdb::RwCursor<'a>) -> Cursor<'a> {
        Cursor::ReadWrite(cursor)
    }

    #[inline]
    /// Iterate over all key/value pairs
    pub fn iter(&mut self) -> lmdb::Iter<'a> {
        match self {
            &mut Cursor::ReadOnly(ref mut ro) => ro.iter(),
            &mut Cursor::ReadWrite(ref mut rw) => rw.iter()
        }
    }

    #[inline]
    /// Iterate over key/values pairs starting at `key`
    pub fn iter_from<K: Key>(&mut self, key: K) -> lmdb::Iter<'a> {
         match self {
            &mut Cursor::ReadOnly(ref mut ro) => ro.iter_from(key),
            &mut Cursor::ReadWrite(ref mut rw) => rw.iter_from(key)
        }
    }

    #[inline]
    /// Insert a value at the current position
    pub fn put<'b, K: Key, V: Value<'b>>(&mut self, key: K, value: V) -> Result<(), Error> {
         match self {
            &mut Cursor::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Cursor::ReadWrite(ref mut rw) => Ok(rw.put(&key, &value, lmdb::WriteFlags::empty())?)
        }

    }

    #[inline]
    /// Insert a value at the current position
    pub fn del<K: Key>(&mut self) -> Result<(), Error> {
         match self {
            &mut Cursor::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Cursor::ReadWrite(ref mut rw) => Ok(rw.del(lmdb::WriteFlags::empty())?)
         }
    }

    #[inline]
    /// Get a value from the cursor
    pub fn get<K: Key, V: Value<'a>>(&self, key: Option<&[u8]>, value: Option<&[u8]>, op: CursorOp) -> Result<(Option<&'a [u8]>, &'a [u8]), Error> {
         match self {
            &Cursor::ReadOnly(ref ro) => Ok(ro.get(key, value, op.flag())?),
            &Cursor::ReadWrite(ref rw) => Ok(rw.get(key, value, op.flag())?)
         }
    }
}
