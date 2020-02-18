#![deny(missing_docs)]

//! `kv` is a simple way to embed a key/value store in Rust applications. It is built using
//! [sled](https://docs.rs/sled) and aims to be as lightweight as possible,
//! while still providing a nice high level interface.
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
//!     let mut cfg = Config::new("./test/example");
//!
//!     // Open the key/value store
//!     let store = Store::new(cfg)?;
//!
//!     // A Bucket provides typed access to a section of the key/value store
//!     let test = store.bucket::<Raw, Raw>(Some("test"))?;
//!
//!     // Set testing = 123
//!     test.set(b"test", b"123")?;
//!     assert!(test.get(b"test").unwrap().unwrap() == "123");
//!     assert!(test.get(b"something else").unwrap() == None);
//!
//!     #[cfg(feature = "json-value")]
//!     {
//!         // Using a Json encoded type is easy, thanks to Serde
//!         let bucket = store.bucket::<&str, Json<SomeType>>(None)?;
//!
//!         let x = SomeType {a: 1, b: 2};
//!         bucket.set("example", Json(x))?;
//!
//!         let x: Json<SomeType> = bucket.get("example")?.unwrap();
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
//!             txn.set("x", Json(SomeType {a: 1, b: 2}))?;
//!             txn.set("y", Json(SomeType {a: 3, b: 4}))?;
//!             txn.set("z", Json(SomeType {a: 5, b: 6}))?;
//!
//!             // A nested transaction
//!             test.transaction(|txn2| {
//!                 let x = txn.get("x")?.unwrap();
//!                 let v = format!("{}", x.inner().a);
//!                 txn2.set(b"x", v.as_str())?;
//!                 Ok(())
//!             })?;
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
mod config;
mod error;
mod store;
mod transaction;
mod types;
mod value;

pub use bucket::{Batch, Bucket, Event, Item, Iter, Watch};
pub use config::Config;
pub use error::Error;
pub use store::Store;
pub use transaction::{Transaction, TransactionError};
pub use types::{Integer, Key, Raw, Value};
pub use value::*;

/// Abort a transaction
pub fn abort<E>(x: E) -> TransactionError<E> {
    TransactionError::Abort(x)
}

#[cfg(test)]
mod tests;
