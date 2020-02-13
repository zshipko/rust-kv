use crate::{Buffer, Error, FromValue, ToValue, Value};

#[cfg(feature = "msgpack-value")]
mod msgpack_value {
    use super::*;

    /// Msgpack encoding for any serde-compatible type
    pub struct Msgpack<'a, T: serde::Serialize + serde::de::DeserializeOwned>(pub &'a T);

    impl<'a, T: serde::Serialize + serde::de::DeserializeOwned> ToValue<Buffer<Msgpack<'a, T>>>
        for Msgpack<'a, T>
    {
        fn to_value(self) -> Result<Buffer<Msgpack<'a, T>>, Error> {
            let x = rmp_serde::to_vec(&self.0)?;
            Ok(Buffer::new(x))
        }
    }
}

#[cfg(feature = "json-value")]
mod json_value {
    use super::*;

    /// JSON encoding for any serde-compatible type
    pub struct Json<'a, T: serde::Serialize + serde::de::DeserializeOwned>(pub &'a T);

    impl<'a, T: serde::Serialize + serde::de::DeserializeOwned> ToValue<Buffer<Json<'a, T>>>
        for Json<'a, T>
    {
        fn to_value(self) -> Result<Buffer<Json<'a, T>>, Error> {
            let x = serde_json::to_vec(&self.0)?;
            Ok(Buffer::new(x))
        }
    }
}

#[cfg(feature = "bincode-value")]
mod bincode_value {
    use super::*;

    /// Bincode encoding for any serde-compatible type
    pub struct Bincode<'a, T: serde::Serialize + serde::de::DeserializeOwned>(pub &'a T);

    impl<'a, T: serde::Serialize + serde::de::DeserializeOwned> ToValue<Buffer<Bincode<'a, T>>>
        for Bincode<'a, T>
    {
        fn to_value(self) -> Result<Buffer<Bincode<'a, T>>, Error> {
            let x = bincode::serialize(&self.0)?;
            Ok(Buffer::new(x))
        }
    }

    impl<'a, T: serde::Serialize + serde::de::DeserializeOwned, V: Value<'a>> FromValue<V> for T {
        fn from_value(v: V) -> Result<T, Error> {
            let x = v.to_raw_value();
            let x = bincode::deserialize(&x)?;
            Ok(x)
        }
    }
}

#[cfg(feature = "json-value")]
pub use json_value::Json;

#[cfg(feature = "msgpack-value")]
pub use msgpack_value::Msgpack;

#[cfg(feature = "bincode-value")]
pub use self::bincode_value::Bincode;
