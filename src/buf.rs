use std::io::{self, Read, Write};
use std::marker::PhantomData;

use crate::types::{Value, ValueMut};
use crate::encoding::Encoding;
use crate::error::Error;

/// A Value can be used to dynamically build values
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ValueBuf<T>(pub Vec<u8>, PhantomData<T>);

impl<T: Encoding> ValueBuf<T> {
    /// Create an empty value buffer
    pub fn empty() -> ValueBuf<T> {
        ValueBuf(Vec::new(), PhantomData)
    }

    /// Create a new ValueBuf with the given size
    pub fn new(n: usize) -> ValueBuf<T> {
        ValueBuf(Vec::with_capacity(n), PhantomData)
    }

    /// Get inner value
    pub fn inner(&self) -> Result<T, Error> {
        T::decode(self)
    }
}

impl<T: Encoding> AsRef<[u8]> for ValueBuf<T> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<'a, T: Encoding> Value<'a> for ValueBuf<T> {
    fn from_raw(raw: &[u8]) -> Self {
        ValueBuf(raw.to_vec(), PhantomData)
    }
}

impl<T: Encoding> Read for ValueBuf<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.as_slice().read(buf)
    }
}

impl<T: Encoding> Write for ValueBuf<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<'a> Write for ValueMut<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.as_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.as_mut().flush()
    }
}
