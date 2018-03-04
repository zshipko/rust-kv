use std::str;

/// A Key can be used as a key to a database
pub trait Key: AsRef<[u8]> {

}

/// A Value can be stored in a database
pub trait Value<'a>: AsRef<[u8]> {
    /// Used to convert a byte-slice to Value
    fn from_raw(raw: &'a [u8]) -> Self;
}

impl Key for str {

}

impl <'a> Key for &'a str {

}

impl <'a> Key for &'a [u8] {

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
