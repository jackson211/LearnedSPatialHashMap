use core::hash::{BuildHasher, Hash};
use core::mem;
use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::raw::RawTable;

pub struct LearnedHashMap<K, V, H = DefaultHashBuilder> {
    hash_builder: H,
    table: RawTable<(K, V)>,
}

#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_insert_hash<K, S>(hash_builder: &S, val: &K) -> u64
where
    K: Hash,
    S: BuildHasher,
{
    hash_builder.hash_one(val)
}

impl<K, V, S> LearnedHashMap<K, V, S> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub const fn with_hasher(hash_builder: S) -> Self {
        Self {
            hash_builder,
            table: RawTable::new(),
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            hash_builder,
            table: RawTable::with_capacity(capacity),
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let hash = make_insert_hash::<K, S>(&self.hash_builder, &k);
        if let Some((_, item)) = self.table.get_mut(hash, equivalent_key(&k)) {
            Some(mem::replace(item, v))
        } else {
            self.table
                .insert(hash, (k, v), make_hasher::<K, _, V, S>(&self.hash_builder));
            None
        }
    }
}
