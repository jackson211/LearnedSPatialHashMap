use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::raw::RawTable;

pub struct LearnedHashMap<K, V, H = DefaultHashBuilder> {
    hash_builder: H,
    table: RawTable<(K, V)>,
}

impl<K, V> LearnedHashMap<K, V, DefaultHashBuilder> {
    pub fn new() -> Self {
        let table = RawTable::new();
        Self {
            hash_builder: DefaultHashBuilder::default(),
            table,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        let table = RawTable::with_capacity(capacity);
        Self {
            hash_builder: DefaultHashBuilder::default(),
            table,
        }
    }
}

impl<K, V, S> LearnedHashMap<K, V, S> {
    pub const fn with_hasher(hash_builder: S) -> Self {
        Self {
            hash_builder,
            table: RawTable::new(),
        }
    }
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            hash_builder,
            table: RawTable::with_capacity(capacity),
        }
    }
}
