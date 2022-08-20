use pin_project_lite::pin_project;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use sled::Transactional;

use crate::{Error, Key, Raw, Transaction, TransactionError, Value};

/// Provides typed access to the key/value store
#[derive(Clone)]
pub struct Bucket<'a, K: Key<'a>, V: Value>(
    pub(crate) sled::Tree,
    PhantomData<K>,
    PhantomData<V>,
    PhantomData<&'a ()>,
);

/// Key/value pair
#[derive(Clone)]
pub struct Item<K, V>(Raw, Raw, PhantomData<K>, PhantomData<V>);

/// Batch update
#[derive(Clone)]
pub struct Batch<K, V>(pub(crate) sled::Batch, PhantomData<K>, PhantomData<V>);

pin_project! {
    /// Subscribe to key updated
    pub struct Watch<K, V> {
        #[pin]
        subscriber: sled::Subscriber,
        phantom: PhantomData<(K, V)>
    }
}

/// Event is used to describe the type of update
pub enum Event<K, V> {
    /// A key has been updated
    Set(Item<K, V>),
    /// A key has been removed
    Remove(Raw),
}

impl<'a, K: Key<'a>, V> Event<K, V> {
    fn from_sled(event: sled::Event) -> Self {
        match event {
            sled::Event::Insert { key, value } => {
                Event::Set(Item(key, value, PhantomData, PhantomData))
            }
            sled::Event::Remove { key } => Event::Remove(key),
        }
    }
}

impl<'a, K: Key<'a>, V> Iterator for Watch<K, V> {
    type Item = Result<Event<K, V>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.subscriber.next() {
            None => None,
            Some(e) => Some(Ok(Event::from_sled(e))),
        }
    }
}

impl<'a, K: Key<'a>, V> Future for Watch<K, V> {
    type Output = Option<Event<K, V>>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.subscriber.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(r) => Poll::Ready(r.map(Event::from_sled)),
        }
    }
}

impl<'a, K: Key<'a>, V: Value> Event<K, V> {
    /// Returns true when event is `Set`
    pub fn is_set(&self) -> bool {
        matches!(self, Event::Set(_))
    }

    /// Returns true when event is `Remove`
    pub fn is_remove(&self) -> bool {
        matches!(self, Event::Remove(_))
    }

    /// Get event key
    pub fn key(&'a self) -> Result<K, Error> {
        match self {
            Event::Remove(k) => K::from_raw_key(k),
            Event::Set(item) => item.key(),
        }
    }

    /// Get event value (for insert)
    pub fn value(&'a self) -> Result<Option<V>, Error> {
        match self {
            Event::Remove(_) => Ok(None),
            Event::Set(item) => item.value().map(Some),
        }
    }
}

impl<'a, K: Key<'a>, V: Value> Item<K, V> {
    /// Get the value associated with the specified key
    pub fn value<T: From<V>>(&'a self) -> Result<T, Error> {
        let x = V::from_raw_value(self.1.clone())?;
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

    /// Returns true if the bucket contains the given key
    pub fn contains(&self, key: &K) -> Result<bool, Error> {
        let v = self.0.contains_key(key.to_raw_key()?)?;
        Ok(v)
    }

    /// Get the value associated with the specified key
    pub fn get(&self, key: &K) -> Result<Option<V>, Error> {
        let v = self.0.get(key.to_raw_key()?)?;

        match v {
            None => Ok(None),
            Some(x) => Ok(Some(V::from_raw_value(x)?)),
        }
    }

    /// Set the value associated with the specified key to the provided value
    pub fn set(&self, key: &K, value: &V) -> Result<Option<V>, Error> {
        let v = value.to_raw_value()?;
        Ok(self
            .0
            .insert(key.to_raw_key()?, v)?
            .map(|x| V::from_raw_value(x))
            // https://users.rust-lang.org/t/convenience-method-for-flipping-option-result-to-result-option/13695/7
            .map_or(Ok(None), |v| v.map(Some))?)
    }

    /// Set the value associated with the specified key to the provided value, only if the existing
    /// value matches the `old` parameter
    pub fn compare_and_swap(
        &self,
        key: &K,
        old: Option<&V>,
        value: Option<&V>,
    ) -> Result<(), Error> {
        let old = match old {
            Some(x) => Some(x.to_raw_value()?),
            None => None,
        };

        let value = match value {
            Some(x) => Some(x.to_raw_value()?),
            None => None,
        };

        let a = self.0.compare_and_swap(key.to_raw_key()?, old, value)?;

        Ok(a?)
    }

    /// Remove the value associated with the specified key from the database
    pub fn remove(&self, key: &K) -> Result<Option<V>, Error> {
        Ok(self
            .0
            .remove(key.to_raw_key()?)?
            .map(|x| V::from_raw_value(x))
            // https://users.rust-lang.org/t/convenience-method-for-flipping-option-result-to-result-option/13695/7
            .map_or(Ok(None), |v| v.map(Some))?)
    }

    /// Get an iterator over keys/values
    pub fn iter(&self) -> Iter<K, V> {
        Iter(self.0.iter(), PhantomData, PhantomData)
    }

    /// Get an iterator over keys/values in the specified range
    pub fn iter_range(&self, a: &K, b: &K) -> Result<Iter<K, V>, Error> {
        let a = a.to_raw_key()?;
        let b = b.to_raw_key()?;
        Ok(Iter(self.0.range(a..b), PhantomData, PhantomData))
    }

    /// Iterate over keys/values with the specified prefix
    pub fn iter_prefix(&self, a: &K) -> Result<Iter<K, V>, Error> {
        let a = a.to_raw_key()?;
        Ok(Iter(self.0.scan_prefix(a), PhantomData, PhantomData))
    }

    /// Apply batch update
    pub fn batch(&self, batch: Batch<K, V>) -> Result<(), Error> {
        self.0.apply_batch(batch.0)?;
        Ok(())
    }

    /// Get updates when a key with the given prefix is changed
    pub fn watch_prefix(&self, prefix: Option<&K>) -> Result<Watch<K, V>, Error> {
        let k = match prefix {
            Some(k) => k.to_raw_key()?,
            None => b"".into(),
        };
        let subscriber = self.0.watch_prefix(k);
        Ok(Watch {
            subscriber,
            phantom: PhantomData {},
        })
    }

    /// Execute a transaction
    pub fn transaction<
        A,
        E: From<sled::Error>,
        F: Fn(Transaction<K, V>) -> Result<A, TransactionError<E>>,
    >(
        &self,
        f: F,
    ) -> Result<A, E> {
        let result = self.0.transaction(|t| {
            let txn = Transaction::new(t);
            f(txn)
        });

        match result {
            Ok(x) => Ok(x),
            Err(sled::transaction::TransactionError::Abort(x)) => Err(x),
            Err(sled::transaction::TransactionError::Storage(e)) => Err(e.into()),
        }
    }

    /// Create a transaction with access to two buckets
    pub fn transaction2<
        A,
        T: Key<'a>,
        U: Value,
        E: From<sled::Error>,
        F: Fn(Transaction<K, V>, Transaction<T, U>) -> Result<A, TransactionError<E>>,
    >(
        &self,
        other: &Bucket<'a, T, U>,
        f: F,
    ) -> Result<A, E> {
        let result = (&self.0, &other.0).transaction(|(a, b)| {
            let a = Transaction::new(a);
            let b = Transaction::new(b);
            f(a, b)
        });

        match result {
            Ok(x) => Ok(x),
            Err(sled::transaction::TransactionError::Abort(x)) => Err(x),
            Err(sled::transaction::TransactionError::Storage(e)) => Err(e.into()),
        }
    }

    /// Create a transaction with access to three buckets
    pub fn transaction3<
        A,
        T: Key<'a>,
        U: Value,
        X: Key<'a>,
        Y: Value,
        E: From<sled::Error>,
        F: Fn(
            Transaction<K, V>,
            Transaction<T, U>,
            Transaction<X, Y>,
        ) -> Result<A, TransactionError<E>>,
    >(
        &self,
        other: &Bucket<'a, T, U>,
        other1: &Bucket<'a, X, Y>,
        f: F,
    ) -> Result<A, E> {
        let result = (&self.0, &other.0, &other1.0).transaction(|(a, b, c)| {
            let a = Transaction::new(a);
            let b = Transaction::new(b);
            let c = Transaction::new(c);
            f(a, b, c)
        });

        match result {
            Ok(x) => Ok(x),
            Err(sled::transaction::TransactionError::Abort(x)) => Err(x),
            Err(sled::transaction::TransactionError::Storage(e)) => Err(e.into()),
        }
    }

    /// Get previous key and value in order, if one exists
    pub fn prev_key(&self, key: &K) -> Result<Option<Item<K, V>>, Error> {
        let item = self.0.get_lt(key)?;
        Ok(item.map(|(k, v)| Item(k, v, PhantomData, PhantomData)))
    }

    /// Get next key and value in order, if one exists
    pub fn next_key(&self, key: &K) -> Result<Option<Item<K, V>>, Error> {
        let item = self.0.get_gt(key)?;
        Ok(item.map(|(k, v)| Item(k, v, PhantomData, PhantomData)))
    }

    /// Flush to disk
    pub fn flush(&self) -> Result<usize, Error> {
        Ok(self.0.flush()?)
    }

    /// Flush to disk
    pub async fn flush_async(&self) -> Result<usize, Error> {
        let f = self.0.flush_async().await?;
        Ok(f)
    }

    /// Remove and return the last item
    pub fn pop_back(&self) -> Result<Option<Item<K, V>>, Error> {
        let x = self.0.pop_max()?;
        Ok(x.map(|(k, v)| Item(k, v, PhantomData, PhantomData)))
    }

    /// Remove and return the first item
    pub fn pop_front(&self) -> Result<Option<Item<K, V>>, Error> {
        let x = self.0.pop_min()?;
        Ok(x.map(|(k, v)| Item(k, v, PhantomData, PhantomData)))
    }

    /// Get the first item
    pub fn first(&self) -> Result<Option<Item<K, V>>, Error> {
        let x = self.0.first()?;
        Ok(x.map(|(k, v)| Item(k, v, PhantomData, PhantomData)))
    }

    /// Get the last item
    pub fn last(&self) -> Result<Option<Item<K, V>>, Error> {
        let x = self.0.last()?;
        Ok(x.map(|(k, v)| Item(k, v, PhantomData, PhantomData)))
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true when there are no items
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Remove all items
    pub fn clear(&self) -> Result<(), Error> {
        self.0.clear()?;
        Ok(())
    }

    /// CRC32 checksum of all keys and values
    pub fn checksum(&self) -> Result<u32, Error> {
        Ok(self.0.checksum()?)
    }
}

impl<'a, K: Key<'a>, V: Value> Default for Batch<K, V> {
    fn default() -> Self {
        Batch::new()
    }
}

impl<'a, K: Key<'a>, V: Value> Batch<K, V> {
    /// Create a new Batch instance
    pub fn new() -> Batch<K, V> {
        Batch(sled::Batch::default(), PhantomData, PhantomData)
    }

    /// Set the value associated with the specified key to the provided value
    pub fn set(&mut self, key: &K, value: &V) -> Result<(), Error> {
        let v = value.to_raw_value()?;
        self.0.insert(key.to_raw_key()?, v);
        Ok(())
    }

    /// Remove the value associated with the specified key from the database
    pub fn remove(&mut self, key: &K) -> Result<(), Error> {
        self.0.remove(key.to_raw_key()?);
        Ok(())
    }
}
