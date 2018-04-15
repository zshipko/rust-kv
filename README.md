# kv

<a href="https://crates.io/crates/kv">
    <img src="https://img.shields.io/crates/v/kv.svg">
</a>

An embedded key/value store for Rust built on [LMDB](https://github.com/LMDB/lmdb)

- Easy configuration
- Integer keys
- Serde integration (see the Encoding trait)

## Optional features

* `cbor-value`
    - CBOR value encoding using `serde`
* `json-value`
    - JSON value encoding using `serde`
* `bincode-value`
    - bincode value encoding using `serde`
* `capnp-value`
    - Cap'n Proto value encoding using `capnp`

For more information about implementing your own datatypes using serde see: [github.com/asonix/kv-testing](https://github.com/asonix/kv-testing)

## Documentation

See [https://docs.rs/kv](https://docs.rs/kv)

