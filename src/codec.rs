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

    ($x:ident, {$ser:expr, $de:expr}) => {
        codec!($x);

        impl<T: serde::Serialize + serde::de::DeserializeOwned> Value for $x<T> {
            fn to_raw_value(&self) -> Result<Raw, Error> {
                let x = $ser(&self.0)?;
                Ok(x.into())
            }

            fn from_raw_value(r: Raw) -> Result<Self, Error> {
                let x = $de(&r)?;
                Ok($x(x))
            }
        }
    };
}

#[cfg(feature = "msgpack-value")]
mod msgpack_value {
    use super::*;

    codec!(Msgpack, {rmp_serde::to_vec, rmp_serde::from_slice});
}

#[cfg(feature = "json-value")]
mod json_value {
    use super::*;

    codec!(Json, {serde_json::to_vec, serde_json::from_slice});

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

    codec!(Bincode, {bincode::serialize, bincode::deserialize});
}

#[cfg(feature = "json-value")]
pub use json_value::Json;

#[cfg(feature = "msgpack-value")]
pub use msgpack_value::Msgpack;

#[cfg(feature = "bincode-value")]
pub use bincode_value::Bincode;
