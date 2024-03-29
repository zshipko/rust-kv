use std::{fs, path};

use crate::*;

fn reset(name: &str) -> String {
    let s = format!("./test/{}", name);
    let _ = fs::remove_dir_all(&s);
    s
}

#[test]
fn test_basic() {
    let path = reset("basic");

    // Create a new store
    let cfg = Config::new(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<Raw, Raw>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let key = Raw::from("testing");
    let value = Raw::from(b"abc123");
    bucket.set(&key, &value).unwrap();
    assert_eq!(bucket.get(&key).unwrap().unwrap(), b"abc123");
}

#[test]
fn test_integer_keys() {
    let path = reset("integer_keys");

    let cfg = Config::new(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<Integer, Raw>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let key = Integer::from(0x1234);
    bucket.set(&key, &Raw::from("abc123")).unwrap();
    assert_eq!(bucket.get(&key).unwrap().unwrap(), b"abc123");
}

#[test]
fn test_iter() {
    let path = reset("iter");

    let cfg = Config::new(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<Integer, String>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    for i in 0..100 {
        bucket.set(&i.into(), &format!("{}", i)).unwrap();
    }

    let iter = bucket.iter();

    iter.enumerate().for_each(|(index, item)| {
        let item = item.unwrap();
        let key: u128 = item.key().unwrap();
        assert_eq!(key, index as u128);
        assert_eq!(item.value::<String>().unwrap(), format!("{}", index));
    });
}

#[cfg(feature = "msgpack-value")]
#[test]
fn test_msgpack_encoding() {
    use crate::Msgpack;
    let path = reset("msgpack");

    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct Testing {
        a: i32,
        b: String,
    }

    let cfg = Config::new(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<&str, Msgpack<Testing>>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let key = "testing";
    bucket
        .set(
            &key,
            &Msgpack(Testing {
                a: 1,
                b: "field".into(),
            }),
        )
        .unwrap();

    let v = bucket.get(&key).unwrap();
    assert_eq!(
        v.unwrap().0,
        Testing {
            a: 1,
            b: "field".into(),
        }
    );
}

#[cfg(feature = "json-value")]
#[test]
fn test_json_encoding() {
    use crate::Json;
    let path = reset("json");

    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct Testing {
        a: i32,
        b: String,
    }

    let cfg = Config::new(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<&str, Json<Testing>>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let key = "testing";
    bucket
        .set(
            &key,
            &Json(Testing {
                a: 1,
                b: "field".into(),
            }),
        )
        .unwrap();

    let v = bucket.get(&key).unwrap();
    assert_eq!(
        v.unwrap().0,
        Testing {
            a: 1,
            b: "field".into(),
        }
    );
}

#[cfg(feature = "bincode-value")]
#[test]
fn test_bincode_encoding() {
    use crate::Bincode;
    let path = reset("bincode");

    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct Testing {
        a: i32,
        b: String,
    }

    let cfg = Config::new(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<&str, Bincode<Testing>>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let key = "testing";
    bucket
        .set(
            &key,
            &Bincode(Testing {
                a: 1,
                b: "field".into(),
            }),
        )
        .unwrap();

    let v = bucket.get(&key).unwrap();
    assert_eq!(
        v.unwrap().0,
        Testing {
            a: 1,
            b: "field".into(),
        }
    );
}

#[cfg(feature = "lexpr-value")]
#[test]
fn test_sexpr_encoding() {
    use crate::Lexpr;
    let path = reset("sexpr");

    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct Testing {
        a: i32,
        b: String,
    }

    let cfg = Config::new(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<&str, Lexpr<Testing>>(None).unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let key = "testing";
    let value = Lexpr(Testing {
        a: 1,
        b: "field".into(),
    });

    bucket.set(&key, &value).unwrap();
    let v = bucket.get(&key).unwrap();
    assert_eq!(
        v.unwrap().0,
        Testing {
            a: 1,
            b: "field".into(),
        }
    );
}

#[test]
fn test_config_encoding() {
    let mut cfg = Config::new("./test");
    cfg.use_compression = true;
    cfg.save("./config").unwrap();
    let cfg2 = Config::load("./config").unwrap();
    assert!(cfg == cfg2);
    let _ = fs::remove_file("./config");
}

#[test]
fn test_watch() {
    let path = reset("watch");
    let cfg = Config::new(path.clone());
    let store = Store::new(cfg).unwrap();
    let bucket = store.bucket::<&str, Raw>(Some("watch")).unwrap();
    let mut watch = bucket.watch_prefix(None).unwrap();

    let key = "abc";
    bucket.set(&key, &Raw::from(b"123")).unwrap();

    let next = watch.next().unwrap();
    let next = next.unwrap();

    assert!(next.is_set());
    assert!(next.value().unwrap().unwrap() == b"123");
    assert!(next.key().unwrap() == "abc");

    bucket.remove(&key).unwrap();

    let next = watch.next().unwrap();
    let next = next.unwrap();

    assert!(next.is_remove());
    assert!(next.value().unwrap() == None);
    assert!(next.key().unwrap() == "abc");
}
