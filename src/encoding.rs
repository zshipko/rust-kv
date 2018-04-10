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
    fn decode<'a, V: Value<'a>>(val: &'a V) -> Result<Self, Error> {
        let mut v = val.as_ref();
        Self::decode_from(&mut v)
    }
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
    }
}

#[cfg(feature = "bincode-value")]
/// Bincode encoding
pub mod bincode {
    extern crate bincode;
    use std::collections::BTreeMap;
    use std::cmp::Ordering;

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
    /// Float wrapper
    pub struct Float(f64);

    impl Eq for Float {

    }

    impl Ord for Float {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap_or(Ordering::Less)
        }
    }

    impl From<f64> for Float {
        fn from(f: f64) -> Float {
            Float(f)
        }
    }

    impl From<Float> for f64 {
        fn from(f: Float) -> f64 {
            f.0
        }
    }

    #[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
    /// Bincode data
    pub enum Data {
        /// Null value
        Null,

        /// Boolean
        Bool(bool),

        /// Integer
        Int(i64),

        /// Double
        Float(Float),

        /// String
        String(String),

        /// Array
        Array(Vec<Data>),

        /// Dictionary
        Dict(BTreeMap<Data, Data>),

        /// Custom value
        Custom(String, Box<Data>)
    }

    impl From<bool> for Data {
        fn from(b: bool) -> Data {
            Data::Bool(b)
        }
    }

    impl From<i64> for Data {
        fn from(i: i64) -> Data {
            Data::Int(i)
        }
    }

    impl From<f64> for Data {
        fn from(f: f64) -> Data {
            Data::Float(Float(f))
        }
    }

    impl From<String> for Data {
        fn from(s: String) -> Data {
            Data::String(s)
        }
    }

    impl From<Vec<Data>> for Data {
        fn from(v: Vec<Data>) -> Data {
            Data::Array(v)
        }
    }

    impl From<BTreeMap<Data, Data>> for Data {
        fn from(d: BTreeMap<Data, Data>) -> Data {
            Data::Dict(d)
        }
    }

    impl From<(String, Data)> for Data {
        fn from((s, d): (String, Data)) -> Data {
            Data::Custom(s, Box::new(d))
        }
    }

    impl ::Encoding for Data {
        fn encode_to<W: ::std::io::Write>(&self, w: &mut W) -> Result<(), ::Error> {
            match bincode::serialize_into(w, self) {
                Ok(()) => Ok(()),
                Err(_) => Err(::Error::InvalidEncoding),
            }
        }

        fn decode_from<R: ::std::io::Read>(r: &mut R) -> Result<Data, ::Error> {
            match bincode::deserialize_from(r) {
                Ok(x) => Ok(x),
                Err(_) => Err(::Error::InvalidEncoding),
            }
        }
    }
}

#[cfg(feature = "capnp-value")]
/// Cap N Proto encoding
pub mod capnp {
    extern crate capnp;

    use self::capnp::message::{Builder, Reader};

    pub enum Proto {
        Writer(Builder<capnp::message::HeapAllocator>),
        Reader(Reader<capnp::serialize::OwnedSegments>)
    }

    impl From<Builder<capnp::message::HeapAllocator>> for Proto {
        fn from(b: Builder<capnp::message::HeapAllocator>) -> Proto {
            Proto::Writer(b)
        }
    }

    impl From<Reader<capnp::serialize::OwnedSegments>> for Proto {
        fn from(r: Reader<capnp::serialize::OwnedSegments>) -> Proto {
            Proto::Reader(r)
        }
    }

    impl ::Encoding for Proto {
        fn encode_to<W: ::std::io::Write>(&self, w: &mut W) -> Result<(), ::Error> {
            match self {
                Proto::Writer(p) => Ok(capnp::serialize::write_message(w, p)?),
                Proto::Reader(_) => Err(::Error::InvalidEncoding)
            }
        }

        fn decode_from<R: ::std::io::Read>(r: &mut R) -> Result<Self, ::Error> {
            let opts = capnp::message::ReaderOptions::default();
            let msg = match capnp::serialize::read_message(r, opts) {
                Ok(msg) => msg,
                Err(_) => return Err(::Error::InvalidEncoding)
            };

            Ok(Proto::Reader(msg))
        }
    }
}

