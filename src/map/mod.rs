mod nn;
mod table;

use crate::{
    error::*,
    geometry::{distance::*, Point},
    hasher::*,
    map::{nn::*, table::*},
    models::Model,
};
use core::iter::Sum;
use core::mem;
use num_traits::{
    cast::{AsPrimitive, FromPrimitive},
    float::Float,
};
use std::collections::BinaryHeap;
use std::fmt::Debug;

/// Initial bucket size is set to 1
const INITIAL_NBUCKETS: usize = 1;

/// LearnedHashMap takes a model instead of an hasher for hashing indexes in the table
///
/// Default Model for the LearndedHashMap is Linear regression
/// In order to build a ordered HashMap, we need to make sure that the model is monotonic
#[derive(Debug, Clone)]
pub struct LearnedHashMap<M, F> {
    hasher: LearnedHasher<M>,
    table: Table<Point<F>>,
    items: usize,
}

impl<M, F> Default for LearnedHashMap<M, F>
where
    F: Float,
    M: Model<F = F> + Default,
{
    fn default() -> Self {
        Self {
            hasher: LearnedHasher::<M>::new(),
            table: Table::new(),
            items: 0,
        }
    }
}

impl<M, F> LearnedHashMap<M, F>
where
    F: Float + Default + AsPrimitive<u64> + FromPrimitive + Debug,
    M: Model<F = F> + Default + Clone,
{
    /// Returns a default LearnedHashMap with Model and Float type
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel};
    /// let map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a default LearnedHashMap with Model and Float type
    ///
    /// # Arguments
    /// * `hasher` - A LearnedHasher with model
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, LearnedHasher};
    /// let map = LearnedHashMap::<LinearModel<f64>, f64>::with_hasher(LearnedHasher::new());
    /// ```
    pub fn with_hasher(hasher: LearnedHasher<M>) -> Self {
        Self {
            hasher,
            table: Table::new(),
            items: 0,
        }
    }

    /// Returns a default LearnedHashMap with Model and Float type
    ///
    /// # Arguments
    /// * `capacity` - A predefined capacity size for the LearnedHashMap
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, LearnedHasher};
    /// let map = LearnedHashMap::<LinearModel<f64>, f64>::with_capacity(10usize);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            hasher: Default::default(),
            table: Table::with_capacity(capacity),
            items: 0,
        }
    }

    pub fn get(&mut self, p: &(F, F)) -> Option<&Point<F>> {
        let hash = make_hash_point(&mut self.hasher, p) as usize;
        if hash > self.table.capacity() {
            return None;
        }
        self.find_by_hash(hash, p)
    }

    pub fn find_by_hash(&self, hash: usize, p: &(F, F)) -> Option<&Point<F>> {
        self.table[hash]
            .iter()
            .find(|&ep| ep.x == p.0 && ep.y == p.1)
    }

    #[inline]
    pub fn contains_key(&mut self, key: &(F, F)) -> bool {
        self.get(key).is_some()
    }

    #[inline]
    pub fn remove(&mut self, p: &Point<F>) -> Option<Point<F>> {
        let hash = make_hash_point(&mut self.hasher, &(p.x, p.y));
        self.items -= 1;
        self.table.remove_entry(hash, *p)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.table.len()
    }

    #[inline]
    pub fn items(&self) -> usize {
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
        let mut new_table = Table::with_capacity(target_size);
        new_table.extend((0..target_size).map(|_| Bucket::new()));

        for p in self.table.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let hash = make_hash_point(&mut self.hasher, &(p.x, p.y)) as usize;
            new_table[hash].push(p);
        }

        self.table = new_table;
    }

    /// Range search finds all points for a given 2d range
    /// Returns all the points within the given range
    /// ```text
    ///      |                    top right
    ///      |        .-----------*
    ///      |        | .   .     |
    ///      |        |  .  .  .  |
    ///      |        |       .   |
    ///   bottom left *-----------.
    ///      |
    ///      |        |           |
    ///      |________v___________v________
    ///              left       right
    ///              hash       hash
    /// ```
    /// # Arguments
    ///
    /// * `bottom_left` - A tuple containing a pair of points that represent the bottom left of the
    /// range.
    ///
    /// * `top_right` - A tuple containing a pair of points that represent the top right of the
    /// range.
    pub fn range_search(
        &mut self,
        bottom_left: &(F, F),
        top_right: &(F, F),
    ) -> Option<Vec<Point<F>>> {
        let mut right_hash = make_hash_point(&mut self.hasher, top_right) as usize;
        if right_hash > self.table.capacity() {
            right_hash = self.table.capacity() as usize - 1;
        }
        let left_hash = make_hash_point(&mut self.hasher, bottom_left) as usize;
        if left_hash > self.table.capacity() || left_hash > right_hash {
            return None;
        }
        let mut result: Vec<Point<F>> = Vec::new();
        for i in left_hash..=right_hash {
            let bucket = &self.table[i];
            for item in bucket.iter() {
                if item.x >= bottom_left.0
                    && item.x <= top_right.0
                    && item.y >= bottom_left.1
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

    pub fn radius_range(&mut self, query_point: &(F, F), radius: F) -> Option<Vec<Point<F>>> {
        self.range_search(
            &(query_point.0 - radius, query_point.1 - radius),
            &(query_point.0 + radius, query_point.1 + radius),
        )
    }

    fn local_min_heap(
        &self,
        heap: &mut BinaryHeap<NearestNeighborState<F>>,
        local_hash: u64,
        query_point: &(F, F),
        min_d: &mut F,
        nearest_neighbor: &mut Point<F>,
    ) {
        let bucket = &self.table[local_hash as usize];
        if !bucket.is_empty() {
            for p in bucket.iter() {
                let d = Euclidean::distance(query_point, &(p.x, p.y));
                heap.push(NearestNeighborState {
                    distance: d,
                    point: *p,
                });
            }
        }
        match heap.pop() {
            Some(v) => {
                let local_min_d = v.distance;
                // Update the nearest neighbour and minimum distance
                if &local_min_d < min_d {
                    *nearest_neighbor = v.point;
                    *min_d = local_min_d;
                }
            }
            None => (),
        }
    }

    fn horizontal_distance(&mut self, query_point: &(F, F), hash: u64) -> F {
        let x = unhash(&mut self.hasher, hash);
        match self.hasher.sort_by_x() {
            true => Euclidean::distance(&(query_point.0, F::zero()), &(x, F::zero())),
            false => Euclidean::distance(&(query_point.1, F::zero()), &(x, F::zero())),
        }
    }

    /// Nearest neighbor search for the cloest point for given query point
    /// Returns the closest point
    ///```text
    ///      |
    ///      |            .
    ///      |         .  |
    ///      |         |. |  *  . <- nearest neighbor
    ///      |         || |  | .|
    ///      |  expand <--------> expand
    ///      |  left         |     right
    ///      |               |
    ///      |_______________v_____________
    ///                    query
    ///                    point
    ///```
    /// # Arguments
    ///
    /// * `query_point` - A tuple containing a pair of points for querying
    ///
    pub fn nearest_neighbor(&mut self, query_point: &(F, F)) -> Option<Point<F>> {
        let mut hash = make_hash_point(&mut self.hasher, query_point);
        let max_capacity = self.table.capacity() as u64;

        // if hash out of max bound, still search right most bucket
        if hash > max_capacity {
            hash = max_capacity - 1;
        }

        let mut heap = BinaryHeap::new();
        let mut min_d = F::max_value();
        let mut nearest_neighbor = Point::default();

        // Searching at current hash index
        self.local_min_heap(
            &mut heap,
            hash,
            query_point,
            &mut min_d,
            &mut nearest_neighbor,
        );

        // Measure left horizontal distance from current bucket to left hash bucket
        // left hash must >= 0
        let mut left_hash = hash.saturating_sub(1);
        // Unhash the left_hash, then calculate the vertical distance between
        // left hash point and query point
        let mut left_hash_d = self.horizontal_distance(query_point, left_hash);

        // Iterate over left
        while left_hash_d < min_d {
            self.local_min_heap(
                &mut heap,
                left_hash,
                query_point,
                &mut min_d,
                &mut nearest_neighbor,
            );

            // break before update
            if left_hash == 0 {
                break;
            }

            // Update next right side bucket distance
            left_hash = left_hash.saturating_sub(1);
            left_hash_d = self.horizontal_distance(query_point, left_hash);
        }

        // Measure right vertical distance from current bucket to right hash bucket
        let mut right_hash = hash + 1;
        // Unhash the right_hash, then calculate the vertical distance between
        // right hash point and query point
        let mut right_hash_d = self.horizontal_distance(query_point, right_hash);

        // Iterate over right
        while right_hash_d < min_d {
            self.local_min_heap(
                &mut heap,
                right_hash,
                query_point,
                &mut min_d,
                &mut nearest_neighbor,
            );

            // Move to next right bucket
            right_hash += 1;

            // break after update
            if right_hash == self.table.capacity() as u64 {
                break;
            }
            // Update next right side bucket distance
            right_hash_d = self.horizontal_distance(query_point, right_hash);
        }

        Some(nearest_neighbor)
    }
}

impl<M, F> LearnedHashMap<M, F>
where
    F: Float + AsPrimitive<u64> + FromPrimitive + Default + Debug + Sum,
    M: Model<F = F> + Default + Clone,
{
    /// Returns a default LearnedHashMap with Model and Float type
    ///
    /// # Arguments
    /// * `data` - A Vec<[F; 2]> of 2d points for the map
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel};
    /// let data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
    /// let map = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&data);
    /// ```
    pub fn with_data(data: &Vec<[F; 2]>) -> Result<(Self, Vec<Point<F>>), Error> {
        use crate::helper::convert_to_points;
        let mut map = LearnedHashMap::with_capacity(data.len());
        let mut ps = convert_to_points(data).unwrap();
        match map.batch_insert(&mut ps) {
            Ok(()) => Ok((map, ps)),
            Err(err) => Err(err),
        }
    }

    pub fn insert(&mut self, p: Point<F>) -> Option<Point<F>> {
        // Resize if the table is empty or 3/4 size of the table is full
        if self.table.is_empty() || self.items() > 3 * self.table.len() / 4 {
            self.resize();
        }

        // Find where to put the key at second bucket
        let p_value = match self.hasher.sort_by_x() {
            true => p.x,
            false => p.y,
        };

        self.insert_with_axis(p_value, p)
    }

    #[inline]
    fn insert_with_axis(&mut self, p_value: F, p: Point<F>) -> Option<Point<F>> {
        let mut insert_index = 0;
        let hash = make_hash_point::<M, F>(&mut self.hasher, &(p.x, p.y)) as usize;
        let bucket = &mut self.table[hash];
        if self.hasher.sort_by_x() {
            // Get index from the hasher
            for ep in bucket.iter_mut() {
                if ep == &mut p.clone() {
                    return Some(mem::replace(ep, p));
                }
                if ep.x < p.x {
                    insert_index += 1;
                }
            }
        } else {
            for ep in bucket.iter_mut() {
                if ep == &mut p.clone() {
                    return Some(mem::replace(ep, p));
                }
                if ep.y < p_value {
                    insert_index += 1;
                }
            }
        }
        bucket.insert(insert_index, p);
        self.items += 1;
        None
    }

    pub fn model_fit(&mut self, data: &[(F, F)]) -> Result<(), Error> {
        self.hasher.model.fit_tuple(data)
    }

    fn _batch_insert_inner(&mut self, ps: &[Point<F>]) {
        // Allocate table capacity before insert
        let n = ps.len();
        self.resize_with_capacity(n);
        for p in ps.iter() {
            self.insert(*p);
        }
    }

    pub fn batch_insert(&mut self, ps: &mut [Point<F>]) -> Result<(), Error> {
        // Select suitable axis for training
        use crate::geometry::Axis;
        use crate::models::Trainer;

        // Loading data into trainer
        if let Ok(trainer) = Trainer::with_points(ps) {
            trainer.train(&mut self.hasher.model).unwrap();
            let axis = trainer.axis();
            match axis {
                Axis::X => self.hasher.set_sort_by_x(true),
                _ => self.hasher.set_sort_by_x(false),
            };

            // Pick out values from one axis
            let data: Vec<(F, F)> = if self.hasher.sort_by_x() {
                ps.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
                ps.iter()
                    .enumerate()
                    .map(|(id, p)| (p.x, F::from_usize(id).unwrap()))
                    .collect()
            } else {
                ps.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
                ps.iter()
                    .enumerate()
                    .map(|(id, p)| (p.y, F::from_usize(id).unwrap()))
                    .collect()
            };
            // Fit the data into model
            self.model_fit(&data).unwrap();
            // Batch insert into the map
            self._batch_insert_inner(ps);
        }
        Ok(())
    }
}

pub struct Iter<'a, M, F>
where
    F: Float,
    M: Model<F = F> + Default + Clone,
{
    map: &'a LearnedHashMap<M, F>,
    bucket: usize,
    at: usize,
}

impl<'a, M, F> Iterator for Iter<'a, M, F>
where
    F: Float,
    M: Model<F = F> + Default + Clone,
{
    type Item = &'a Point<F>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.table.get(self.bucket) {
                Some(bucket) => {
                    match bucket.get(self.at) {
                        Some(p) => {
                            // move along self.at and self.bucket
                            self.at += 1;
                            break Some(p);
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

impl<'a, M, F> IntoIterator for &'a LearnedHashMap<M, F>
where
    F: Float,
    M: Model<F = F> + Default + Clone,
{
    type Item = &'a Point<F>;
    type IntoIter = Iter<'a, M, F>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            map: self,
            bucket: 0,
            at: 0,
        }
    }
}

pub struct IntoIter<M, F>
where
    F: Float,
    M: Model<F = F> + Default + Clone,
{
    map: LearnedHashMap<M, F>,
    bucket: usize,
}

impl<M, F> Iterator for IntoIter<M, F>
where
    F: Float,
    M: Model<F = F> + Default + Clone,
{
    type Item = Point<F>;
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

impl<M, F> IntoIterator for LearnedHashMap<M, F>
where
    F: Float,
    M: Model<F = F> + Default + Clone,
{
    type Item = Point<F>;
    type IntoIter = IntoIter<M, F>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            map: self,
            bucket: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::models::LinearModel;
    use crate::test_utilities::*;

    #[test]
    fn insert() {
        let a: Point<f64> = Point {
            id: 1,
            x: 0.,
            y: 1.,
        };

        let b: Point<f64> = Point {
            id: 2,
            x: 1.,
            y: 0.,
        };

        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        map.insert(a);
        map.insert(b);

        assert_eq!(map.items(), 2);
        assert_eq!(map.get(&(0., 1.)).unwrap(), &a);
        assert_eq!(map.get(&(1., 0.)).unwrap(), &b);
    }

    #[test]
    fn insert_repeated() {
        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        let a: Point<f64> = Point {
            id: 1,
            x: 0.,
            y: 1.,
        };

        let b: Point<f64> = Point {
            id: 2,
            x: 0.,
            y: 1.,
        };

        let res = map.insert(a);
        assert_eq!(map.items(), 1);
        assert_eq!(res, None);

        let res = map.insert(b);
        assert_eq!(map.items(), 2);
        assert_eq!(res, None);
    }

    #[test]
    fn fit_batch_insert() {
        let mut data: Vec<Point<f64>> = vec![
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
        map.batch_insert(&mut data).unwrap();
        dbg!(&map);

        assert_delta!(1.02272, map.hasher.model.coefficient, 0.00001);
        assert_delta!(-0.86363, map.hasher.model.intercept, 0.00001);
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
        let mut data: Vec<Point<f64>> = vec![
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
        map.batch_insert(&mut data).unwrap();
        // dbg!(&map);

        let found: Vec<Point<f64>> = vec![
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

        let found: Vec<Point<f64>> = vec![Point {
            id: 1,
            x: 1.,
            y: 1.,
        }];

        assert_eq!(Some(found), map.range_search(&(1., 1.), &(3., 1.)));
        assert_eq!(None, map.range_search(&(4., 2.), &(5., 3.)));
    }

    #[test]
    fn test_nearest_neighbor() {
        let points = create_random_point_type_points(1000, SEED_1);
        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        map.batch_insert(&mut points.clone()).unwrap();

        let sample_points = create_random_point_type_points(100, SEED_2);
        let mut i = 0;
        for sample_point in &sample_points {
            let mut nearest = None;
            let mut closest_dist = ::core::f64::INFINITY;
            for point in &points {
                let new_dist = Euclidean::distance_point(&point, &sample_point);
                if new_dist < closest_dist {
                    closest_dist = new_dist;
                    nearest = Some(point);
                }
            }
            let map_nearest = map
                .nearest_neighbor(&(sample_point.x, sample_point.y))
                .unwrap();
            assert_eq!(nearest.unwrap(), &map_nearest);
            i = i + 1;
        }
    }
}
