use std::marker::PhantomData;

use lmdb;
use lmdb::Cursor as LMDBCursor;

use crate::error::Error;
use crate::types::{Key, Value};

pub struct Hidden<A, B>(PhantomData<A>, PhantomData<B>);

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

/// Iter wrapper
pub struct Iter<'a, K, V>(lmdb::Iter<'a>, Hidden<K, V>);

impl<'a, K: Key, V: Value<'a>> Iterator for Iter<'a, K, V>
where
    K: From<&'a [u8]>,
{
    type Item = (K, V);
    fn next(&mut self) -> Option<(K, V)> {
        let (k, v) = match lmdb::Iter::next(&mut self.0) {
            Some((k, v)) => (k, v),
            None => return None,
        };
        Some((K::from(k), V::from_raw(v)))
    }
}

impl<'a, K: Key, V: Value<'a>> Cursor<'a, K, V> {
    /// Returns true when the transaction is ReadOnly
    pub fn is_read_only(&self) -> bool {
        match self {
            Cursor::ReadOnly(_) => true,
            Cursor::ReadWrite(_) => false,
            Cursor::Phantom(_) => unreachable!(),
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
    pub fn iter(&mut self) -> Iter<'a, K, V> {
        match self {
            Cursor::ReadOnly(ref mut ro) => Iter(ro.iter(), Hidden(PhantomData, PhantomData)),
            Cursor::ReadWrite(ref mut rw) => Iter(rw.iter(), Hidden(PhantomData, PhantomData)),
            Cursor::Phantom(_) => unreachable!(),
        }
    }

    #[inline]
    /// Iterate over key/values pairs starting at `key`
    pub fn iter_from(&mut self, key: &'a K) -> Iter<'a, K, V> {
        match self {
            Cursor::ReadOnly(ref mut ro) => {
                Iter(ro.iter_from(key.as_ref()), Hidden(PhantomData, PhantomData))
            }
            Cursor::ReadWrite(ref mut rw) => {
                Iter(rw.iter_from(key.as_ref()), Hidden(PhantomData, PhantomData))
            }
            Cursor::Phantom(_) => unreachable!(),
        }
    }

    #[inline]
    /// Insert a value at the current position
    pub fn set<V0: Into<V>>(&mut self, key: &'a K, value: V0) -> Result<(), Error> {
        match self {
            Cursor::ReadOnly(_) => Err(Error::ReadOnly),
            Cursor::ReadWrite(ref mut rw) => {
                rw.put(
                    &key.as_ref(),
                    &value.into().as_ref(),
                    lmdb::WriteFlags::empty(),
                )?;
                Ok(())
            }
            Cursor::Phantom(_) => unreachable!(),
        }
    }

    #[inline]
    /// Insert a value at the current position
    pub fn del(&mut self) -> Result<(), Error> {
        match self {
            Cursor::ReadOnly(_) => Err(Error::ReadOnly),
            Cursor::ReadWrite(ref mut rw) => {
                rw.del(lmdb::WriteFlags::empty())?;
                Ok(())
            }
            Cursor::Phantom(_) => unreachable!(),
        }
    }

    /// Get a value from the cursor
    pub fn get(&self, key: Option<&K>, op: CursorOp) -> Result<(Option<K>, V), Error>
    where
        K: From<&'a [u8]>,
    {
        let k = match key {
            Some(ref k) => Some(k.as_ref()),
            None => None,
        };

        match self {
            Cursor::ReadOnly(ref ro) => {
                let (_k, _v) = ro.get(k, None, op.flag())?;
                Ok((_k.map(K::from), V::from_raw(_v)))
            }
            Cursor::ReadWrite(ref rw) => {
                let (_k, _v) = rw.get(k, None, op.flag())?;
                Ok((_k.map(K::from), V::from_raw(_v)))
            }
            Cursor::Phantom(_) => unreachable!(),
        }
    }
}
