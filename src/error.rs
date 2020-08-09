use std::io;
use std::sync::PoisonError;

use thiserror::Error as TError;

#[derive(Debug, TError)]
/// Error type
pub enum Error {
    /// A Sled error
    #[error("Error in Sled: {0}")]
    Sled(#[from] sled::Error),

    /// An IO error
    #[error("IO error: {0}")]
    IO(#[from] io::Error),

    /// Configuration is invalid
    #[error("Configuration is invalid")]
    InvalidConfiguration,

    /// RwLock is poisoned
    #[error("RwLock is poisoned")]
    Poison,

    /// UTF8 Error
    #[error("UTF8 error")]
    Utf8(std::str::Utf8Error),

    /// String UTF8 Error
    #[error("String UTF8 error")]
    FromUtf8(std::string::FromUtf8Error),

    /// SystemTime
    #[error("SystemTime: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),

    /// Generic message
    #[error("Message: {0}")]
    Message(String),

    /// Json error
    #[cfg(feature = "json-value")]
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Msgpack encoding error
    #[cfg(feature = "msgpack-value")]
    #[error("Msgpack encoding error: {0}")]
    MsgpackEncode(#[from] rmp_serde::encode::Error),

    /// Msgpack decoding error
    #[cfg(feature = "msgpack-value")]
    #[error("Msgpack decoding error: {0}")]
    MsgpackDecode(#[from] rmp_serde::decode::Error),

    /// Bincode error
    #[cfg(feature = "bincode-value")]
    #[error("Bincode encoding Error: {0}")]
    Bincode(#[from] Box<bincode::ErrorKind>),

    /// Lexpr error
    #[cfg(feature = "lexpr-value")]
    #[error("S-Expression error: {0}")]
    Lexpr(#[from] serde_lexpr::Error),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Error {
        Error::Poison
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Error {
        Error::Utf8(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Error {
        Error::FromUtf8(e)
    }
}

impl From<Error> for sled::transaction::ConflictableTransactionError<Error> {
    fn from(e: Error) -> sled::transaction::ConflictableTransactionError<Error> {
        sled::transaction::ConflictableTransactionError::Abort(e)
    }
}
