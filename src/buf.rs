use std::io::{self, Read, Write};

use types::{Value, ValueMut};

/// A Value can be used to dynamically build values
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ValueBuf(pub Vec<u8>);

impl ValueBuf {
    /// Create an empty value buffer
    pub fn empty() -> ValueBuf {
        ValueBuf(Vec::new())
    }

    /// Create a new ValueBuf with the given size
    pub fn new(n: usize) -> ValueBuf {
        ValueBuf(Vec::with_capacity(n))
    }
}

impl AsRef<[u8]> for ValueBuf {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<'a> Value<'a> for ValueBuf {
    fn from_raw(raw: &[u8]) -> Self {
        ValueBuf(raw.to_vec())
    }
}

impl Read for ValueBuf {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.as_slice().read(buf)
    }
}

impl Write for ValueBuf {
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
