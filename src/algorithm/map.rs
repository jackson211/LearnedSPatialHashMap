use crate::algorithm::{hasher::*, model::Model};
use crate::primitives::point::Point;
use core::mem;
use num_traits::{
    cast::{AsPrimitive, FromPrimitive},
    float::Float,
};
use std::borrow::Borrow;
use std::fmt::Debug;

const INITIAL_NBUCKETS: usize = 1;
#[derive(Default, Debug)]
pub struct LearnedHashMap<M, K>
where
    M: Model + Default,
    K: Float,
{
    hasher: LearnedHasher<M>,
    table: Vec<Vec<Point<K>>>,
    items: usize,
}

impl<M, K> LearnedHashMap<M, K>
where
    K: Float + AsPrimitive<u64> + FromPrimitive + Debug,
    M: Model<F = K> + Default,
{
    pub fn new() -> Self {
        Self {
            hasher: LearnedHasher::new(),
            table: Vec::new(),
            items: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            hasher: LearnedHasher::new(),
            table: Vec::with_capacity(capacity),
            items: 0,
        }
    }

    pub fn with_hasher(hasher: LearnedHasher<M>) -> Self {
        Self {
            hasher,
            table: Vec::new(),
            items: 0,
        }
    }

    pub fn insert(&mut self, p: Point<K>) -> Option<Point<K>> {
        // Resize if the table is empty or 3/4 size of the table is full
        if self.table.is_empty() || self.items > 3 * self.table.len() / 4 {
            self.resize();
        }
        // Get index from the hasher
        let hash = make_hash(&mut self.hasher, &p.value) as usize;
        let bucket = &mut self.table[hash];

        // Find where to put the key at second bucket
        let p_value = match self.hasher.sort_by_lat {
            true => p.value.0,
            false => p.value.1,
        };

        let mut insert_index = 0;
        if self.hasher.sort_by_lat {
            for mut ep in bucket.iter_mut() {
                if ep == &mut p.clone() {
                    return Some(mem::replace(&mut ep, p));
                }
                if ep.value.0 < p_value {
                    insert_index += 1;
                }
            }
        } else {
            for mut ep in bucket.iter_mut() {
                if ep == &mut p.clone() {
                    return Some(mem::replace(&mut ep, p));
                }
                if ep.value.1 < p_value {
                    insert_index += 1;
                }
            }
        }

        bucket.insert(insert_index, p);
        self.items += 1;
        None
    }

    pub fn batch_insert(&mut self, ps: &[Point<K>]) {
        // Allocate table capacity before insert
        let n = ps.len();
        self.resize_with_capacity(n);
        for p in ps.iter() {
            self.insert(*p);
        }
    }

    pub fn fit_batch_insert(&mut self, ps: &[Point<K>]) {
        let data: Vec<(K, K)>;
        if self.hasher.sort_by_lat {
            data = ps
                .iter()
                .map(|&p| (p.value.0, K::from_usize(p.id).unwrap()))
                .collect();
        } else {
            data = ps
                .iter()
                .map(|&p| (p.value.1, K::from_usize(p.id).unwrap()))
                .collect();
        }
        self.hasher.model.fit_tuple(&data).unwrap();
        self.batch_insert(ps);
    }

    pub fn get(&mut self, p: &(K, K)) -> Option<&Point<K>> {
        let hash = make_hash(&mut self.hasher, p) as usize;

        if hash > self.table.capacity() {
            return None;
        }

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
        self.resize_with_capacity(target_size);
    }

    fn resize_with_capacity(&mut self, target_size: usize) {
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
    use crate::algorithm::linear::LinearModel;
    use crate::primitives::point::Point;

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

        let mut map: LearnedHashMap<LinearModel<f64>, f64> = LearnedHashMap::new();
        map.insert(a);
        map.insert(b);

        assert_eq!(map.items, 2);
        assert_eq!(map.get(&(0., 1.)).unwrap(), &a);
        assert_eq!(map.get(&(1., 0.)).unwrap(), &b);
    }

    #[test]
    fn insert_repeated() {
        let mut map: LearnedHashMap<LinearModel<f64>, f64> = LearnedHashMap::new();
        let a: Point<f64> = Point {
            id: 1,
            value: (0., 1.),
        };

        let b: Point<f64> = Point {
            id: 2,
            value: (0., 1.),
        };

        let res = map.insert(a);
        assert_eq!(map.items, 1);
        assert_eq!(res, None);

        let res = map.insert(b);
        assert_eq!(map.items, 2);
        assert_eq!(res, None);
    }

    #[test]
    fn fit_batch_insert() {
        let data: Vec<Point<f64>> = vec![
            Point {
                id: 1,
                value: (1., 1.),
            },
            Point {
                id: 2,
                value: (3., 1.),
            },
            Point {
                id: 3,
                value: (2., 1.),
            },
            Point {
                id: 4,
                value: (3., 2.),
            },
            Point {
                id: 5,
                value: (5., 1.),
            },
        ];
        let mut map: LearnedHashMap<LinearModel<f64>, f64> = LearnedHashMap::new();
        map.fit_batch_insert(&data);
        dbg!(&map);

        assert_delta!(0.90909f64, map.hasher.model.coefficient, 0.00001);
        assert_delta!(0.45455f64, map.hasher.model.intercept, 0.00001);
        assert_eq!(
            Some(&Point {
                id: 1,
                value: (1., 1.),
            }),
            map.get(&(1., 1.))
        );
        assert_eq!(
            Some(&Point {
                id: 2,
                value: (3., 1.),
            }),
            map.get(&(3., 1.))
        );
        assert_eq!(
            Some(&Point {
                id: 5,
                value: (5., 1.),
            }),
            map.get(&(5., 1.))
        );

        assert_eq!(None, map.get(&(5., 2.)));
        assert_eq!(None, map.get(&(2., 2.)));
        assert_eq!(None, map.get(&(50., 10.)));
        assert_eq!(None, map.get(&(500., 100.)));
    }
}
