#[cfg(feature = "with-serde")] extern crate serde;

use buf::ValueBuf;
use error::Error;
use types::Value;

/// Encoded values
pub trait Encoding: Sized {
    /// Encode an object to ValueBuf
    fn encode(&self) -> Result<ValueBuf<Self>, Error>;

    /// Decode an object from a value reference
    fn decode<'a, V: Value<'a>>(val: &'a V) -> Result<Self, Error>;
}

impl <E: Encoding> From<E> for ValueBuf<E> {
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
        /// Encode a CBOR value to ValueBuf
        fn encode(&self) -> Result<::ValueBuf<Cbor>, ::Error> {
            let mut dst = ::ValueBuf::empty();
            match serde_cbor::to_writer(&mut dst, self) {
                Ok(()) => Ok(dst),
                Err(_) => Err(::Error::InvalidEncoding)
            }
        }

        /// Decode a Value to CBOR value
        fn decode<'a, V: ::Value<'a>>(val: &'a V) -> Result<Cbor, ::Error> {
            match serde_cbor::from_slice(val.as_ref()) {
                Ok(x) => Ok(x),
                Err(_) => Err(::Error::InvalidEncoding)
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
        fn encode(&self) -> Result<::ValueBuf<Json>, ::Error> {
            let mut dst = ::ValueBuf::empty();
            match serde_json::to_writer(&mut dst, self) {
                Ok(()) => Ok(dst),
                Err(_) => Err(::Error::InvalidEncoding)
            }
        }

        fn decode<'a, V: ::Value<'a>>(val: &'a V) -> Result<Json, ::Error> {
            match serde_json::from_slice(val.as_ref()) {
                Ok(x) => Ok(x),
                Err(_) => Err(::Error::InvalidEncoding)
            }
        }
    }
}

