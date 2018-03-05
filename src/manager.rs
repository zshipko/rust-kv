// Copyright 2018 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use lmdb;

use std::collections::{
    BTreeMap,
};

use std::collections::btree_map::{
    Entry,
};

use std::path::{
    Path,
    PathBuf,
};

use std::sync::{
    Arc,
    Mutex,
    RwLock,
};

use error::{
    Error,
};

use store::{
    Store
};

use types::{
    Key
};

use config::{
    Config
};

#[derive(Debug)]
pub struct Handle {
    env: lmdb::Environment,
    cfg: Config,
}

impl Handle {
    pub fn store<K: Key>(self) -> Result<Store<K>, Error> {
        Store::wrap(self.env, self.cfg)
    }
}

/// A process is only permitted to have one open handle to each database. This manager
/// exists to enforce that constraint: don't open databases directly.
pub struct Manager {
    stores: Mutex<BTreeMap<PathBuf, Arc<RwLock<Handle>>>>,
}

impl Manager {
    /// Create a new store manager
    pub fn new() -> Manager {
        Manager {
            stores: Mutex::new(Default::default()),
        }
    }

    /// Return the open store at `path`, returning `None` if it has not already been opened.
    pub fn get<'p, P, T>(&self, path: P) -> Result<Option<Arc<RwLock<Handle>>>, Error>
    where P: Into<&'p Path>,
          T: Key
    {
        let canonical = path.into().canonicalize()?;
        Ok(self.stores.lock().unwrap().get(&canonical).cloned())
    }

    /// Return the open store at cfg.path, or create it using the given config.
    pub fn get_or_create<T>(&mut self, mut cfg: Config) -> Result<Arc<RwLock<Handle>>, Error>
    where T: Key
    {
        let canonical = cfg.path.as_path().canonicalize()?;
        let mut map = self.stores.lock().unwrap();
        Ok(match map.entry(canonical) {
            Entry::Occupied(e) => e.get().clone(),
            Entry::Vacant(e) => {
                let env = cfg.env()?;
                let k = Arc::new(RwLock::new(Handle{
                    cfg: cfg,
                    env: env
                }));
                e.insert(k).clone()
            }
        })
    }
}

#[cfg(test)]
mod test {
    extern crate tempdir;

    use self::tempdir::TempDir;
    use std::fs;

    use super::*;

    /// Test that the manager will return the same Handle instance each time for each path.
    #[test]
    fn test_same() {
        let root = TempDir::new("test_same").expect("tempdir");
        fs::create_dir_all(root.path()).expect("dir created");

        let mut manager = Manager::new();

        let p = root.path();
        assert!(manager.get::<_, &str>(p).expect("success").is_none());

        let created_arc = manager.get_or_create::<&str>(Config::default(p)).expect("created");
        let fetched_arc = manager.get::<_, &str>(p).expect("success").expect("existed");
        assert!(Arc::ptr_eq(&created_arc, &fetched_arc));
    }
}
