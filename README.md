# kv

<a href="https://crates.io/crates/kv">
    <img src="https://img.shields.io/crates/v/kv.svg">
</a>

An embedded key/value store for Rust built on [LMDB](https://github.com/LMDB/lmdb)

- Easy configuration
- Integer keys
- Serde integration (see the Encoding trait)


## Example


```rust
// Create the default configuration
let mut cfg = Config::default(path);

// Load configuration from a TOML file
let mut cfg = Config::load("test.conf");

// Add a bucket named `test`
cfg.bucket("test", None);

// Create a manager
// Managers are used to ensure that each process only has access to one LMDB environment
// reference at a time
let mut mgr = Manager::new();

// Get a Store handle
let handle = mgr.open(cfg).unwrap();

// Get acess to the underlying store
let store = handle.write().unwrap();

// Load a bucket
//
// A reference to a bucket will be the first argument to all functions that
// read or write to the database
let bucket = store.bucket::<&str, &str>(Some("test")).unwrap();

/// Write a value to the store
let mut txn = store.write_txn().unwrap();
txn.set(&bucket, "testing", "abc123").unwrap();

// Don't forget to commit the transaction
txn.commit().unwrap();

/// Read a value
let txn = store.read_txn().unwrap();
let val = txn.get(&bucket, "testing").unwrap();
println!("testing => {}", val);

// You can also abort a transaction
txn.abort();
```

See [https://docs.rs/kv](https://docs.rs/kv) for more information

## Optional

* `cbor-value`
    - CBOR value encoding using `serde`
* `json-value`
    - JSON value encoding using `serde`




