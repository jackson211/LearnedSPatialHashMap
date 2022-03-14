use crate::algorithm::hasher::LearnedHasherBuilder;
use core::hash::{BuildHasher, Hash};
use core::mem;
use std::borrow::Borrow;

const INITIAL_NBUCKETS: usize = 1;

#[derive(Default)]
pub struct LearnedHashMap<K, V, S = LearnedHasherBuilder> {
    hash_builder: S,
    table: Vec<Vec<(K, V)>>,
    items: usize,
}

// pub(crate) fn make_insert_hash<K, S>(hash_builder: &S, val: &K) -> u64
// where
//     K: Hash,
//     S: BuidHasher,
// {
//     hash_builder.hash_one(val)
// }

/// copy of hashbrown::hash_map::make_hash()
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

impl<K: Hash + Eq, V, S: BuildHasher + Default> LearnedHashMap<K, V, S> {
    pub fn new() -> LearnedHashMap<K, V, S> {
        Self {
            hash_builder: Default::default(),
            table: Vec::new(),
            items: 0,
        }
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> LearnedHashMap<K, V, S> {
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            hash_builder,
            table: Vec::new(),
            items: 0,
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            hash_builder,
            table: Vec::with_capacity(capacity),
            items: 0,
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if self.table.is_empty() || self.items > 3 * self.table.len() / 4 {
            self.resize();
        }
        let hash = make_hash::<K, _, S>(&self.hash_builder, &k) as usize;
        println!("inserting with hash {:?}", hash);
        // if let Some((_, item)) = self.table.get_mut(hash, equivalent_key(&k)) {
        //     Some(mem::replace(item, v))
        // } else {
        //     self.table
        //         .insert(hash, (k, v), make_hasher::<K, _, V, S>(&self.hash_builder));
        //     None
        // }

        // Find the bucket at hash location
        let bucket = &mut self.table[hash];

        // Find the key at second bucket
        for &mut (ref ek, ref mut ev) in bucket.iter_mut() {
            if ek == &k {
                return Some(mem::replace(ev, v));
            }
        }

        self.items += 1;
        bucket.push((k, v));
        None
    }

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

    fn get_inner<Q>(&self, k: &Q) -> Option<&(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = make_hash::<K, Q, S>(&self.hash_builder, k) as usize;
        // self.table.get(hash, equivalent_key(k))

        self.table[hash]
            .iter()
            .find(|&(ref ekey, _)| ekey.borrow() == k)
            .map(|i| i)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = make_hash::<K, Q, S>(&self.hash_builder, k) as usize;
        let bucket = &mut self.table[hash];
        let i = bucket
            .iter()
            .position(|&(ref ekey, _)| ekey.borrow() == k)?;
        self.items -= 1;
        Some(bucket.swap_remove(i).1)
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }
    fn resize(&mut self) {
        let target_size = match self.table.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };

        let mut new_table = Vec::with_capacity(target_size);
        new_table.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.table.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            //let mut hasher = DefaultHasher::new();
            //key.hash(&mut hasher);
            let hash = make_hash::<K, _, S>(&self.hash_builder, &key) as usize;
            new_table[hash].push((key, value));
        }

        mem::replace(&mut self.table, new_table);
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    map: &'a LearnedHashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.table.get(self.bucket) {
                Some(bucket) => {
                    match bucket.get(self.at) {
                        Some(&(ref k, ref v)) => {
                            // move along self.at and self.bucket
                            self.at += 1;
                            break Some((k, v));
                        }
                        None => {
                            self.bucket += 1;
                            self.at = 0;
                            continue;
                        }
                    }
                }
                None => break None,
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a LearnedHashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            map: self,
            bucket: 0,
            at: 0,
        }
    }
}

pub struct IntoIter<K, V> {
    map: LearnedHashMap<K, V>,
    bucket: usize,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.table.get_mut(self.bucket) {
                Some(bucket) => match bucket.pop() {
                    Some(x) => break Some(x),
                    None => {
                        self.bucket += 1;
                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<K, V> IntoIterator for LearnedHashMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            map: self,
            bucket: 0,
        }
    }
}

use std::iter::FromIterator;
impl<K, V> FromIterator<(K, V)> for LearnedHashMap<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut map = LearnedHashMap::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::map::{LearnedHashMap, LearnedHasherBuilder};
    use geo_types::{Coordinate, Line, LineString, Point, Polygon};
    #[test]
    fn initialize_map_with_points() {
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
    fn initialize_map_with_lines() {
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
    fn initialize_map_with_polygon() {
        let mut map: LearnedHashMap<u64, Polygon<f64>> = LearnedHashMap::<u64, Polygon<f64>>::new();
        let a: Polygon<f64> = Polygon::new(
            LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.)]),
            vec![],
        );
        let b: Polygon<f64> = Polygon::new(
            LineString::from(vec![(0., 0.), (1., 2.), (1., 0.), (0., 0.)]),
            vec![],
        );

        let id_a: u64 = 1;
        let id_b: u64 = 2;
        map.insert(id_a, a.clone());
        map.insert(id_b, b.clone());
        assert_eq!(map.get(&id_a).unwrap(), &a);
        assert_eq!(map.get(&id_b).unwrap(), &b);
    }

    #[test]
    fn map_with_hasher() {
        let mut map: LearnedHashMap<u64, Point<f64>, LearnedHasherBuilder> =
            LearnedHashMap::<u64, Point<f64>, LearnedHasherBuilder>::with_hasher(
                LearnedHasherBuilder::new(),
            );
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
