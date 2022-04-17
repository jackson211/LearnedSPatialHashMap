use core::ops::{Deref, DerefMut};
use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub(crate) struct Bucket<V> {
    entry: SmallVec<[V; 6]>,
}

impl<V> Bucket<V> {
    pub fn new() -> Self {
        Self {
            entry: SmallVec::new(),
        }
    }

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

#[derive(Debug, Clone)]
pub(crate) struct Table<V> {
    buckets: Vec<Bucket<V>>,
}

impl<V> Table<V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buckets: Vec::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.buckets.capacity()
    }

    pub fn bucket(&self, hash: u64) -> usize {
        hash as usize % self.buckets.len()
    }
}
impl<V> Table<V>
where
    V: PartialEq,
{
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
