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
        Error::LMDB(err)
    }
}
