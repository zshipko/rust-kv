#![deny(missing_docs)]

//! `kv` is a simple way to embed a key/value store in Rust applications. It is built using
//! [sled](https://docs.rs/sled) and aims to be as lightweight as possible while still
//! providing a nice high level interface.
//!
//! ## Getting started
//!
//! ```rust
//! use kv::*;
//!
//! #[derive(serde::Serialize, serde::Deserialize, PartialEq)]
//! struct SomeType {
//!     a: i32,
//!     b: i32
//! }
//!
//! fn run() -> Result<(), Error> {
//!     // Configure the database
//!     let mut cfg = Config::new("./test/example1");
//!
//!     // Open the key/value store
//!     let store = Store::new(cfg)?;
//!
//!     // A Bucket provides typed access to a section of the key/value store
//!     let test = store.bucket::<Raw, Raw>(Some("test"))?;
//!
//!     let key = Raw::from(b"test");
//!     let value = Raw::from(b"123");
//!
//!     // Set test = 123
//!     test.set(&key, &value)?;
//!     assert!(test.get(&key).unwrap().unwrap() == value);
//!     assert!(test.get(&b"something else".into()).unwrap() == None);
//!
//!     // Integer keys
//!     let aaa = store.bucket::<Integer, String>(Some("aaa"))?;
//!     let key = Integer::from(1);
//!     let value = String::from("Testing");
//!     aaa.set(&key, &value);
//!
//!     #[cfg(feature = "json-value")]
//!     {
//!         // Using a Json encoded type is easy, thanks to Serde
//!         let bucket = store.bucket::<&str, Json<SomeType>>(None)?;
//!
//!         let k = "example";
//!         let x = Json(SomeType {a: 1, b: 2});
//!         bucket.set(&k, &x)?;
//!
//!         let x: Json<SomeType> = bucket.get(&k)?.unwrap();
//!
//!         for item in bucket.iter() {
//!             let item = item?;
//!             let key: String = item.key()?;
//!             let value = item.value::<Json<SomeType>>()?;
//!             println!("key: {}, value: {}", key, value);
//!         }
//!
//!         // A transaction
//!         bucket.transaction(|txn| {
//!             txn.set(&"x", &Json(SomeType {a: 1, b: 2}))?;
//!             txn.set(&"y", &Json(SomeType {a: 3, b: 4}))?;
//!             txn.set(&"z", &Json(SomeType {a: 5, b: 6}))?;
//!
//!             Ok(())
//!         })?;
//!     }
//!     Ok(())
//! }
//! #
//! # fn main() {
//! #     run().unwrap();
//! # }
//! ```

mod bucket;
mod codec;
mod config;
mod error;
mod key;
mod store;
mod transaction;
mod value;

pub use bucket::{Batch, Bucket, Event, Item, Iter, Watch};
pub use codec::*;
pub use config::Config;
pub use error::Error;
pub use key::{Integer, Key};
pub use store::Store;
pub use transaction::{Transaction, TransactionError};
pub use value::{Raw, Value};

/// Abort a transaction
pub fn abort<E>(x: E) -> TransactionError<E> {
    TransactionError::Abort(x)
}

#[cfg(test)]
mod tests;
