use std::{fs, path};

use config::{Config, Flag};
use store::Store;
use types::Integer;
use manager::Manager;

fn reset(name: &str) -> String {
    let s = format!("./test/{}", name);
    let _ = fs::remove_dir_all(&s);
    s
}

#[test]
fn test_basic() {
    let path = reset("basic");

    // Create a new store
    let cfg = Config::default(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<&str, &str>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn().unwrap();
    txn.set(&bucket, "testing", "abc123").unwrap();
    txn.commit().unwrap();

    let txn = store.read_txn().unwrap();
    assert_eq!(txn.get(&bucket, "testing").unwrap(), "abc123");
    txn.abort();
}

#[test]
fn test_integer_keys() {
    let path = reset("integer_keys");

    // Create a new store
    let cfg = Config::default(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.int_bucket::<&str>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn().unwrap();
    let key = 0x1234;
    txn.set(&bucket, key.into(), "abc123").unwrap();
    txn.commit().unwrap();

    let txn = store.read_txn().unwrap();
    assert_eq!(txn.get(&bucket, key.into()).unwrap(), "abc123");
    txn.abort();
}

#[test]
fn test_cursor() {
    let path = reset("cursor");

    // Create a new store
    let cfg = Config::default(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.int_bucket::<String>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn().unwrap();

    for i in 0..100 {
        txn.set(&bucket, i.into(), format!("{}", i)).unwrap();
    }

    txn.commit().unwrap();

    let txn = store.read_txn().unwrap();
    {
        let mut cursor = txn.read_cursor(&bucket).unwrap();
        let mut index = 0;

        for (k, v) in cursor.iter() {
            assert_eq!(k, Integer::from(index));
            assert_eq!(v, format!("{}", index));
            index += 1;
        }
    }
    txn.abort();
}

#[cfg(feature = "cbor-value")]
#[test]
fn test_cbor_encoding() {
    use encoding::Serde;
    use cbor::Cbor;
    use buf::ValueBuf;
    let path = reset("cbor");

    // Create a new store
    let cfg = Config::default(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<&str, ValueBuf<Cbor<bool>>>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn().unwrap();
    for i in 0..2 {
        match txn.set_no_overwrite(&bucket, "testing", Cbor::to_value_buf(true).unwrap()) {
            Ok(_) => assert_eq!(i, 0),
            Err(_) => assert_eq!(i, 1),
        }
    }
    txn.commit().unwrap();

    let txn = store.read_txn().unwrap();
    let v = txn.get(&bucket, "testing").unwrap().inner().unwrap();
    assert_eq!(v.to_serde(), true);
    txn.abort();
}

#[cfg(feature = "json-value")]
#[test]
fn test_json_encoding() {
    use encoding::Serde;
    use json::Json;
    use buf::ValueBuf;
    let path = reset("json");

    // Create a new store
    let cfg = Config::default(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<&str, ValueBuf<Json<bool>>>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn().unwrap();
    for i in 0..2 {
        match txn.set_no_overwrite(&bucket, "testing", Json::to_value_buf(true).unwrap()) {
            Ok(_) => assert_eq!(i, 0),
            Err(_) => assert_eq!(i, 1),
        }
    }
    txn.commit().unwrap();

    let txn = store.read_txn().unwrap();
    let v = txn.get(&bucket, "testing").unwrap().inner().unwrap();
    assert_eq!(v.to_serde(), true);
    txn.abort();
}

#[cfg(feature = "bincode-value")]
#[test]
fn test_bincode_encoding() {
    use encoding::Serde;
    use bincode::Bincode;
    use buf::ValueBuf;
    let path = reset("bincode");

    // Create a new store
    let cfg = Config::default(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<&str, ValueBuf<Bincode<i32>>>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn().unwrap();
    for i in 0..2 {
        match txn.set_no_overwrite(
            &bucket,
            "testing",
            Bincode::to_value_buf(12345).unwrap(),
        ) {
            Ok(_) => assert_eq!(i, 0),
            Err(_) => assert_eq!(i, 1),
        }
    }
    txn.commit().unwrap();

    let txn = store.read_txn().unwrap();
    let v = txn.get(&bucket, "testing").unwrap().inner().unwrap();
    assert_eq!(v.to_serde(), 12345);
    txn.abort();
}

#[test]
fn test_config_encoding() {
    let mut cfg = Config::default("./test");
    cfg.bucket("a", None);
    cfg.bucket("b", None);
    cfg.bucket("c", Some(Flag::IntegerKey));
    cfg.save("./config").unwrap();
    let cfg2 = Config::load("./config").unwrap();
    assert!(cfg == cfg2);
    let _ = fs::remove_file("./config");
}

#[test]
fn test_manager() {
    let path = reset("manager");

    // Create a new store
    let mut cfg = Config::default(path.clone());
    cfg.bucket("test", None);
    let mut mgr = Manager::new();

    let handle = mgr.open(cfg).unwrap();
    let store = handle.write().unwrap();
    let bucket = store.bucket::<&str, &str>(Some("test")).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn().unwrap();
    txn.set(&bucket, "testing", "abc123").unwrap();
    txn.commit().unwrap();

    let txn = store.read_txn().unwrap();
    assert_eq!(txn.get(&bucket, "testing").unwrap(), "abc123");
    txn.abort();
}
