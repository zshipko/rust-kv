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

impl <'a> From<&'a [u8]> for Integer {
    fn from(buf: &'a [u8]) -> Integer {
        let mut dst = Integer::from(0);
        for i in 0..8 {
            dst.0[i] = buf[i]
        }
        dst
    }
}

/// A reference to an existing value slice
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ValueRef<'a>(&'a [u8]);

impl<'a> ValueRef<'a> {
    /// Create a new ValueRef from an existing byte slice
    pub fn new(buf: &'a [u8]) -> ValueRef<'a> {
        ValueRef(buf)
    }
}

impl<'a> AsRef<[u8]> for ValueRef<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> Value<'a> for ValueRef<'a> {
    fn from_raw(raw: &'a [u8]) -> ValueRef<'a> {
        ValueRef(raw)
    }
}

impl<'a> From<&'a str> for ValueRef<'a> {
    fn from(s: &'a str) -> ValueRef<'a> {
        ValueRef(s.as_bytes())
    }
}

/// A mutable reference to an existing value slice
#[derive(Debug)]
pub struct ValueMut<'a>(&'a mut [u8]);

impl<'a> ValueMut<'a> {
    /// Create a new ValueMut from an existing byte slice
    pub fn new(buf: &'a mut [u8]) -> ValueMut<'a> {
        ValueMut(buf)
    }

    /// Convert a ValueMut to ValueRef
    pub fn as_value<V: Value<'a>>(&'a self) -> ValueRef<'a> {
        ValueRef(self.0)
    }
}

impl<'a> AsMut<[u8]> for ValueMut<'a> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0
    }
}

impl<'a> AsRef<[u8]> for ValueMut<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a, S: AsRef<[u8]>> Key for S {
}

impl<'a> Value<'a> for &'a [u8] {
    fn from_raw(raw: &'a [u8]) -> Self {
        raw
    }
}

impl<'a> Value<'a> for &'a str {
    fn from_raw(raw: &'a [u8]) -> Self {
        unsafe { str::from_utf8_unchecked(raw) }
    }
}

impl<'a> Value<'a> for String {
    fn from_raw(raw: &'a [u8]) -> Self {
        unsafe {
            String::from(str::from_utf8_unchecked(raw))
        }
    }
}
