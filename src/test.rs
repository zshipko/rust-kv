use std::{path, fs};

use config::Config;
use store::Store;
use txn::Txn;

const DB_PATH: &'static str = "./test/test.db";

#[test]
fn test_basic(){
    // Delete current store
    let _ = fs::remove_dir_all(DB_PATH);
    let _ = fs::remove_dir_all("./test");

    // Create a new store
    let cfg = Config::default(DB_PATH);
    let store: Store<&str> = Store::new(cfg).unwrap();
    let bucket = store.default().unwrap();
    assert!(path::Path::new(DB_PATH).exists());

    {
        let mut txn = store.write_txn().unwrap();
        txn.set(bucket, "testing", "abc123").unwrap();
        txn.commit().unwrap();
    }

    {
        let txn: Txn<&str> = store.read_txn().unwrap();
        assert_eq!(txn.get(bucket, "testing").unwrap(), "abc123");
    }
}
