static EMPTY_BUCKET: u64 = 0u64;

struct RawTable<K, V> {
    capacity: usize,
    size: usize,
    key: *mut K,
    value: *mut V,
}
