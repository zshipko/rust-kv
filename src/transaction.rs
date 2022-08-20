use std::marker::PhantomData;

use crate::{Batch, Error, Key, Value};

/// Transaction error
pub type TransactionError<E> = sled::transaction::ConflictableTransactionError<E>;

/// Transaction
#[derive(Clone)]
pub struct Transaction<'a, 'b, K: Key<'a>, V: Value>(
    &'b sled::transaction::TransactionalTree,
    PhantomData<K>,
    PhantomData<V>,
    PhantomData<&'a ()>,
);

impl<'a, 'b, K: Key<'a>, V: Value> Transaction<'a, 'b, K, V> {
    pub(crate) fn new(t: &'b sled::transaction::TransactionalTree) -> Self {
        Transaction(t, PhantomData, PhantomData, PhantomData)
    }

    /// Get the value associated with the specified key
    pub fn get(&self, key: &K) -> Result<Option<V>, TransactionError<Error>> {
        let v = self
            .0
            .get(key.to_raw_key().map_err(TransactionError::Abort)?)?;

        match v {
            None => Ok(None),
            Some(x) => Ok(Some(V::from_raw_value(x).map_err(TransactionError::Abort)?)),
        }
    }

    /// Set the value associated with the specified key to the provided value
    pub fn set(&self, key: &K, value: &V) -> Result<Option<V>, TransactionError<Error>> {
        let v = value.to_raw_value().map_err(TransactionError::Abort)?;
        Ok(self
            .0
            .insert(key.to_raw_key().map_err(TransactionError::Abort)?, v)?
            .map(|x| V::from_raw_value(x).map_err(TransactionError::Abort))
            // https://users.rust-lang.org/t/convenience-method-for-flipping-option-result-to-result-option/13695/7
            .map_or(Ok(None), |v| v.map(Some))?)
    }

    /// Remove the value associated with the specified key from the database
    pub fn remove(&self, key: &K) -> Result<Option<V>, TransactionError<Error>> {
        Ok(self
            .0
            .remove(key.to_raw_key().map_err(TransactionError::Abort)?)?
            .map(|x| V::from_raw_value(x).map_err(TransactionError::Abort))
            // https://users.rust-lang.org/t/convenience-method-for-flipping-option-result-to-result-option/13695/7
            .map_or(Ok(None), |v| v.map(Some))?)
    }

    /// Apply batch update
    pub fn batch(&self, batch: &Batch<K, V>) -> Result<(), TransactionError<Error>> {
        self.0.apply_batch(&batch.0)?;
        Ok(())
    }

    /// Generate a monotonic ID. Not guaranteed to be contiguous or idempotent, can produce different values in the same transaction in case of conflicts
    pub fn generate_id(&self) -> Result<u64, TransactionError<Error>> {
        Ok(self.0.generate_id()?)
    }
}
