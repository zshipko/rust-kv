use lmdb;

#[derive(Debug)]
pub enum Error {
    LMDB(lmdb::Error),
    InvalidBucket,
    NotFound,
    ReadOnly
}

impl From<lmdb::Error> for Error {
    fn from(err: lmdb::Error) -> Error {
        match err {
            lmdb::Error::NotFound => Error::NotFound,
            lmdb::Error::BadDbi => Error::InvalidBucket,
            _ => Error::LMDB(err)
        }

    }
}

impl Error {
    pub fn key_exists_error(&self) -> bool {
        match self {
            &Error::LMDB(lmdb::Error::KeyExist) => true,
            _ => false
        }
    }
}
