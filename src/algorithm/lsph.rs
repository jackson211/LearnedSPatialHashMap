use std::marker::PhantomData;
static EMPTY_BUCKET: u64 = 0u64;

pub struct RawTable {
    capacity: usize,
    size: usize,
}

pub struct RawBucket<K, V> {
    hash: usize,
    pair: *const (K, V),
    idx: usize,
    _marker: PhantomData<(K, V)>,
}

impl<K, V> Copy for RawBucket<K, V> {}
impl<K, V> Clone for RawBucket<K, V> {
    fn clone(&self) -> RawBucket<K, V> {
        *self
    }
}
