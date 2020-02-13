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
//!     let bucket = store.bucket::<&str, Raw>(Some("test"))?;
//!
//!     bucket.set("testing", "123")?;
//!
//!     let bucket = store.bucket::<&str, Buffer<Json<SomeType>>>(None)?;
//!
//!     let x = SomeType {a: 1, b: 2};
//!
//!     bucket.set("example", Json(&x))?;
//!
//!     let x: SomeType = bucket.get("example")?.unwrap();
//!
//!     for item in bucket.iter() {
//!         let item = item?;
//!         println!("{}", item.key().unwrap());
//!         assert!(item.key().unwrap() == "example");
//!         assert!(item.value::<SomeType>().unwrap() == x);
//!
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

pub use bucket::{Bucket, Iter};
pub use config::Config;
pub use error::Error;
pub use store::Store;
pub use types::{Buffer, FromValue, Integer, Key, OwnedKey, Raw, ToValue, Value};
pub use value::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
