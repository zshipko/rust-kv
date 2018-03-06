use std::io;

use lmdb;

#[derive(Debug)]
/// Error type
pub enum Error {
    /// An LMDB error
    LMDB(lmdb::Error),

    /// An IO error
    IO(io::Error),

    /// A non-existant or invalid bucket was used
    InvalidBucket,

    /// A resource could not be found
    NotFound,

    /// A transaction is readonly but something tried to write to it
    ReadOnly,

    /// An encoding error
    InvalidEncoding,

    /// Configuration is invalid
    InvalidConfiguration,

    /// Directory doesn't exist
    DirectoryNotFound
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

impl Error {
    /// Returns true when the error is because of a duplicate key
    pub fn key_exists_error(&self) -> bool {
        match self {
            &Error::LMDB(lmdb::Error::KeyExist) => true,
            _ => false,
        }
    }
}
