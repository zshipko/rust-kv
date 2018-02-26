pub trait Key: AsRef<[u8]> {

}

pub trait Value: AsRef<[u8]> {

}

impl Key for str {

}

impl <'a> Key for &'a str {

}

impl <T: AsRef<[u8]>> Value for T {

}
