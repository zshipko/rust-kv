use std::marker::PhantomData;
use std::{mem, str};

use crate::Error;

/// A Key can be used as a key to a database
pub trait Key: Sized {
    /// Convert to Raw
    fn to_raw_key(self) -> Raw;
}

/// An OwnedKey is used to take ownership of a Raw
pub trait OwnedKey<'a>: 'a + Key {
    /// Convert from Raw
    fn from_raw_key(r: Raw) -> Result<Self, Error>;
}

impl Key for Raw {
    fn to_raw_key(self) -> Raw {
        self
    }
}

impl<'a> Key for &'a [u8] {
    fn to_raw_key(self) -> Raw {
        self.into()
    }
}

impl<'a> Key for &'a str {
    fn to_raw_key(self) -> Raw {
        self.into()
    }
}

impl<'a> Key for Vec<u8> {
    fn to_raw_key(self) -> Raw {
        self.into()
    }
}

impl<'a> Key for String {
    fn to_raw_key(self) -> Raw {
        self.as_str().into()
    }
}

impl<'a> Key for Integer {
    fn to_raw_key(self) -> Raw {
        self.as_ref().into()
    }
}

impl<'a> OwnedKey<'a> for Raw {
    fn from_raw_key(x: Raw) -> Result<Raw, Error> {
        Ok(x)
    }
}

impl<'a> OwnedKey<'a> for Vec<u8> {
    fn from_raw_key(x: Raw) -> Result<Self, Error> {
        Ok(x.to_vec())
    }
}

impl<'a> OwnedKey<'a> for String {
    fn from_raw_key(x: Raw) -> Result<Self, Error> {
        Ok(std::str::from_utf8(x.as_ref())?.to_string())
    }
}

impl<'a> OwnedKey<'a> for Integer {
    fn from_raw_key(x: Raw) -> Result<Integer, Error> {
        Ok(Integer::from(x.as_ref()))
    }
}

/// Integer key type
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Integer([u8; 8]);

impl From<u64> for Integer {
    #[cfg(target_endian = "little")]
    fn from(i: u64) -> Integer {
        unsafe { Integer(mem::transmute(i.to_le())) }
    }

    #[cfg(target_endian = "big")]
    fn from(i: u64) -> Integer {
        unsafe { Integer(mem::transmute(i.to_be())) }
    }
}

impl From<Integer> for u64 {
    fn from(i: Integer) -> u64 {
        unsafe { mem::transmute(i.0) }
    }
}

impl AsRef<[u8]> for Integer {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> From<&'a [u8]> for Integer {
    fn from(buf: &'a [u8]) -> Integer {
        let mut dst = Integer::from(0);
        dst.0[..8].clone_from_slice(&buf[..8]);
        dst
    }
}

/// A trait used to convert between types and `Raw`
pub trait Value<'a>: 'a + Sized {
    /// Convert to Raw
    fn to_raw_value(self) -> Raw;

    /// Convert from Raw
    fn from_raw_value(r: Raw) -> Result<Self, Error>;
}

/// Raw is an alias for `sled::IVec`
pub type Raw = sled::IVec;

impl<'a> Value<'a> for Raw {
    fn to_raw_value(self) -> Raw {
        self
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        Ok(r)
    }
}

impl<'a> Value<'a> for Vec<u8> {
    fn to_raw_value(self) -> Raw {
        self.into()
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        Ok(r.to_vec())
    }
}

impl<'a> Value<'a> for String {
    fn to_raw_value(self) -> Raw {
        self.as_str().into()
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        let x: std::sync::Arc<[u8]> = r.into();
        Ok(String::from_utf8(x.to_vec())?)
    }
}

/// Buffer provides a value implementation using an owned buffer
pub struct Buffer<T>(Vec<u8>, PhantomData<T>);

/// ToValue is definted for types that can be converted to `Value` with potential errors
pub trait ToValue<V> {
    /// Convert to a value type
    fn to_value(self) -> Result<V, Error>;
}

impl<'a, T: Value<'a>, X: Into<T>> ToValue<T> for X {
    fn to_value(self) -> Result<T, Error> {
        Ok(self.into())
    }
}

/// FromValue is defined for types that can be converted from `Value` with potential errors
pub trait FromValue<V>: Sized {
    /// Convert from value type
    fn from_value(v: V) -> Result<Self, Error>;
}

/*impl<'a, T: Value<'a>, X: From<T>> FromValue<T> for X {
    fn from_value(v: T) -> Result<Self, Error> {
        Ok(v.into())
    }
}*/

impl<T> Buffer<T> {
    /// Create a new Buffer from an existing buffer
    pub fn new(data: Vec<u8>) -> Buffer<T> {
        Buffer(data, PhantomData)
    }
}

impl<'a, T: 'a> Value<'a> for Buffer<T> {
    fn to_raw_value(self) -> Raw {
        self.0.into()
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        Ok(Buffer(r.to_vec(), PhantomData))
    }
}

impl<T> AsRef<[u8]> for Buffer<T> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
