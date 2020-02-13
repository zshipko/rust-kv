use std::{mem, str};

use crate::Error;

/// A Key can be used as a key to a database
pub trait Key: AsRef<[u8]> + Sized {
    /// Convert to Raw
    fn to_raw_key(self) -> Raw {
        self.as_ref().into()
    }
}

/// An OwnedKey is used to take ownership of a Raw
pub trait OwnedKey<'a>: 'a + Key {
    /// Convert from Raw
    fn from_raw_key(r: Raw) -> Result<Self, Error>;
}

impl Key for Raw {}

impl<'a> Key for &'a [u8] {}

impl<'a> Key for &'a str {}

impl<'a> Key for Vec<u8> {}

impl<'a> Key for String {}

impl<'a> Key for Integer {}

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
pub trait Value<'a>: 'a + AsRef<[u8]> + Sized {
    /// Convert to Raw
    fn to_raw_value(self) -> Raw {
        self.as_ref().into()
    }

    /// Convert from Raw
    fn from_raw_value(r: Raw) -> Result<Self, Error>;
}

/// Raw is an alias for `sled::IVec`
pub type Raw = sled::IVec;

impl<'a> Value<'a> for Raw {
    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        Ok(r)
    }
}

impl<'a> Value<'a> for Vec<u8> {
    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        Ok(r.to_vec())
    }
}

impl<'a> Value<'a> for String {
    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        let x: std::sync::Arc<[u8]> = r.into();
        Ok(String::from_utf8(x.to_vec())?)
    }
}
