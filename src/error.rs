use std::io;
use std::sync::PoisonError;

use lmdb;

#[derive(Debug, Fail)]
/// Error type
pub enum Error {
    /// An LMDB error
    #[fail(display = "Error in LMDB: {}", _0)]
    LMDB(#[cause] lmdb::Error),

    /// An IO error
    #[fail(display = "IO error: {}", _0)]
    IO(#[cause] io::Error),

    /// A non-existant or invalid bucket was used
    #[fail(display = "Requested bucket doesn't exist")]
    InvalidBucket,

    /// A resource could not be found
    #[fail(display = "Requested key doesn't exist")]
    NotFound,

    /// A transaction is readonly but something tried to write to it
    #[fail(display = "Cannot write in a ReadOnly transaction")]
    ReadOnly,

    /// An encoding error
    #[fail(display = "Could not encode or decode value")]
    InvalidEncoding,

    /// Configuration is invalid
    #[fail(display = "Configuration is invalid")]
    InvalidConfiguration,

    /// Directory doesn't exist
    #[fail(display = "Directory doesn't exist")]
    DirectoryNotFound,

    /// RwLock is poisoned
    #[fail(display = "RwLock is poisoned")]
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

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
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
            &Error::LMDB(lmdb::Error::KeyExist) => true,
            _ => false,
        }
    }
}
