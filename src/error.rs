use std::io;
use std::sync::PoisonError;

use lmdb;
use thiserror::Error as TError;

#[derive(Debug, TError)]
/// Error type
pub enum Error {
    /// An LMDB error
    #[error("Error in LMDB: {0}")]
    LMDB(#[source] lmdb::Error),

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
}

impl From<lmdb::Error> for Error {
    fn from(err: lmdb::Error) -> Error {
        match err {
            lmdb::Error::NotFound => Error::NotFound,
            lmdb::Error::BadDbi => Error::InvalidBucket,
            lmdb::Error::Other(2) => Error::DirectoryNotFound,
            _ => Error::LMDB(err),
        }
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Error {
        Error::Poison
    }
}

impl Error {
    /// Returns true when the error is because of a duplicate key
    pub fn key_exists_error(&self) -> bool {
        match self {
            Error::LMDB(lmdb::Error::KeyExist) => true,
            _ => false,
        }
    }
}
