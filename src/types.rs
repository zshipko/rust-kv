use std::str;

pub trait Key: AsRef<[u8]> {

}

pub trait Value<'a>: AsRef<[u8]> {
    fn from_raw(raw: &'a [u8]) -> Self;
}

impl Key for str {

}

impl <'a> Key for &'a str {

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
