use lmdb;
use lmdb::Cursor as LMDBCursor;

use types::{Key, Value};
use txn::Hidden;
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
    SetRange = 17,
}

impl CursorOp {
    fn flag(self) -> u32 {
        self as u32
    }
}

/// Iterable access to the database
pub enum Cursor<'a, K, V> {
    /// Readonly access
    ReadOnly(lmdb::RoCursor<'a>),

    /// Read-write access
    ReadWrite(lmdb::RwCursor<'a>),

    /// Type information
    Phantom(Hidden<K, V>),
}

impl<'a, K: Key, V: Value<'a>> Cursor<'a, K, V> {
    /// Returns true when the transaction is ReadOnly
    pub fn is_read_only(&self) -> bool {
        match self {
            &Cursor::ReadOnly(_) => true,
            &Cursor::ReadWrite(_) => false,
            &Cursor::Phantom(_) => unreachable!(),
        }
    }

    pub(crate) fn read_only(cursor: lmdb::RoCursor<'a>) -> Cursor<'a, K, V> {
        Cursor::ReadOnly(cursor)
    }

    pub(crate) fn read_write(cursor: lmdb::RwCursor<'a>) -> Cursor<'a, K, V> {
        Cursor::ReadWrite(cursor)
    }

    #[inline]
    /// Iterate over all key/value pairs
    pub fn iter(&mut self) -> lmdb::Iter<'a> {
        match self {
            &mut Cursor::ReadOnly(ref mut ro) => ro.iter(),
            &mut Cursor::ReadWrite(ref mut rw) => rw.iter(),
            &mut Cursor::Phantom(_) => unreachable!(),
        }
    }

    #[inline]
    /// Iterate over key/values pairs starting at `key`
    pub fn iter_from(&mut self, key: &'a K) -> lmdb::Iter<'a> {
        match self {
            &mut Cursor::ReadOnly(ref mut ro) => ro.iter_from(key.as_ref()),
            &mut Cursor::ReadWrite(ref mut rw) => rw.iter_from(key.as_ref()),
            &mut Cursor::Phantom(_) => unreachable!(),
        }
    }

    #[inline]
    /// Insert a value at the current position
    pub fn put<V0: AsRef<V>>(&mut self, key: &'a K, value: &'a V0) -> Result<(), Error> {
        match self {
            &mut Cursor::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Cursor::ReadWrite(ref mut rw) => {
                Ok(rw.put(&key.as_ref(), &value.as_ref(), lmdb::WriteFlags::empty())?)
            }
            &mut Cursor::Phantom(_) => unreachable!(),
        }
    }

    #[inline]
    /// Insert a value at the current position
    pub fn del(&mut self) -> Result<(), Error> {
        match self {
            &mut Cursor::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Cursor::ReadWrite(ref mut rw) => Ok(rw.del(lmdb::WriteFlags::empty())?),
            &mut Cursor::Phantom(_) => unreachable!(),
        }
    }

    /// Get a value from the cursor
    pub fn get(&self, key: Option<K>, op: CursorOp) -> Result<(Option<K>, V), Error>
    where
        K: From<&'a [u8]>,
    {
        let k = match key {
            Some(ref k) => Some(k.as_ref()),
            None => None,
        };

        match self {
            &Cursor::ReadOnly(ref ro) => {
                let (_k, _v) = ro.get(k, None, op.flag())?;
                Ok((_k.map(K::from), V::from_raw(_v)))
            }
            &Cursor::ReadWrite(ref rw) => {
                let (_k, _v) = rw.get(k, None, op.flag())?;
                Ok((_k.map(K::from), V::from_raw(_v)))
            }
            &Cursor::Phantom(_) => unreachable!(),
        }
    }
}
