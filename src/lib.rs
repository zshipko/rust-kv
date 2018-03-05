#![deny(missing_docs)]

//! `kv` is a simple way to embed a key/value store in any application written in Rust

extern crate lmdb;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

mod config;
mod error;
mod txn;
mod store;
mod cursor;
mod types;
mod buf;
mod manager;
mod encoding;
#[cfg(test)] mod test;

#[cfg(feature = "cbor-value")] pub use encoding::cbor;
#[cfg(feature = "json-value")] pub use encoding::json;

pub use config::Config;
pub use error::Error;
pub use txn::Txn;
pub use store::{Bucket, Store};
pub use cursor::{Cursor, CursorOp};
pub use buf::ValueBuf;
pub use types::{Integer, Key, Value, ValueMut, ValueRef};
pub use manager::Manager;
pub use encoding::Encoding;
