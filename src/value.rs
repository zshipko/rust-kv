use crate::Error;

/// A trait used to convert between types and `Raw`
pub trait Value: Sized {
    /// Wrapper around AsRef<[u8]>
    fn to_raw_value(&self) -> Result<Raw, Error>;

    /// Convert from Raw
    fn from_raw_value(r: Raw) -> Result<Self, Error>;
}

/// Raw is an alias for `sled::IVec`
pub type Raw = sled::IVec;

impl Value for Raw {
    fn to_raw_value(&self) -> Result<Raw, Error> {
        Ok(self.clone())
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        Ok(r)
    }
}

impl Value for std::sync::Arc<[u8]> {
    fn to_raw_value(&self) -> Result<Raw, Error> {
        Ok(self.as_ref().into())
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        Ok(r.as_ref().into())
    }
}

impl Value for Vec<u8> {
    fn to_raw_value(&self) -> Result<Raw, Error> {
        Ok(self.as_slice().into())
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        Ok(r.to_vec())
    }
}

impl Value for String {
    fn to_raw_value(&self) -> Result<Raw, Error> {
        Ok(self.as_str().into())
    }

    fn from_raw_value(r: Raw) -> Result<Self, Error> {
        let x = r.to_vec();
        Ok(String::from_utf8(x)?)
    }
}
