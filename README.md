# kv

<a href="https://crates.io/crates/kv">
    <img src="https://img.shields.io/crates/v/kv.svg">
</a>

An embedded key/value store for Rust built on [sled](https://docs.rs/sled)

- Easy configuration
- Integer keys
- Serde integration

Note: `kv` `0.20` and greater have been completely re-written to use [sled](https://docs.rs/sled) instead of LMDB, in the process the entire API has been rewritten.

## Optional features

* `msgpack-value`
    - MessagePack encoding using `serde`
* `json-value`
    - JSON value encoding using `serde`
* `bincode-value`
    - bincode value encoding using `serde`

Some examples of implementing your own encodings using serde can be found [here](https://github.com/asonix/kv-testing)

## Documentation

See [https://docs.rs/kv](https://docs.rs/kv)

