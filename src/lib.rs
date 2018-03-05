#![deny(missing_docs)]

//! `kv` is a simple way to embed a key/value store in any application written in Rust

extern crate lmdb;

mod config;
mod error;
mod txn;
mod store;
mod cursor;
mod types;
mod buf;
mod manager;
#[cfg(test)] mod test;
mod encoding;

#[cfg(feature = "cbor-value")] pub use encoding::cbor;

pub use config::Config;
pub use error::Error;
pub use txn::Txn;
pub use store::{Bucket, Store};
pub use cursor::{Cursor, CursorOp};
pub use buf::ValueBuf;
pub use types::{Integer, Key, Value, ValueMut, ValueRef};
pub use manager::Manager;
pub use encoding::Encoding;
