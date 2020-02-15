use std::marker::PhantomData;

use crate::{Error, Key, Raw, Value};

/// Provides typed access to the key/value store
pub struct Bucket<'a, K: Key<'a>, V: Value<'a>>(
    pub(crate) sled::Tree,
    PhantomData<K>,
    PhantomData<V>,
    PhantomData<&'a ()>,
);

pub struct Item<K, V>((Raw, Raw), PhantomData<K>, PhantomData<V>);

impl<'a, K: Key<'a>, V: Value<'a>> Item<K, V> {
    /// Get the value associated with the specified key
    pub fn value<T: From<V>>(&'a self) -> Result<T, Error> {
        let x = V::from_raw_value((self.0).1.clone())?;
        Ok(x.into())
    }

    /// Get the value associated with the specified key
    pub fn key<T>(&'a self) -> Result<T, Error>
    where
        K: Into<T>,
    {
        let k = K::from_raw_key(&(self.0).0)?;
        Ok(k.into())
    }
}

/// Iterator over Bucket keys and values
pub struct Iter<K, V>(sled::Iter, PhantomData<K>, PhantomData<V>);

impl<'a, K, V> Iterator for Iter<K, V>
where
    K: Key<'a>,
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

impl<'a, K, V> DoubleEndedIterator for Iter<K, V>
where
    K: Key<'a>,
    V: Value<'a>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0.next_back() {
            None => None,
            Some(Err(e)) => Some(Err(e.into())),
            Some(Ok((k, v))) => Some(Ok(Item((k, v), PhantomData, PhantomData))),
        }
    }
}

impl<'a, K: Key<'a>, V: Value<'a>> Bucket<'a, K, V> {
    pub(crate) fn new(t: sled::Tree) -> Bucket<'a, K, V> {
        Bucket(t, PhantomData, PhantomData, PhantomData)
    }

    /// Get the value associated with the specified key
    pub fn get<X: Into<K>>(&'a self, key: X) -> Result<Option<V>, Error> {
        let v = self.0.get(key.into().to_raw_key()?)?;

        match v {
            None => Ok(None),
            Some(x) => Ok(Some(V::from_raw_value(x)?)),
        }
    }

    /// Set the value associated with the specified key to the provided value
    pub fn set<X: Into<K>, Y: Into<V>>(&self, key: X, value: Y) -> Result<(), Error> {
        let v = value.into().to_raw_value()?;
        self.0.insert(key.into().to_raw_key()?, v)?;
        Ok(())
    }

    /// Remove the value associated with the specified key from the database
    pub fn remove<X: Into<K>>(&self, key: X) -> Result<(), Error> {
        self.0.remove(key.into().to_raw_key()?)?;
        Ok(())
    }

    /// Get an iterator over keys/values
    pub fn iter(&self) -> Iter<K, V> {
        Iter(self.0.iter(), PhantomData, PhantomData)
    }

    /// Get an iterator over keys/values in the specified range
    pub fn iter_range<X: Into<K>>(&self, a: X, b: X) -> Iter<K, V> {
        let a = a.into();
        let b = b.into();
        Iter(self.0.range(a..b), PhantomData, PhantomData)
    }

    /// Iterate over keys/values with the specified prefix
    pub fn iter_prefix<X: Into<K>>(&self, a: X) -> Iter<K, V> {
        let a = a.into();
        Iter(self.0.scan_prefix(a), PhantomData, PhantomData)
    }

    /// Apply batch update
    pub fn batch(&self, batch: Batch<K, V>) -> Result<(), Error> {
        self.0.apply_batch(batch.0)?;
        Ok(())
    }
}

/// Batch update
pub struct Batch<K, V>(sled::Batch, PhantomData<K>, PhantomData<V>);

impl<'a, K: Key<'a>, V: Value<'a>> Batch<K, V> {
    /// Set the value associated with the specified key to the provided value
    pub fn set<X: Into<K>>(&mut self, key: X, value: &V) -> Result<(), Error> {
        let v = value.to_raw_value()?;
        self.0.insert(key.into().to_raw_key()?, v);
        Ok(())
    }

    /// Remove the value associated with the specified key from the database
    pub fn remove<X: Into<K>>(&mut self, key: X) -> Result<(), Error> {
        self.0.remove(key.into().to_raw_key()?);
        Ok(())
    }
}
