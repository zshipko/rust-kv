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

/// A trait for types wrapping Serde values
pub trait Serde<T>: Encoding {
    /// Wraps a serde-compatible type in a `Serde`
    fn from_serde(t: T) -> Self;

    /// Unwraps a serde-compatible type from a `Serde`
    fn to_serde(self) -> T;

    /// Converts a serde-compatible type to `ValueBuf` directly
    fn to_value_buf(t: T) -> Result<ValueBuf<Self>, Error> {
        Self::from_serde(t).encode()
    }
}

#[cfg(feature = "cbor-value")]
/// The cbor encoding allows for any {de|se}rializable type to be read/written to the database
/// using a ValueBuf, for example:
///
/// ```rust
/// extern crate kv;
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
///
/// use serde::{Deserialize, Serialize};
/// use kv::cbor::Cbor;
/// use kv::{Config, Encoding, Error, Manager, Serde, ValueBuf};
///
/// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// struct Testing {
///     a: i32,
///     b: String
/// }
///
/// fn run() -> Result<(), Error> {
///     let mut mgr = Manager::new();
///     let mut cfg = Config::default("/tmp/rust-kv");
///     let handle = mgr.open(cfg)?;
///     let store = handle.write()?;
///     let bucket = store.bucket::<&str, ValueBuf<Cbor<Testing>>>(None)?;
///     let mut txn = store.write_txn()?;
///     let t = Testing{a: 123, b: "abc".to_owned()};
///     txn.set(
///         &bucket,
///         "testing",
///         Cbor::to_value_buf(t)?,
///     )?;
///     txn.commit()?;
///
///     let txn = store.read_txn()?;
///     let buf = txn.get(&bucket, "testing")?;
///     let v = buf.inner()?;
///     println!("{:?}", v.to_serde());
///     Ok(())
/// }
/// #
/// # fn main() {
/// #     run().unwrap();
/// # }
/// ```
pub mod cbor {
    extern crate serde_cbor;

    use std::io::{Read, Write};

    use self::serde_cbor::{from_reader, to_writer};
    use serde::{de::DeserializeOwned, ser::Serialize};
    use super::{Encoding, Error, Serde};

    /// An opaque type for CBOR encoding that wraps a Serde-compatible type T.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Cbor<T>(T);

    impl<T> AsRef<T> for Cbor<T> {
        fn as_ref(&self) -> &T {
            &self.0
        }
    }

    impl<T> AsMut<T> for Cbor<T> {
        fn as_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl<T> Serde<T> for Cbor<T>
    where
        T: DeserializeOwned + Serialize,
    {
        fn from_serde(t: T) -> Self {
            Cbor(t)
        }

        fn to_serde(self) -> T {
            self.0
        }
    }

    impl<T> Encoding for Cbor<T>
    where
        T: DeserializeOwned + Serialize,
    {
        fn encode_to<W: Write>(&self, w: &mut W) -> Result<(), Error> {
            to_writer(w, &self.0).map_err(|_| Error::InvalidEncoding)
        }

        fn decode_from<R: Read>(r: &mut R) -> Result<Self, Error> {
            from_reader(r).map(Cbor).map_err(|_| Error::InvalidEncoding)
        }
    }
}

#[cfg(feature = "json-value")]
/// The json encoding allows for any {de|se}rializable type to be read/written to the database
/// using a ValueBuf, for example:
///
/// ```rust
/// extern crate kv;
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
///
/// use serde::{Deserialize, Serialize};
/// use kv::json::Json;
/// use kv::{Config, Encoding, Error, Manager, Serde, ValueBuf};
///
/// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// struct Testing {
///     a: i32,
///     b: String
/// }
///
/// fn run() -> Result<(), Error> {
///     let mut mgr = Manager::new();
///     let mut cfg = Config::default("/tmp/rust-kv");
///     let handle = mgr.open(cfg)?;
///     let store = handle.write()?;
///     let bucket = store.bucket::<&str, ValueBuf<Json<Testing>>>(None)?;
///     let mut txn = store.write_txn()?;
///     let t = Testing{a: 123, b: "abc".to_owned()};
///     txn.set(
///         &bucket,
///         "testing",
///         Json::to_value_buf(t)?
///     )?;
///     txn.commit()?;
///
///     let txn = store.read_txn()?;
///     let buf = txn.get(&bucket, "testing")?;
///     let v = buf.inner()?;
///     println!("{:?}", v.to_serde());
///     Ok(())
/// }
/// #
/// # fn main() {
/// #     run().unwrap();
/// # }
/// ```
pub mod json {
    extern crate serde_json;

    use std::io::{Read, Write};

    use self::serde_json::{from_reader, to_writer};
    use serde::{de::DeserializeOwned, ser::Serialize};
    use super::{Encoding, Error, Serde};

    /// An opaque type for JSON encoding that wraps a Serde-compatible type T.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Json<T>(T);

    impl<T> AsRef<T> for Json<T> {
        fn as_ref(&self) -> &T {
            &self.0
        }
    }

    impl<T> AsMut<T> for Json<T> {
        fn as_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl<T> Serde<T> for Json<T>
    where
        T: DeserializeOwned + Serialize,
    {
        fn from_serde(t: T) -> Self {
            Json(t)
        }

        fn to_serde(self) -> T {
            self.0
        }
    }

    impl<T> Encoding for Json<T>
    where
        T: DeserializeOwned + Serialize,
    {
        fn encode_to<W: Write>(&self, w: &mut W) -> Result<(), Error> {
            to_writer(w, &self.0).map_err(|_| Error::InvalidEncoding)
        }

        fn decode_from<R: Read>(r: &mut R) -> Result<Self, Error> {
            from_reader(r).map(Json).map_err(|_| Error::InvalidEncoding)
        }
    }
}

#[cfg(feature = "bincode-value")]
/// The bincode encoding allows for any {de|se}rializable type to be read/written to the database
/// using a ValueBuf, for example:
///
/// ```rust
/// extern crate kv;
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
///
/// use serde::{Deserialize, Serialize};
/// use kv::bincode::Bincode;
/// use kv::{Config, Encoding, Error, Manager, Serde, ValueBuf};
///
/// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// struct Testing {
///     a: i32,
///     b: String
/// }
///
/// fn run() -> Result<(), Error> {
///     let mut mgr = Manager::new();
///     let mut cfg = Config::default("/tmp/rust-kv");
///     let handle = mgr.open(cfg)?;
///     let store = handle.write()?;
///     let bucket = store.bucket::<&str, ValueBuf<Bincode<Testing>>>(None)?;
///     let mut txn = store.write_txn()?;
///     let t = Testing{a: 123, b: "abc".to_owned()};
///     txn.set(
///         &bucket,
///         "testing",
///         Bincode::to_value_buf(t)?,
///     )?;
///     txn.commit()?;
///
///     let txn = store.read_txn()?;
///     let buf = txn.get(&bucket, "testing")?;
///     let v = buf.inner()?;
///     println!("{:?}", v.to_serde());
///     Ok(())
/// }
/// #
/// # fn main() {
/// #     run().unwrap();
/// # }
/// ```
pub mod bincode {
    extern crate bincode;

    use std::io::{Read, Write};

    use self::bincode::{deserialize_from, serialize_into};
    use serde::{de::DeserializeOwned, ser::Serialize};
    use super::{Encoding, Error, Serde};

    /// An opaque type for Bincode encoding that wraps a Serde-compatible type T.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Bincode<T>(T);

    impl<T> AsRef<T> for Bincode<T> {
        fn as_ref(&self) -> &T {
            &self.0
        }
    }

    impl<T> AsMut<T> for Bincode<T> {
        fn as_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl<T> Serde<T> for Bincode<T>
    where
        T: DeserializeOwned + Serialize,
    {
        fn from_serde(t: T) -> Self {
            Bincode(t)
        }

        fn to_serde(self) -> T {
            self.0
        }
    }

    impl<T> Encoding for Bincode<T>
    where
        T: DeserializeOwned + Serialize,
    {
        fn encode_to<W: Write>(&self, w: &mut W) -> Result<(), Error> {
            serialize_into(w, &self.0).map_err(|_| Error::InvalidEncoding)
        }

        fn decode_from<R: Read>(r: &mut R) -> Result<Self, Error> {
            deserialize_from(r)
                .map(Bincode)
                .map_err(|_| Error::InvalidEncoding)
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
        Reader(Reader<capnp::serialize::OwnedSegments>),
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
                Proto::Reader(_) => Err(::Error::InvalidEncoding),
            }
        }

        fn decode_from<R: ::std::io::Read>(r: &mut R) -> Result<Self, ::Error> {
            let opts = capnp::message::ReaderOptions::default();
            let msg = match capnp::serialize::read_message(r, opts) {
                Ok(msg) => msg,
                Err(_) => return Err(::Error::InvalidEncoding),
            };

            Ok(Proto::Reader(msg))
        }
    }
}
