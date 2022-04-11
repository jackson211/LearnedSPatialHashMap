use crate::{algorithm::Model, hasher::*, primitives::Point};
use core::mem;
use num_traits::{
    cast::{AsPrimitive, FromPrimitive},
    float::Float,
};
use smallvec::SmallVec;
use std::fmt::Debug;

const INITIAL_NBUCKETS: usize = 1;

/// Default Point Item for HashMap
type PItem<T> = Point<T>;

/// Default Bucket array for HashMap
type Bucket<T> = SmallVec<[PItem<T>; 6]>;

/// LearnedHashMap takes a model instead of an hasher for hashing indexes in the table
///
/// Default Model for the LearndedHashMap is Linear regression
/// In order to build a ordered HashMap, we need to make sure that the model is monotonic
///
#[derive(Debug)]
pub struct LearnedHashMap<M, F>
where
    F: Float,
    M: Model<F = F> + Default,
{
    hasher: LearnedHasher<M, F>,
    table: Vec<Bucket<F>>,
    items: usize,
    sort_by_x: bool,
}

impl<M, F> Default for LearnedHashMap<M, F>
where
    F: Float + Default + AsPrimitive<u64> + FromPrimitive,
    M: Model<F = F> + Default,
{
    fn default() -> Self {
        Self {
            hasher: LearnedHasher::<M, F>::new(),
            table: Vec::new(),
            items: 0,
            sort_by_x: true,
        }
    }
}

impl<M, F> LearnedHashMap<M, F>
where
    F: Float + Default + AsPrimitive<u64> + FromPrimitive,
    M: Model<F = F> + Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_hasher(hasher: LearnedHasher<M, F>) -> Self {
        Self {
            hasher,
            table: Vec::new(),
            items: 0,
            sort_by_x: true,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            hasher: Default::default(),
            table: Vec::with_capacity(capacity),
            items: 0,
            sort_by_x: true,
        }
    }

    pub fn insert(&mut self, p: PItem<F>) -> Option<PItem<F>> {
        // Resize if the table is empty or 3/4 size of the table is full
        if self.table.is_empty() || self.items > 3 * self.table.len() / 4 {
            self.resize();
        }

        // Find where to put the key at second bucket
        let p_value = match self.sort_by_x {
            true => p.x,
            false => p.y,
        };

        self.insert_with_axis(p_value, p)
    }

    #[inline]
    fn insert_with_axis(&mut self, p_value: F, p: PItem<F>) -> Option<PItem<F>> {
        let mut insert_index = 0;
        if self.sort_by_x {
            // Get index from the hasher
            let hash = make_hash::<M, F>(&mut self.hasher, &p.x) as usize;
            let bucket = &mut self.table[hash];
            for ep in bucket.iter_mut() {
                if ep == &mut p.clone() {
                    return Some(mem::replace(ep, p));
                }
                if ep.x < p.x {
                    insert_index += 1;
                }
            }
            bucket.insert(insert_index, p);
        } else {
            let hash = make_hash::<M, F>(&mut self.hasher, &p.y) as usize;
            let bucket = &mut self.table[hash];
            for ep in bucket.iter_mut() {
                if ep == &mut p.clone() {
                    return Some(mem::replace(ep, p));
                }
                if ep.y < p_value {
                    insert_index += 1;
                }
            }
            bucket.insert(insert_index, p);
        }

        self.items += 1;
        None
    }

    pub fn batch_insert(&mut self, ps: &[PItem<F>]) {
        // Allocate table capacity before insert
        let n = ps.len();
        self.resize_with_capacity(n * 2);
        for p in ps.iter() {
            self.insert(*p);
        }
    }

    pub fn fit_batch_insert(&mut self, ps: &[PItem<F>]) {
        let data: Vec<(F, F)> = if self.hasher.sort_by_lat {
            ps.iter()
                .map(|&p| (p.x, F::from_usize(p.id).unwrap()))
                .collect()
        } else {
            ps.iter()
                .map(|&p| (p.y, F::from_usize(p.id).unwrap()))
                .collect()
        };
        self.hasher.model.fit_tuple(&data).unwrap();
        self.batch_insert(ps);
    }

    pub fn get(&mut self, p: &(F, F)) -> Option<&PItem<F>> {
        let hash = make_hash(&mut self.hasher, &p.0) as usize;
        if hash > self.table.capacity() {
            return None;
        }
        self.find_by_hash(hash, p)
    }

    pub fn find_by_hash(&self, hash: usize, p: &(F, F)) -> Option<&PItem<F>> {
        self.table[hash]
            .iter()
            .find(|&ep| ep.x == p.0 && ep.y == p.1)
    }

    #[inline]
    pub fn contains_key(&mut self, key: &(F, F)) -> bool {
        self.get(key).is_some()
    }

    #[inline]
    pub fn remove(&mut self, p: &(F, F)) -> Option<PItem<F>> {
        let hash = make_hash(&mut self.hasher, &p.0) as usize;
        let bucket = &mut self.table[hash];
        let i = bucket.iter().position(|ek| ek.x == p.0 && ek.y == p.1)?;
        self.items -= 1;
        Some(bucket.swap_remove(i))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items
    }

    #[inline]
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

    #[inline(never)]
    fn resize_with_capacity(&mut self, target_size: usize) {
        let mut new_table = Vec::with_capacity(target_size);
        new_table.extend((0..target_size).map(|_| SmallVec::new()));

        for p in self.table.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            //let mut hasher = DefaultHasher::new();
            //key.hash(&mut hasher);
            let hash = make_hash(&mut self.hasher, &p.x) as usize;
            new_table[hash].push(p);
        }

        self.table = new_table;
    }

    pub fn range_search(
        &mut self,
        bottom_left: &(F, F),
        top_right: &(F, F),
    ) -> Option<Vec<PItem<F>>> {
        let right_hash = make_hash(&mut self.hasher, &top_right.0) as usize;
        if right_hash > self.table.capacity() {
            return None;
        }
        let left_hash = make_hash(&mut self.hasher, &bottom_left.0) as usize;
        if left_hash > self.table.capacity() || left_hash > right_hash {
            return None;
        }
        let mut result: Vec<PItem<F>> = Vec::new();
        for i in left_hash..=right_hash {
            let bucket = &self.table[i];
            for item in bucket.iter() {
                if item.x >= bottom_left.0
                    && item.y >= bottom_left.1
                    && item.x <= top_right.0
                    && item.y <= top_right.1
                {
                    result.push(*item);
                }
            }
        }
        if result.is_empty() {
            return None;
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::LinearModel;
    use crate::primitives::point::Point;

    #[test]
    fn insert() {
        let a: PItem<f64> = Point {
            id: 1,
            x: 0.,
            y: 1.,
        };

        let b: PItem<f64> = Point {
            id: 2,
            x: 1.,
            y: 0.,
        };

        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        map.insert(a);
        map.insert(b);

        assert_eq!(map.items, 2);
        assert_eq!(map.get(&(0., 1.)).unwrap(), &a);
        assert_eq!(map.get(&(1., 0.)).unwrap(), &b);
    }

    #[test]
    fn insert_repeated() {
        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        let a: PItem<f64> = Point {
            id: 1,
            x: 0.,
            y: 1.,
        };

        let b: PItem<f64> = Point {
            id: 2,
            x: 0.,
            y: 1.,
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
        let data: Vec<PItem<f64>> = vec![
            Point {
                id: 1,
                x: 1.,
                y: 1.,
            },
            Point {
                id: 2,
                x: 3.,
                y: 1.,
            },
            Point {
                id: 3,
                x: 2.,
                y: 1.,
            },
            Point {
                id: 4,
                x: 3.,
                y: 2.,
            },
            Point {
                id: 5,
                x: 5.,
                y: 1.,
            },
        ];
        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        map.fit_batch_insert(&data);
        dbg!(&map);

        assert_delta!(0.90909, map.hasher.model.coefficient, 0.00001);
        assert_delta!(0.45455, map.hasher.model.intercept, 0.00001);
        assert_eq!(
            Some(&Point {
                id: 1,
                x: 1.,
                y: 1.,
            }),
            map.get(&(1., 1.))
        );
        assert_eq!(
            Some(&Point {
                id: 2,
                x: 3.,
                y: 1.,
            }),
            map.get(&(3., 1.))
        );
        assert_eq!(
            Some(&Point {
                id: 5,
                x: 5.,
                y: 1.,
            }),
            map.get(&(5., 1.))
        );

        assert_eq!(None, map.get(&(5., 2.)));
        assert_eq!(None, map.get(&(2., 2.)));
        assert_eq!(None, map.get(&(50., 10.)));
        assert_eq!(None, map.get(&(500., 100.)));
    }

    #[test]
    fn range_search() {
        let data: Vec<PItem<f64>> = vec![
            Point {
                id: 1,
                x: 1.,
                y: 1.,
            },
            Point {
                id: 2,
                x: 2.,
                y: 2.,
            },
            Point {
                id: 3,
                x: 3.,
                y: 3.,
            },
            Point {
                id: 4,
                x: 4.,
                y: 4.,
            },
            Point {
                id: 5,
                x: 5.,
                y: 5.,
            },
        ];
        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        map.fit_batch_insert(&data);
        // dbg!(&map);

        let found: Vec<PItem<f64>> = vec![
            Point {
                id: 1,
                x: 1.,
                y: 1.,
            },
            Point {
                id: 2,
                x: 2.,
                y: 2.,
            },
            Point {
                id: 3,
                x: 3.,
                y: 3.,
            },
        ];

        assert_eq!(Some(found), map.range_search(&(1., 1.), &(3.5, 3.)));

        let found: Vec<PItem<f64>> = vec![Point {
            id: 1,
            x: 1.,
            y: 1.,
        }];

        assert_eq!(Some(found), map.range_search(&(1., 1.), &(3., 1.)));
        assert_eq!(None, map.range_search(&(4., 2.), &(5., 3.)));
    }
}
