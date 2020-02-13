use std::marker::PhantomData;

use crate::{Error, FromValue, Key, OwnedKey, ToValue, Value};

/// Provides typed access to the key/value store
pub struct Bucket<'a, K: Key, V: Value<'a>>(
    pub(crate) sled::Tree,
    PhantomData<K>,
    PhantomData<V>,
    PhantomData<&'a ()>,
);

/// Iterator over Bucket keys and values
pub struct Iter<K, V>(sled::Iter, PhantomData<K>, PhantomData<V>);

impl<'a, K, V> Iterator for Iter<K, V>
where
    K: OwnedKey<'a>,
    V: Value<'a>,
{
    type Item = Result<(K, V), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(Err(e)) => Some(Err(e.into())),
            Some(Ok((k, v))) => {
                let k = match OwnedKey::from_raw_key(k) {
                    Ok(k) => k,
                    Err(e) => return Some(Err(e)),
                };
                let v = match Value::from_raw_value(v) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                Some(Ok((k, v)))
            }
        }
    }
}

impl<'a, K: Key, V: Value<'a>> Bucket<'a, K, V> {
    pub(crate) fn new(t: sled::Tree) -> Bucket<'a, K, V> {
        Bucket(t, PhantomData, PhantomData, PhantomData)
    }

    /// Get the value associated with the specified key
    pub fn get<T: FromValue<V>>(&self, key: K) -> Result<Option<T>, Error> {
        let v = self.0.get(key.to_raw_key())?;

        match v {
            None => Ok(None),
            Some(x) => {
                let x = V::from_raw_value(x)?;
                let x = T::from_value(x)?;
                Ok(Some(x))
            }
        }
    }

    /// Set the value associated with the specified key to the provided value
    pub fn set<T: ToValue<V>>(&self, key: K, value: T) -> Result<(), Error> {
        let v = value.to_value()?.to_raw_value();
        self.0.insert(key.to_raw_key(), v)?;
        Ok(())
    }

    /// Remove the value associated with the specified key from the database
    pub fn remove(&self, key: K) -> Result<(), Error> {
        self.0.remove(key.to_raw_key())?;
        Ok(())
    }

    /// Get an iterator over keys/values
    pub fn iter(&self) -> Iter<K, V> {
        Iter(self.0.iter(), PhantomData, PhantomData)
    }
}
