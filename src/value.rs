use crate::{Error, Raw, Value};

#[cfg(feature = "msgpack-value")]
mod msgpack_value {
    use super::*;

    /// Msgpack encoding for any serde-compatible type
    pub struct Msgpack<T: serde::Serialize + serde::de::DeserializeOwned>(pub T);

    impl<T: serde::Serialize + serde::de::DeserializeOwned> Value for Msgpack<T> {
        fn to_raw_value(&self) -> Result<Raw, Error> {
            let x = rmp_serde::to_vec(&self.0)?;
            Ok(x.into())
        }

        fn from_raw_value(r: &Raw) -> Result<Self, Error> {
            let x = rmp_serde::from_slice(r)?;
            Ok(Msgpack(x))
        }
    }
}

#[cfg(feature = "json-value")]
mod json_value {
    use super::*;

    /// JSON encoding for any serde-compatible type
    pub struct Json<T: serde::Serialize + serde::de::DeserializeOwned>(pub T);

    impl<T: serde::Serialize + serde::de::DeserializeOwned> Value for Json<T> {
        fn to_raw_value(&self) -> Result<Raw, Error> {
            let x = serde_json::to_vec(&self.0)?;
            Ok(x.into())
        }

        fn from_raw_value(r: &Raw) -> Result<Self, Error> {
            let x = serde_json::from_slice(r)?;
            Ok(Json(x))
        }
    }

    impl<T: serde::Serialize + serde::de::DeserializeOwned> std::fmt::Display for Json<T> {
        fn fmt(&self, w: &mut std::fmt::Formatter) -> std::fmt::Result {
            let s = match serde_json::to_string_pretty(&self.0) {
                Ok(s) => s,
                Err(_) => return Err(std::fmt::Error),
            };
            write!(w, "{}", s)?;
            Ok(())
        }
    }
}

#[cfg(feature = "bincode-value")]
mod bincode_value {
    use super::*;

    /// Bincode encoding for any serde-compatible type
    pub struct Bincode<T: serde::Serialize + serde::de::DeserializeOwned>(pub T);

    impl<T: serde::Serialize + serde::de::DeserializeOwned> Value for Bincode<T> {
        fn to_raw_value(&self) -> Result<Raw, Error> {
            let x = bincode::serialize(&self.0)?;
            Ok(x.into())
        }

        fn from_raw_value(r: &Raw) -> Result<Self, Error> {
            let x = bincode::deserialize(r)?;
            Ok(Bincode(x))
        }
    }
}

#[cfg(feature = "json-value")]
pub use json_value::Json;

#[cfg(feature = "msgpack-value")]
pub use msgpack_value::Msgpack;

#[cfg(feature = "bincode-value")]
pub use self::bincode_value::Bincode;
