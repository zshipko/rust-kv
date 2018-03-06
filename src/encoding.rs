use std::io;

use buf::ValueBuf;
use error::Error;
use types::Value;

/// Encoded values
pub trait Encoding: Sized {
    /// Encode an object to io::Write
    fn encode_to<W: io::Write>(&self, w: &mut W) -> Result<(), Error>;

    /// Encode an object to ValueBuf
    fn encode(&self) -> Result<ValueBuf<Self>, Error> {
        let mut dst: ValueBuf<Self> = ValueBuf::empty();
        self.encode_to(&mut dst)?;
        Ok(dst)
    }

    /// Decode from a reader
    fn decode_from<R: io::Read>(r: &mut R) -> Result<Self, Error>;

    /// Decode from an existing value
    fn decode<'a, V: Value<'a>>(val: &'a V) -> Result<Self, Error>;
}

impl<E: Encoding> From<E> for ValueBuf<E> {
    fn from(x: E) -> ::ValueBuf<E> {
        ::Encoding::encode(&x).unwrap()
    }
}

#[cfg(feature = "cbor-value")]
/// CBOR encoding
pub mod cbor {
    extern crate serde_cbor;

    /// CBOR datatype
    pub use self::serde_cbor::Value as Cbor;

    impl ::Encoding for Cbor {
        fn encode_to<W: ::std::io::Write>(&self, w: &mut W) -> Result<(), ::Error> {
            match serde_cbor::to_writer(w, self) {
                Ok(()) => Ok(()),
                Err(_) => Err(::Error::InvalidEncoding),
            }
        }

        fn decode_from<R: ::std::io::Read>(r: &mut R) -> Result<Cbor, ::Error> {
            match serde_cbor::from_reader(r) {
                Ok(x) => Ok(x),
                Err(_) => Err(::Error::InvalidEncoding),
            }
        }

        fn decode<'a, V: ::Value<'a>>(v: &'a V) -> Result<Cbor, ::Error> {
            match serde_cbor::from_slice(v.as_ref()) {
                Ok(x) => Ok(x),
                Err(_) => Err(::Error::InvalidEncoding),
            }
        }
    }
}

#[cfg(feature = "json-value")]
/// JSON encoding
pub mod json {
    extern crate serde_json;

    pub use self::serde_json::Value as Json;

    impl ::Encoding for Json {
        fn encode_to<W: ::std::io::Write>(&self, w: &mut W) -> Result<(), ::Error> {
            match serde_json::to_writer(w, self) {
                Ok(()) => Ok(()),
                Err(_) => Err(::Error::InvalidEncoding),
            }
        }

        fn decode_from<R: ::std::io::Read>(r: &mut R) -> Result<Json, ::Error> {
            match serde_json::from_reader(r) {
                Ok(x) => Ok(x),
                Err(_) => Err(::Error::InvalidEncoding),
            }
        }

        fn decode<'a, V: ::Value<'a>>(val: &'a V) -> Result<Json, ::Error> {
            match serde_json::from_slice(val.as_ref()) {
                Ok(x) => Ok(x),
                Err(_) => Err(::Error::InvalidEncoding),
            }
        }
    }
}
