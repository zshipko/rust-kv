#[allow(unused_imports)]
use crate::{Error, Raw, Value};

/// Base trait for values that can be encoded using serde
pub trait Codec<T: serde::Serialize + serde::de::DeserializeOwned>:
    Value + AsRef<T> + AsMut<T>
{
    /// Convert back into inner value
    fn to_inner(self) -> T;
}

#[macro_export]
/// Define a codec type and implement the Codec trait
macro_rules! codec {
    ($x:ident) => {
        /// Codec implementation
        pub struct $x<T: serde::Serialize + serde::de::DeserializeOwned>(pub T);

        impl<T: serde::Serialize + serde::de::DeserializeOwned> AsRef<T> for $x<T> {
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<T: serde::Serialize + serde::de::DeserializeOwned> AsMut<T> for $x<T> {
            fn as_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }

        impl<T: serde::Serialize + serde::de::DeserializeOwned> Codec<T> for $x<T> {
            fn to_inner(self) -> T {
                self.0
            }
        }

        impl<T: Clone + serde::Serialize + serde::de::DeserializeOwned> Clone for $x<T> {
            fn clone(&self) -> Self {
                $x(self.0.clone())
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

#[cfg(feature = "lexpr-value")]
mod lexpr_value {
    use super::*;

    codec!(Lexpr, {serde_lexpr::to_vec, serde_lexpr::from_slice});
}

#[cfg(feature = "json-value")]
pub use json_value::Json;

#[cfg(feature = "msgpack-value")]
pub use msgpack_value::Msgpack;

#[cfg(feature = "bincode-value")]
pub use bincode_value::Bincode;

#[cfg(feature = "lexpr-value")]
pub use lexpr_value::Lexpr;
