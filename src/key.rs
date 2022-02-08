use std::mem;
use std::time::SystemTime;

use crate::{Error, Raw};

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
pub struct Integer([u8; 16]);

impl From<u128> for Integer {
    fn from(i: u128) -> Integer {
        unsafe { Integer(mem::transmute(i.to_be())) }
    }
}

impl From<u64> for Integer {
    fn from(i: u64) -> Integer {
        let i = i as u128;
        i.into()
    }
}

impl From<u32> for Integer {
    fn from(i: u32) -> Integer {
        let i = i as u128;
        i.into()
    }
}

impl From<i32> for Integer {
    fn from(i: i32) -> Integer {
        let i = i as u128;
        i.into()
    }
}

impl From<usize> for Integer {
    fn from(i: usize) -> Integer {
        let i = i as u128;
        i.into()
    }
}

impl From<Integer> for u128 {
    #[cfg(target_endian = "big")]
    fn from(i: Integer) -> u128 {
        unsafe { mem::transmute(i.0) }
    }

    #[cfg(target_endian = "little")]
    fn from(i: Integer) -> u128 {
        u128::from_be(unsafe { mem::transmute(i.0) })
    }
}

impl From<Integer> for u64 {
    fn from(i: Integer) -> u64 {
        let i: u128 = i.into();
        i as u64
    }
}

impl From<Integer> for usize {
    fn from(i: Integer) -> usize {
        let i: u128 = i.into();
        i as usize
    }
}

impl AsRef<[u8]> for Integer {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> From<&'a [u8]> for Integer {
    fn from(buf: &'a [u8]) -> Integer {
        let mut dst = Integer::from(0u128);
        dst.0[..16].clone_from_slice(&buf[..16]);
        dst
    }
}

impl Integer {
    /// Current timestamp in seconds from the Unix epoch
    pub fn timestamp() -> Result<Integer, Error> {
        let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(Integer::from(ts.as_secs() as u128))
    }

    /// Current timestamp in milliseconds from the Unix epoch
    pub fn timestamp_ms() -> Result<Integer, Error> {
        let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(Integer::from(ts.as_millis()))
    }
}
