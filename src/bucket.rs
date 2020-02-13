use std::marker::PhantomData;

use crate::{Error, FromValue, Integer, Key, Raw, ToValue, Value};

/// Provides typed access to the key/value store
pub struct Bucket<'a, K: Key, V: Value<'a>>(
    pub(crate) sled::Tree,
    PhantomData<K>,
    PhantomData<V>,
    PhantomData<&'a ()>,
);

pub struct Item<K, V>((Raw, Raw), PhantomData<K>, PhantomData<V>);

impl<'a, K, V: Value<'a>> Item<K, V> {
    /// Get the value associated with the specified key
    pub fn value<T: FromValue<V>>(self) -> Result<T, Error> {
        let x = V::from_raw_value((self.0).1)?;
        let x = T::from_value(x)?;
        Ok(x)
    }
}

impl<'a, V: Value<'a>> Item<&str, V> {
    /// Get the value associated with the specified key
    pub fn key(&self) -> Result<&str, Error> {
        Ok(std::str::from_utf8((self.0).0.as_ref())?)
    }
}

impl<'a, V: Value<'a>> Item<String, V> {
    /// Get the value associated with the specified key
    pub fn key(&self) -> Result<String, Error> {
        Ok(std::str::from_utf8((self.0).0.as_ref())?.to_string())
    }
}

impl<'a, V: Value<'a>> Item<&[u8], V> {
    /// Get the value associated with the specified key
    pub fn key(&self) -> Result<&[u8], Error> {
        Ok((self.0).0.as_ref())
    }
}

impl<'a, V: Value<'a>> Item<Integer, V> {
    /// Get the value associated with the specified key
    pub fn key(&self) -> Result<Integer, Error> {
        Ok(Integer::from((self.0).0.as_ref()))
    }
}

impl<'a, V: Value<'a>> Item<Raw, V> {
    /// Get the value associated with the specified key
    pub fn key(&self) -> Result<&Raw, Error> {
        Ok(&(self.0).0)
    }
}

/// Iterator over Bucket keys and values
pub struct Iter<K, V>(sled::Iter, PhantomData<K>, PhantomData<V>);

impl<'a, K, V> Iterator for Iter<K, V>
where
    K: Key,
    V: Value<'a>,
{
    type Item = Result<Item<K, V>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(Err(e)) => Some(Err(e.into())),
            Some(Ok((k, v))) => Some(Ok(Item((k, v), PhantomData, PhantomData))),
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
