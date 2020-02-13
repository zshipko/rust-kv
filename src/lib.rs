#![deny(missing_docs)]

//! `kv` is a simple way to embed a key/value store in Rust applications. It is built using
//! [sled](https://docs.rs/sled) and aims to be as lightweight as possible,
//! while still providing a nice high level interface.
//!
//! ## Getting started
//!
//! ```rust
//! use kv::{Config, Error, Store, Raw};
//!
//! fn run() -> Result<(), Error> {
//!     // Configure the database
//!     let mut cfg = Config::new("/tmp/rust-kv");
//!
//!     // Open the key/value store
//!     let store = Store::new(cfg)?;
//!
//!     // A Bucket provides typed access to an LMDB database
//!     let bucket = store.bucket::<&str, Raw>("test")?;
//!
//!     bucket.set("testing", "123")?;
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

pub use bucket::Bucket;
pub use config::Config;
pub use error::Error;
pub use store::Store;
pub use types::{Integer, Key, OwnedKey, Raw, Value};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
