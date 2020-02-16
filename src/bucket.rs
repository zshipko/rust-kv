use std::marker::PhantomData;

use crate::{Error, Key, Raw, Transaction, TransactionError, Value};

/// Provides typed access to the key/value store
pub struct Bucket<'a, K: Key<'a>, V: Value>(
    pub(crate) sled::Tree,
    PhantomData<K>,
    PhantomData<V>,
    PhantomData<&'a ()>,
);

/// Iterator item
pub struct Item<K, V>(Raw, Raw, PhantomData<K>, PhantomData<V>);

/// Batch update
pub struct Batch<K, V>(pub(crate) sled::Batch, PhantomData<K>, PhantomData<V>);

/// Subscribe to key updated
pub struct Watch<K, V>(sled::Subscriber, PhantomData<K>, PhantomData<V>);

/// Event is used to describe the type of update
pub enum Event<K, V> {
    /// A key has been updated
    Insert(Item<K, V>),
    /// A key has been removed
    Remove(Raw),
}

impl<'a, K: Key<'a>, V> Iterator for Watch<K, V> {
    type Item = Result<Event<K, V>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(sled::Event::Insert(k, v)) => {
                let k: Raw = k.into();
                Some(Ok(Event::Insert(Item(k, v, PhantomData, PhantomData))))
            }
            Some(sled::Event::Remove(k)) => {
                let k: Raw = k.into();
                Some(Ok(Event::Remove(k)))
            }
        }
    }
}

impl<'a, K: Key<'a>, V: Value> Event<K, V> {
    /// Returns true when event is insert
    pub fn is_insert(&self) -> bool {
        match self {
            Event::Insert(_) => true,
            _ => false,
        }
    }

    /// Returns true when event is remove
    pub fn is_remove(&self) -> bool {
        match self {
            Event::Remove(_) => true,
            _ => false,
        }
    }

    /// Get event key
    pub fn key(&'a self) -> Result<K, Error> {
        match self {
            Event::Remove(k) => K::from_raw_key(k),
            Event::Insert(item) => item.key(),
        }
    }

    /// Get event value (for insert)
    pub fn value(&'a self) -> Result<Option<V>, Error> {
        match self {
            Event::Remove(_) => Ok(None),
            Event::Insert(item) => item.value().map(Some),
        }
    }
}

impl<'a, K: Key<'a>, V: Value> Item<K, V> {
    /// Get the value associated with the specified key
    pub fn value<T: From<V>>(&'a self) -> Result<T, Error> {
        let x = V::from_raw_value((self.1).clone())?;
        Ok(x.into())
    }

    /// Get the value associated with the specified key
    pub fn key<T>(&'a self) -> Result<T, Error>
    where
        K: Into<T>,
    {
        let k = K::from_raw_key(&self.0)?;
        Ok(k.into())
    }
}

/// Iterator over Bucket keys and values
pub struct Iter<K, V>(sled::Iter, PhantomData<K>, PhantomData<V>);

impl<'a, K, V> Iterator for Iter<K, V>
where
    K: Key<'a>,
    V: Value,
{
    type Item = Result<Item<K, V>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(Err(e)) => Some(Err(e.into())),
            Some(Ok((k, v))) => Some(Ok(Item(k, v, PhantomData, PhantomData))),
        }
    }
}

impl<'a, K, V> DoubleEndedIterator for Iter<K, V>
where
    K: Key<'a>,
    V: Value,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0.next_back() {
            None => None,
            Some(Err(e)) => Some(Err(e.into())),
            Some(Ok((k, v))) => Some(Ok(Item(k, v, PhantomData, PhantomData))),
        }
    }
}

impl<'a, K: Key<'a>, V: Value> Bucket<'a, K, V> {
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

    /// Get updates when a key with the given prefix is changed
    pub fn watch_prefix<X: Into<K>>(&self, prefix: X) -> Result<Watch<K, V>, Error> {
        let w = self.0.watch_prefix(prefix.into());
        Ok(Watch(w, PhantomData, PhantomData))
    }

    /// Execute a transaction
    pub fn transaction<A, F: Fn(Transaction<K, V>) -> Result<A, TransactionError>>(
        &self,
        f: F,
    ) -> Result<A, Error> {
        let result = self.0.transaction(|t| {
            let txn = Transaction::new(t);
            f(txn)
        });

        match result {
            Ok(x) => Ok(x),
            Err(sled::TransactionError::Abort(x)) => Err(x),
            Err(sled::TransactionError::Storage(e)) => Err(e.into()),
        }
    }
}

impl<'a, K: Key<'a>, V: Value> Batch<K, V> {
    /// Create a new Batch instance
    pub fn new() -> Batch<K, V> {
        Batch(sled::Batch::default(), PhantomData, PhantomData)
    }

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
