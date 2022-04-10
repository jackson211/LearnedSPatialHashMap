use crate::{algorithm::Model, primitives::Point};
use core::mem;
use num_traits::cast::{AsPrimitive, FromPrimitive};
use smallvec::SmallVec;
use std::fmt::Debug;

const INITIAL_NBUCKETS: usize = 1;

/// Default Point type for HashMap
type PType = f32;

/// Default Point Item for HashMap
type PItem = Point<PType>;

/// Default Bucket array for HashMap
type Bucket = SmallVec<[PItem; 4]>;

/// LearnedHashMap takes a model instead of an hasher for hashing indexes in the table
///
/// Default Model for the LearndedHashMap is Linear regression
/// In order to build a ordered HashMap, we need to make sure that the model is monotonic
///
#[derive(Default, Debug)]
pub struct LearnedHashMap<M: Model> {
    pub model: M,
    table: Vec<Bucket>,
    items: usize,
    sort_by_x: bool,
}

#[inline]
pub(crate) fn make_hash<M: Model>(model: &M, val: PType) -> usize {
    model.predict(val).as_()
}

#[inline]
pub(crate) fn make_hash_point<M: Model>(model: &M, p: &PItem, sort_by_x: bool) -> usize {
    if sort_by_x {
        make_hash(model, p.x)
    } else {
        make_hash(model, p.y)
    }
}

#[inline]
pub(crate) fn make_hash_tuple<M: Model>(model: &M, p: &(PType, PType), sort_by_x: bool) -> usize {
    if sort_by_x {
        make_hash(model, p.0)
    } else {
        make_hash(model, p.1)
    }
}

impl<M> LearnedHashMap<M>
where
    M: Model + Default,
{
    pub fn new() -> Self {
        Self {
            model: Default::default(),
            table: Vec::new(),
            items: 0,
            sort_by_x: true,
        }
    }

    pub fn with_model(model: M) -> Self {
        Self {
            model,
            table: Vec::new(),
            items: 0,
            sort_by_x: true,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            model: Default::default(),
            table: Vec::with_capacity(capacity),
            items: 0,
            sort_by_x: true,
        }
    }

    pub fn insert(&mut self, p: PItem) -> Option<PItem> {
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

    fn insert_with_axis(&mut self, p_value: PType, p: PItem) -> Option<PItem> {
        let mut insert_index = 0;
        if self.sort_by_x {
            // Get index from the hasher
            let hash = make_hash(&self.model, p.x);
            let bucket = &mut self.table[hash];
            for ep in bucket.iter_mut() {
                if ep == &mut p.clone() {
                    return Some(mem::replace(ep, p));
                }
                if ep.x < p_value {
                    insert_index += 1;
                }
            }
            bucket.insert(insert_index, p);
        } else {
            let hash = make_hash(&self.model, p.y);
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

    pub fn fit_batch_insert(&mut self, ps: &[PItem]) {
        let data: Vec<(PType, PType)> = if self.sort_by_x {
            ps.iter()
                .map(|&p| (p.x, PType::from_usize(p.id).unwrap()))
                .collect()
        } else {
            ps.iter()
                .map(|&p| (p.y, PType::from_usize(p.id).unwrap()))
                .collect()
        };
        self.model.fit_tuple(&data).unwrap();
        self.batch_insert(ps);
    }

    fn batch_insert(&mut self, ps: &[PItem]) {
        // Allocate table capacity before insert
        let n = ps.len();
        self.resize_with_capacity(n);
        for p in ps.iter() {
            self.insert(*p);
        }
    }

    pub fn get(&mut self, p: &(PType, PType)) -> Option<&PItem> {
        let hash = make_hash_tuple(&self.model, p, self.sort_by_x);
        if hash > self.table.capacity() {
            return None;
        }
        self.find_by_hash(hash, p)
    }

    pub fn find_by_hash(&self, hash: usize, p: &(PType, PType)) -> Option<&PItem> {
        self.table[hash]
            .iter()
            .find(|&ep| ep.x == p.0 && ep.y == p.1)
    }

    pub fn contains_key(&mut self, key: &(PType, PType)) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, p: &(PType, PType)) -> Option<PItem> {
        let hash = make_hash_tuple(&self.model, p, self.sort_by_x);
        let bucket = &mut self.table[hash];
        let i = bucket.iter().position(|ek| ek.x == p.0 && ek.y == p.1)?;
        self.items -= 1;
        Some(bucket.swap_remove(i))
    }

    pub fn range_search(
        &mut self,
        bottom_left: &(PType, PType),
        top_right: &(PType, PType),
    ) -> Option<Vec<PItem>> {
        let right_hash = make_hash_tuple(&self.model, top_right, self.sort_by_x);

        if right_hash > self.table.capacity() {
            return None;
        }
        let left_hash = make_hash_tuple(&self.model, bottom_left, self.sort_by_x);
        if left_hash > self.table.capacity() || left_hash > right_hash {
            return None;
        }
        let mut found: Vec<PItem> = Vec::new();
        for i in left_hash..=right_hash {
            let bucket = &self.table[i];
            for item in bucket.iter() {
                if item.x >= bottom_left.0
                    && item.y >= bottom_left.1
                    && item.x <= top_right.0
                    && item.y <= top_right.1
                {
                    found.push(*item);
                }
            }
        }
        if found.is_empty() {
            return None;
        }
        Some(found)
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
        new_table.extend((0..target_size).map(|_| SmallVec::new()));

        for p in self.table.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let hash = make_hash_point(&self.model, &p, self.sort_by_x);
            new_table[hash].push(p);
        }

        self.table = new_table;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::LinearModel;
    use crate::primitives::point::Point;

    #[test]
    fn insert() {
        let a: PItem = Point {
            id: 1,
            x: 0.,
            y: 1.,
        };

        let b: PItem = Point {
            id: 2,
            x: 1.,
            y: 0.,
        };

        let mut map = LearnedHashMap::<LinearModel>::new();
        map.insert(a);
        map.insert(b);

        assert_eq!(map.items, 2);
        assert_eq!(map.get(&(0., 1.)).unwrap(), &a);
        assert_eq!(map.get(&(1., 0.)).unwrap(), &b);
    }

    #[test]
    fn insert_repeated() {
        let mut map = LearnedHashMap::<LinearModel>::new();
        let a: PItem = Point {
            id: 1,
            x: 0.,
            y: 1.,
        };

        let b: PItem = Point {
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
        let data: Vec<PItem> = vec![
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
        let mut map = LearnedHashMap::<LinearModel>::new();
        map.fit_batch_insert(&data);
        dbg!(&map);

        assert_delta!(0.90909, map.model.coefficient, 0.00001);
        assert_delta!(0.45455, map.model.intercept, 0.00001);
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
        let data: Vec<PItem> = vec![
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
        let mut map = LearnedHashMap::<LinearModel>::new();
        map.fit_batch_insert(&data);
        // dbg!(&map);

        let found: Vec<PItem> = vec![
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

        let found: Vec<PItem> = vec![Point {
            id: 1,
            x: 1.,
            y: 1.,
        }];

        assert_eq!(Some(found), map.range_search(&(1., 1.), &(3., 1.)));
        assert_eq!(None, map.range_search(&(4., 2.), &(5., 3.)));
    }
}
