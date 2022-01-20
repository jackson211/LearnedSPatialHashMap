use core::hash::{BuildHasher, Hash};
use core::mem;
use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::raw::RawTable;
use std::borrow::Borrow;

pub struct LearnedHasher {
    state: u64,
}

impl std::hash::Hasher for LearnedHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.rotate_left(8) ^ u64::from(byte);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

pub struct BuildLearnedHasher;

impl std::hash::BuildHasher for BuildLearnedHasher {
    type Hasher = LearnedHasher;
    fn build_hasher(&self) -> LearnedHasher {
        LearnedHasher { state: 0 }
    }
}

#[derive(Default)]
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
    use crate::algorithm::map::{BuildLearnedHasher, LearnedHashMap};
    use geo_types::{Coordinate, Line, LineString, Point, Polygon};
    #[test]
    fn test_initialize_map_with_points() {
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

    #[test]
    fn test_initialize_map_with_lines() {
        let mut map: LearnedHashMap<u64, Line<f64>> = LearnedHashMap::<u64, Line<f64>>::new();
        let a: Line<f64> = Line::new(Coordinate { x: 0., y: 1. }, Coordinate { x: 1., y: 2. });
        let b: Line<f64> = Line::new(Coordinate { x: 0., y: 0. }, Coordinate { x: 2., y: 1. });
        let id_a: u64 = 1;
        let id_b: u64 = 2;
        map.insert(id_a, a);
        map.insert(id_b, b);
        assert_eq!(map.get(&id_a).unwrap(), &a);
        assert_eq!(map.get(&id_b).unwrap(), &b);
    }

    #[test]
    fn test_initialize_map_with_polygon() {
        let mut map: LearnedHashMap<u64, Polygon<f64>> = LearnedHashMap::<u64, Polygon<f64>>::new();
        let a: Polygon<f64> = Polygon::new(
            LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.)]),
            vec![],
        );
        let b: Polygon<f64> = Polygon::new(
            LineString::from(vec![(0., 0.), (1., 2.), (1., 0.), (0., 0.)]),
            vec![],
        );
        // Polygon doesn't impl Copy trait
        let a_clone = a.clone();
        let b_clone = b.clone();

        let id_a: u64 = 1;
        let id_b: u64 = 2;
        map.insert(id_a, a);
        map.insert(id_b, b);
        assert_eq!(map.get(&id_a).unwrap(), &a_clone);
        assert_eq!(map.get(&id_b).unwrap(), &b_clone);
    }

    #[test]
    fn test_map_with_hasher() {
        let mut map: LearnedHashMap<u64, Point<f64>, BuildLearnedHasher> =
            LearnedHashMap::<u64, Point<f64>, BuildLearnedHasher>::with_hasher(BuildLearnedHasher);
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
