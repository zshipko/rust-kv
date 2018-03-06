use std::{fs, path};

use config::Config;
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
    use cbor::Cbor;
    use buf::ValueBuf;
    let path = reset("cbor");

    // Create a new store
    let cfg = Config::default(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<&str, ValueBuf<Cbor>>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn().unwrap();
    txn.set(&bucket, "testing", Cbor::from(true)).unwrap();
    txn.commit().unwrap();

    let txn = store.read_txn().unwrap();
    let v = txn.get(&bucket, "testing").unwrap().inner().unwrap();
    assert_eq!(v.as_boolean().unwrap(), true);
    txn.abort();
}

#[cfg(feature = "json-value")]
#[test]
fn test_json_encoding() {
    use json::Json;
    use buf::ValueBuf;
    let path = reset("json");

    // Create a new store
    let cfg = Config::default(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<&str, ValueBuf<Json>>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn().unwrap();
    txn.set(&bucket, "testing", Json::from(true)).unwrap();
    txn.commit().unwrap();

    let txn = store.read_txn().unwrap();
    let v = txn.get(&bucket, "testing").unwrap().inner().unwrap();
    assert_eq!(v.as_bool().unwrap(), true);
    txn.abort();
}

#[test]
fn test_config_encoding() {
    let cfg = Config::default("./test");
    cfg.save("./config").unwrap();
    let cfg2 = Config::load("./config").unwrap();
    assert!(cfg == cfg2);
    let _ = fs::remove_file("./config");
}

#[test]
fn test_manager() {
    let path = reset("manager");

    println!("{}", path);

    // Create a new store
    let mut cfg = Config::default(path.clone());
    cfg.bucket("test", None);
    let mut mgr = Manager::new();

    let handle = mgr.open(cfg).unwrap();
    let store = handle.write().unwrap();
    let bucket = store.bucket::<&str, &str>(Some("test")).unwrap();
    println!("{}", path.as_str());
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn().unwrap();
    txn.set(&bucket, "testing", "abc123").unwrap();
    txn.commit().unwrap();

    let txn = store.read_txn().unwrap();
    assert_eq!(txn.get(&bucket, "testing").unwrap(), "abc123");
    txn.abort();
}
