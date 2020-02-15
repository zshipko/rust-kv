use std::marker::PhantomData;

use crate::{Batch, Error, Key, Value};

// TODO: support transactions with multiple trees

/// Transaction error
pub type TransactionError = sled::ConflictableTransactionError<Error>;

/// Transaction
pub struct Transaction<'a, 'b, K: Key<'a>, V: Value>(
    &'b sled::TransactionalTree,
    PhantomData<K>,
    PhantomData<V>,
    PhantomData<&'a ()>,
);

impl<'a, 'b, K: Key<'a>, V: Value> Transaction<'a, 'b, K, V> {
    pub(crate) fn new(t: &'b sled::TransactionalTree) -> Self {
        Transaction(t, PhantomData, PhantomData, PhantomData)
    }

    /// Get the value associated with the specified key
    pub fn get<X: Into<K>>(&'a self, key: X) -> Result<Option<V>, TransactionError> {
        let v = self
            .0
            .get(key.into().to_raw_key().map_err(TransactionError::Abort)?)?;

        match v {
            None => Ok(None),
            Some(x) => Ok(Some(V::from_raw_value(x).map_err(TransactionError::Abort)?)),
        }
    }

    /// Set the value associated with the specified key to the provided value
    pub fn set<X: Into<K>, Y: Into<V>>(&self, key: X, value: Y) -> Result<(), TransactionError> {
        let v = value
            .into()
            .to_raw_value()
            .map_err(TransactionError::Abort)?;
        self.0
            .insert(key.into().to_raw_key().map_err(TransactionError::Abort)?, v)?;
        Ok(())
    }

    /// Remove the value associated with the specified key from the database
    pub fn remove<X: Into<K>>(&self, key: X) -> Result<(), TransactionError> {
        self.0
            .remove(key.into().to_raw_key().map_err(TransactionError::Abort)?)?;
        Ok(())
    }

    /// Apply batch update
    pub fn batch(&self, batch: Batch<K, V>) -> Result<(), TransactionError> {
        self.0.apply_batch(batch.0)?;
        Ok(())
    }
}
