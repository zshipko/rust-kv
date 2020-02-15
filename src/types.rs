use std::mem;

use crate::Error;

/// A Key can be used as a key to a database
pub trait Key<'a>: Sized + AsRef<[u8]> {
    /// Convert from Raw
    fn from_raw_key(r: &'a Raw) -> Result<Self, Error>;

    /// Wrapper around AsRef<[u8]>
    fn to_raw_key(&self) -> Result<Raw, Error> {
        Ok(self.as_ref().into())
    }
}

impl<'a> Key<'a> for Raw {
    fn from_raw_key(x: &Raw) -> Result<Self, Error> {
        Ok(x.clone())
    }
}

impl<'a> Key<'a> for &'a [u8] {
    fn from_raw_key(x: &'a Raw) -> Result<&'a [u8], Error> {
        Ok(x.as_ref())
    }
}

impl<'a> Key<'a> for &'a str {
    fn from_raw_key(x: &'a Raw) -> Result<Self, Error> {
        Ok(std::str::from_utf8(x.as_ref())?)
    }
}

impl<'a> Key<'a> for Vec<u8> {
    fn from_raw_key(r: &Raw) -> Result<Self, Error> {
        Ok(r.to_vec())
    }
}

impl<'a> Key<'a> for String {
    fn from_raw_key(x: &Raw) -> Result<Self, Error> {
        Ok(std::str::from_utf8(x.as_ref())?.to_string())
    }
}

impl<'a> Key<'a> for Integer {
    fn from_raw_key(x: &Raw) -> Result<Integer, Error> {
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
    /// Wrapper around AsRef<[u8]>
    fn to_raw_value(&self) -> Result<Raw, Error>;

    /// Convert from Raw
    fn from_raw_value(r: Raw) -> Result<Self, Error>;
}

/// Raw is an alias for `sled::IVec`
pub type Raw = sled::IVec;

impl<'a> Value<'a> for Raw {
    fn to_raw_value(&self) -> Result<Raw, Error> {
        Ok(self.clone())
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        Ok(r)
    }
}

impl<'a> Value<'a> for std::sync::Arc<[u8]> {
    fn to_raw_value(&self) -> Result<Raw, Error> {
        Ok(self.clone().into())
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        Ok(r.into())
    }
}

impl<'a> Value<'a> for Vec<u8> {
    fn to_raw_value(&self) -> Result<Raw, Error> {
        Ok(self.as_slice().into())
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        Ok(r.to_vec())
    }
}

impl<'a> Value<'a> for String {
    fn to_raw_value(&self) -> Result<Raw, Error> {
        Ok(self.as_str().into())
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        let x = r.to_vec();
        Ok(String::from_utf8(x)?)
    }
}
