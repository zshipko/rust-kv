#![deny(missing_docs)]

//! `kv` is a simple way to embed a key/value store in any application written in Rust

extern crate lmdb;

mod config;
mod error;
mod txn;
mod store;
mod cursor;
mod types;
#[cfg(test)] mod test;

pub use config::Config;
pub use error::Error;
//pub use txn::Txn;
pub use store::{/*Bucket,*/ Store};
pub use cursor::{/*Cursor,*/ CursorOp};
pub use types::{Key, Value};



