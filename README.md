# kv

<a href="https://crates.io/crates/kv">
    <img src="https://img.shields.io/crates/v/kv.svg">
</a>

An embedded key/value store for Rust built on [sled](https://docs.rs/sled)

- Easy configuration
- Integer keys
- Serde integration

Note: `kv` `0.20` and greater have been completely re-written to use [sled](https://docs.rs/sled) instead of [LMDB](https://github.com/LMDB/lmdb). In the process the entire API has been redesigned and simplified significantly. If you still need to use LMDB or don't like the new interface then you might want to check out [rkv](https://docs.rs/rkv).

## Optional features

* `msgpack-value`
    - MessagePack encoding using `rmp-serde`
* `json-value`
    - JSON encoding using `serde_json`
* `bincode-value`
    - bincode encoding using `bincode`
* `lexpr-value`
    - S-expression encoding using `serde-lexpr`

## Documentation

See [https://docs.rs/kv](https://docs.rs/kv)

