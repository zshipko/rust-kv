pub trait Key: AsRef<[u8]> {

}

pub trait Value<'a>: AsRef<[u8]> + From<&'a [u8]> {

}

impl Key for str {

}

impl <'a> Key for &'a str {

}

impl <'a> Value<'a> for &'a [u8] {

}
