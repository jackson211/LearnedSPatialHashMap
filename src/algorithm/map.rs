use core::hash::{BuildHasher, Hash};
use core::mem;
use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::raw::RawTable;
use std::borrow::Borrow;

pub struct LearnedHashMap<K, V, S = DefaultHashBuilder> {
    hash_builder: S,
    table: RawTable<(K, V)>,
}

// #[cfg_attr(feature = "inline-more", inline)]
// pub(crate) fn make_insert_hash<K, S>(hash_builder: &S, val: &K) -> u64
// where
//     K: Hash,
//     S: BuildHasher,
// {
//     hash_builder.hash_one(val)
// }

/// copy of hashbrown::hash_map::make_hash()
#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hash<K, Q, S>(hash_builder: &S, val: &Q) -> u64
where
    K: Borrow<Q>,
    Q: Hash + ?Sized,
    S: BuildHasher,
{
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    val.hash(&mut state);
    state.finish()
}

/// copy of hashbrown::hash_map::make_hasher()
#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hasher<K, Q, V, S>(hash_builder: &S) -> impl Fn(&(Q, V)) -> u64 + '_
where
    K: Borrow<Q>,
    Q: Hash,
    S: BuildHasher,
{
    move |val| make_hash::<K, Q, S>(hash_builder, &val.0)
}

#[cfg_attr(feature = "inline-more", inline)]
fn equivalent_key<Q, K, V>(k: &Q) -> impl Fn(&(K, V)) -> bool + '_
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
{
    move |x| k.eq(x.0.borrow())
}

impl<K: Hash + Eq, V, S: BuildHasher + Default> LearnedHashMap<K, V, S> {
    pub fn new() -> LearnedHashMap<K, V, S> {
        Self {
            hash_builder: Default::default(),
            table: RawTable::new(),
        }
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> LearnedHashMap<K, V, S> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn with_hasher(hash_builder: S) -> Self {
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
        let hash = make_hash::<K, _, S>(&self.hash_builder, &k);
        if let Some((_, item)) = self.table.get_mut(hash, equivalent_key(&k)) {
            Some(mem::replace(item, v))
        } else {
            self.table
                .insert(hash, (k, v), make_hasher::<K, _, V, S>(&self.hash_builder));
            None
        }
    }

    #[inline]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.get_inner(k) {
            Some(&(_, ref v)) => Some(v),
            None => None,
        }
    }

    fn get_inner<Q: ?Sized>(&self, k: &Q) -> Option<&(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let hash = make_hash::<K, Q, S>(&self.hash_builder, k);
        self.table.get(hash, equivalent_key(k))
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::map::LearnedHashMap;
    use geo_types::Point;
    #[test]
    fn test_initialize_map() {
        let mut map: LearnedHashMap<u64, Point<f64>> = LearnedHashMap::<u64, Point<f64>>::new();
        let a: Point<f64> = (0., 1.).into();
        let b: Point<f64> = (1., 0.).into();
        let id_a: u64 = 1;
        let id_b: u64 = 2;
        map.insert(id_a, a);
        map.insert(id_b, b);
        assert_eq!(map.get(&id_a).unwrap(), &a);
        assert_eq!(map.get(&id_b).unwrap(), &b);
    }
}
