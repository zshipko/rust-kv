#[allow(unused_imports)]
use crate::{Error, Raw, Value};

/// Base trait for shared functionality
pub trait Codec<T>: Value {
    /// Get a reference to inner value
    fn inner(&self) -> &T;

    /// Get a mutable reference to inner value
    fn inner_mut(&mut self) -> &mut T;

    /// Convert back into inner value
    fn into_inner(self) -> T;
}

#[macro_export]
/// Define a codec type and implement the Codec trait
macro_rules! codec {
    ($x:ident) => {
        /// Codec implementation
        pub struct $x<T: serde::Serialize + serde::de::DeserializeOwned>(pub T);

        impl<T: serde::Serialize + serde::de::DeserializeOwned> Codec<T> for $x<T> {
            fn inner(&self) -> &T {
                &self.0
            }

            fn inner_mut(&mut self) -> &mut T {
                &mut self.0
            }

            fn into_inner(self) -> T {
                self.0
            }
        }
    };
}

#[cfg(feature = "msgpack-value")]
mod msgpack_value {
    use super::*;

    codec!(Msgpack);

    impl<T: serde::Serialize + serde::de::DeserializeOwned> Value for Msgpack<T> {
        fn to_raw_value(&self) -> Result<Raw, Error> {
            let x = rmp_serde::to_vec(&self.0)?;
            Ok(x.into())
        }

        fn from_raw_value(r: Raw) -> Result<Self, Error> {
            let x = rmp_serde::from_slice(&r)?;
            Ok(Msgpack(x))
        }
    }
}

#[cfg(feature = "json-value")]
mod json_value {
    use super::*;

    codec!(Json);

    impl<T: serde::Serialize + serde::de::DeserializeOwned> Value for Json<T> {
        fn to_raw_value(&self) -> Result<Raw, Error> {
            let x = serde_json::to_vec(&self.0)?;
            Ok(x.into())
        }

        fn from_raw_value(r: Raw) -> Result<Self, Error> {
            let x = serde_json::from_slice(&r)?;
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

    codec!(Bincode);

    impl<T: serde::Serialize + serde::de::DeserializeOwned> Value for Bincode<T> {
        fn to_raw_value(&self) -> Result<Raw, Error> {
            let x = bincode::serialize(&self.0)?;
            Ok(x.into())
        }

        fn from_raw_value(r: Raw) -> Result<Self, Error> {
            let x = bincode::deserialize(&r)?;
            Ok(Bincode(x))
        }
    }
}

#[cfg(feature = "json-value")]
pub use json_value::Json;

#[cfg(feature = "msgpack-value")]
pub use msgpack_value::Msgpack;

#[cfg(feature = "bincode-value")]
pub use bincode_value::Bincode;
