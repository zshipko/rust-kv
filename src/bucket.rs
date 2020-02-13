use std::marker::PhantomData;

use crate::{Error, Key, Value};

/// Provides typed access to the key/value store
pub struct Bucket<'a, K: Key, V: Value<'a>>(
    pub(crate) sled::Tree,
    PhantomData<K>,
    PhantomData<V>,
    PhantomData<&'a ()>,
);

impl<'a, K: Key, V: Value<'a>> Bucket<'a, K, V> {
    pub(crate) fn new(t: sled::Tree) -> Bucket<'a, K, V> {
        Bucket(t, PhantomData, PhantomData, PhantomData)
    }

    /// Get the value associated with the specified key
    pub fn get(&self, key: K) -> Result<Option<V>, Error> {
        let v = self.0.get(key.as_ref())?;

        match v {
            None => Ok(None),
            Some(x) => Value::from_raw_value(x).map(|x| Some(x)),
        }
    }

    /// Set the value associated with the specified key to the provided value
    pub fn set<T: Into<V>>(&self, key: K, value: T) -> Result<(), Error> {
        let v = value.into().to_raw_value();
        self.0.insert(&key, v)?;
        Ok(())
    }

    /// Remove the value associated with the specified key from the database
    pub fn remove(&self, key: K) -> Result<(), Error> {
        self.0.remove(&key)?;
        Ok(())
    }
}
