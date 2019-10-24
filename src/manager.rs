// Copyright 2018 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

use crate::config::Config;
use crate::error::Error;
use crate::store::Store;

pub type Handle = Arc<RwLock<Store>>;

/// A process is only permitted to have one open handle to each database. This manager
/// exists to enforce that constraint: don't open databases directly.
#[derive(Default)]
pub struct Manager {
    stores: Mutex<BTreeMap<PathBuf, Handle>>,
}

impl Manager {
    /// Create a new store manager
    pub fn new() -> Manager {
        Manager {
            stores: Mutex::new(Default::default()),
        }
    }

    /// Return the open store at `path`, returning `None` if it has not already been opened.
    pub fn get<P>(&self, path: P) -> Result<Option<Handle>, Error>
    where
        P: AsRef<Path>,
    {
        let canonical = path.as_ref().canonicalize()?;
        Ok(self.stores.lock()?.get(&canonical).cloned())
    }

    /// Return the open store at cfg.path, or create it using the given config.
    pub fn open(&mut self, cfg: Config) -> Result<Handle, Error> {
        let _ = fs::create_dir_all(&cfg.path);
        let canonical = cfg.path.as_path().canonicalize()?;
        let mut map = self.stores.lock()?;
        Ok(match map.entry(canonical) {
            Entry::Occupied(e) => e.get().clone(),
            Entry::Vacant(e) => {
                let k = Arc::new(RwLock::new(Store::new(cfg)?));
                e.insert(k).clone()
            }
        })
    }

    /// Load a store from a configuration file
    pub fn load_config_and_open<P>(&mut self, path: P) -> Result<Handle, Error>
    where
        P: AsRef<Path>,
    {
        let config = Config::load(path)?;
        self.open(config)
    }
}
