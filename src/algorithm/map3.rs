use crate::algorithm::{hasher::*, model::Model, Point};
use core::mem;
use num_traits::{
    cast::{AsPrimitive, FromPrimitive},
    float::Float,
};
use std::borrow::Borrow;

const INITIAL_NBUCKETS: usize = 1;
#[derive(Default)]
pub struct LearnedHashMap<M, K>
where
    M: Model,
    K: Float,
{
    hasher: LearnedHasher<M>,
    table: Vec<Vec<Point<K>>>,
    items: usize,
}

impl<M, K> LearnedHashMap<M, K>
where
    K: Float + AsPrimitive<u64> + FromPrimitive,
    M: Model<F = K>,
{
    pub fn new(model: M) -> LearnedHashMap<M, K> {
        LearnedHashMap {
            hasher: LearnedHasher::<M>::new(model),
            table: Vec::new(),
            items: 0,
        }
    }

    pub fn with_capacity(model: M, capacity: usize) -> Self {
        Self {
            hasher: LearnedHasher::new(model),
            table: Vec::with_capacity(capacity),
            items: 0,
        }
    }

    pub fn insert(&mut self, p: Point<K>) -> Option<Point<K>> {
        // Resize if the table is empty or 3/4 size of the table is full
        if self.table.is_empty() || self.items > 3 * self.table.len() / 4 {
            self.resize();
        }
        let hash = make_hash(&mut self.hasher, &p.value) as usize;
        dbg!("inserting with hash {:?}", hash);

        // Find the bucket at hash location
        let bucket = &mut self.table[hash];

        // Find the key at second bucket
        for mut ep in bucket.iter_mut() {
            if ep == &mut p.clone() {
                return Some(mem::replace(&mut ep, p));
            }
        }

        bucket.push(p);
        self.items += 1;
        None
    }

    pub fn batch_insert(&mut self, ps: &[Point<K>]) {
        for p in ps.iter() {
            self.insert(*p);
        }
    }

    pub fn fit_batch_insert(&mut self, ps: &[Point<K>]) {
        let data: Vec<(K, K)> = ps.iter().map(|&p| (p.value.0, p.value.1)).collect();
        self.hasher.model.fit_tuple(&data);
        self.batch_insert(ps);
    }

    pub fn get(&mut self, p: &(K, K)) -> Option<&Point<K>> {
        let hash = make_hash(&mut self.hasher, p) as usize;
        // self.table.get(hash, equivalent_key(k))

        self.table[hash]
            .iter()
            .find(|&ep| ep.value.borrow() == p)
            .map(|i| i)
    }

    pub fn contains_key(&mut self, key: &(K, K)) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, p: &(K, K)) -> Option<Point<K>> {
        let hash = make_hash(&mut self.hasher, p) as usize;
        let bucket = &mut self.table[hash];
        let i = bucket.iter().position(|&ref ek| ek.value.borrow() == p)?;
        self.items -= 1;
        Some(bucket.swap_remove(i))
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

        for p in self.table.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            //let mut hasher = DefaultHasher::new();
            //key.hash(&mut hasher);
            let hash = make_hash(&mut self.hasher, &p.value) as usize;
            new_table[hash].push(p);
        }

        self.table = new_table;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::{linear::LinearModel, Point};

    #[test]
    fn insert() {
        let a: Point<f64> = Point {
            id: 1,
            value: (0., 1.),
        };

        let b: Point<f64> = Point {
            id: 2,
            value: (1., 0.),
        };
        let model: LinearModel<f64> = LinearModel::new();
        let mut map: LearnedHashMap<LinearModel<f64>, f64> = LearnedHashMap::new(model);
        map.insert(a);
        map.insert(b);
        assert_eq!(map.get(&(0., 1.)).unwrap(), &a);
        assert_eq!(map.get(&(1., 0.)).unwrap(), &b);
    }

    #[test]
    fn insert_repeated() {
        let model: LinearModel<f64> = LinearModel::new();
        let mut map: LearnedHashMap<LinearModel<f64>, f64> = LearnedHashMap::new(model);
        let a: Point<f64> = Point {
            id: 1,
            value: (0., 1.),
        };

        let b: Point<f64> = Point {
            id: 2,
            value: (0., 1.),
        };

        let res = map.insert(a);
        assert_eq!(res, None);
        let res = map.insert(b);
        assert_eq!(res, None);
    }
}
