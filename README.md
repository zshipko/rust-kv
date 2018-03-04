# kv

<a href="https://crates.io/crates/kv">
    <img src="https://img.shields.io/crates/v/kv.svg">
</a>

A simple embedded key/value store for Rust built on [LMDB](https://github.com/LMDB/lmdb)


## Example


```rust
let cfg = Config::default(DB_PATH);
let store = Store::<&str>::new(cfg).unwrap();
let bucket = store.default().unwrap();

// Set a value
let mut txn = store.write_txn::<&str>().unwrap();
txn.set(bucket, "testing", "abc123").unwrap();
txn.commit().unwrap();

// Get a value from the store
let txn = store.read_txn::<&str>().unwrap();
assert_eq!(txn.get(bucket, "testing").unwrap(), "abc123");
txn.abort();
```

See [https://docs.rs/kv](https://docs.rs/kv) for more information


