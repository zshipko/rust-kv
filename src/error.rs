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

    /// A non-existant or invalid bucket was used
    #[error("Requested bucket doesn't exist")]
    InvalidBucket,

    /// A resource could not be found
    #[error("Requested key doesn't exist")]
    NotFound,

    /// A transaction is readonly but something tried to write to it
    #[error("Cannot write in a ReadOnly transaction")]
    ReadOnly,

    /// An encoding error
    #[error("Could not encode or decode value")]
    InvalidEncoding,

    /// Configuration is invalid
    #[error("Configuration is invalid")]
    InvalidConfiguration,

    /// Directory doesn't exist
    #[error("Directory doesn't exist")]
    DirectoryNotFound,

    /// RwLock is poisoned
    #[error("RwLock is poisoned")]
    Poison,

    /// UTF8 Error
    #[error("UTF8 error")]
    Utf8(std::str::Utf8Error),

    /// String UTF8 Error
    #[error("String UTF8 error")]
    FromUtf8(std::string::FromUtf8Error),

    /// Generic message
    #[error("Message: {0}")]
    Message(String),
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
