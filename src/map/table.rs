use core::ops::{Deref, DerefMut};
use smallvec::SmallVec;

/// Bucket is the lower unit in the HashMap to store the points
#[derive(Debug, Clone)]
pub(crate) struct Bucket<V> {
    entry: SmallVec<[V; 6]>,
}

impl<V> Bucket<V> {
    /// Returns a default Bucket with value type.
    #[inline]
    pub fn new() -> Self {
        Self {
            entry: SmallVec::new(),
        }
    }

    /// Removes an element from the Bucket and returns it.
    /// The removed element is replaced by the last element of the Bucket.
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> V {
        self.entry.swap_remove(index)
    }
}

impl<V> Deref for Bucket<V> {
    type Target = SmallVec<[V; 6]>;
    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl<V> DerefMut for Bucket<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entry
    }
}

/// Table containing a Vec of Bucket to store the values
#[derive(Debug, Clone)]
pub(crate) struct Table<V> {
    buckets: Vec<Bucket<V>>,
}

impl<V> Table<V> {
    /// Returns a default Table with empty Vec.
    #[inline]
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
        }
    }

    /// Returns a default Table with Vec that with the given capacity.
    ///
    /// # Arguments
    /// * `capacity` - A capacity size for the Table
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buckets: Vec::with_capacity(capacity),
        }
    }

    /// Returns the capacity of the Table.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.buckets.capacity()
    }

    /// Returns the Bucket with given hash value.
    ///
    /// # Arguments
    /// * `hash` - A hash value for indexing the bucket in the table
    #[inline]
    pub fn bucket(&self, hash: u64) -> usize {
        hash as usize % self.buckets.len()
    }
}
impl<V> Table<V>
where
    V: PartialEq,
{
    /// Remove entry with given hash value and entry.
    ///
    /// # Arguments
    /// * `hash` - A hash value for indexing the bucket in the table
    /// * `entry` - Entry to remove
    #[inline]
    pub fn remove_entry(&mut self, hash: u64, entry: V) -> Option<V> {
        let index = self.bucket(hash);
        let bucket = &mut self.buckets[index];
        let i = bucket.iter().position(|ek| ek == &entry)?;
        Some(bucket.swap_remove(i))
    }
}

impl<V> Deref for Table<V> {
    type Target = Vec<Bucket<V>>;
    fn deref(&self) -> &Self::Target {
        &self.buckets
    }
}

impl<V> DerefMut for Table<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buckets
    }
}
