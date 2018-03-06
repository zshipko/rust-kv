# kv

<a href="https://crates.io/crates/kv">
    <img src="https://img.shields.io/crates/v/kv.svg">
</a>

A simple embedded key/value store for Rust built on [LMDB](https://github.com/LMDB/lmdb)


## Example


```rust
// Configuration
let mut cfg = Config::default(path);
cfg.bucket("test", None);

// Create a manager
let mut mgr = Manager::new();

// Get a Store handle
let handle = mgr.open(cfg).unwrap();
let store = handle.write().unwrap();

// Load a bucket
let bucket = store.bucket::<&str, &str>(Some("test")).unwrap();

/// Write a value
let mut txn = store.write_txn().unwrap();
txn.set(&bucket, "testing", "abc123").unwrap();
txn.commit().unwrap();

/// Read a value
let txn = store.read_txn().unwrap();
let val = txn.get(&bucket, "testing").unwrap();
println!("testing => {}", val);
txn.abort();
```

See [https://docs.rs/kv](https://docs.rs/kv) for more information

## Features

* `cbor-value`
    - CBOR value encoding using `serde`
* `json-value`
    - JSON value encoding using `serde`


