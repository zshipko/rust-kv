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
    pub type Cbor = serde_cbor::Value;

    impl ::Encoding for Cbor {
        /// Encode a CBOR value to ValueBuf
        fn encode(&self) -> Result<::ValueBuf<Cbor>, ::Error> {
            let mut dst = ::ValueBuf::empty();
            match serde_cbor::to_writer(&mut dst, self) {
                Ok(()) => (),
                Err(_) => return Err(::Error::InvalidEncoding)
            }
            Ok(dst)
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

