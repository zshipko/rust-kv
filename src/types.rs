use std::{mem, str};

/// A Key can be used as a key to a database
pub trait Key: AsRef<[u8]> {
}

/// A Value can be stored in a database
pub trait Value<'a>: AsRef<[u8]> {
    /// Used to convert a byte-slice to Value
    fn from_raw(raw: &'a [u8]) -> Self;
}


/// Integer key type
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Integer([u8; 8]);

impl From<u64> for Integer {
    #[cfg(target_endian = "little")]
    fn from(i: u64) -> Integer {
        unsafe {
            Integer(mem::transmute(i.to_le()))
        }
    }

    #[cfg(target_endian = "big")]
    fn from(i: u64) -> Integer {
        unsafe {
            Integer(mem::transmute(i.to_be()))
        }
    }
}

impl From<Integer> for u64 {
    fn from(i: Integer) -> u64 {
        unsafe {
            mem::transmute(i.0)
        }
    }
}

impl AsRef<[u8]> for Integer {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl <S: AsRef<[u8]>> Key for S {

}

impl <'a> Value<'a> for &'a [u8] {
    fn from_raw(raw: &'a [u8]) -> Self {
        raw
    }
}

impl <'a> Value<'a> for &'a str {
    fn from_raw(raw: &'a [u8]) -> Self {
        unsafe {
            str::from_utf8_unchecked(raw)
        }
    }
}
