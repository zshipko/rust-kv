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
//!     let mut cfg = Config::new("/tmp/rust-kv");
//!
//!     // Open the key/value store
//!     let store = Store::new(cfg)?;
//!
//!     // A Bucket provides typed access to a section of the key/value store
//!     let bucket = store.bucket::<Raw, String>(Some("test"))?;
//!
//!     // Set testing = 123
//!     bucket.set(b"testing", "123")?;
//!
//!     let bucket = store.bucket::<String, Json<SomeType>>(None)?;
//!
//!     let x = SomeType {a: 1, b: 2};
//!
//!     bucket.set("example", Json(x))?;
//!
//!     let x: Json<SomeType> = bucket.get("example")?.unwrap();
//!
//!     for item in bucket.iter() {
//!         let item = item?;
//!         let key: String = item.key()?;
//!         let value = item.value::<Json<SomeType>>()?;
//!         println!("key: {}, value: {}", key, value);
//!     }
//!
//!
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
mod types;
mod value;

pub use bucket::{Batch, Bucket, Iter};
pub use config::Config;
pub use error::Error;
pub use store::Store;
pub use types::{Integer, Key, Raw, Value};
pub use value::*;

#[cfg(test)]
mod tests;
