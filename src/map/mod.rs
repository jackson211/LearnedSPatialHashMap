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

/// LearnedHashMap takes a model instead of an hasher for hashing indexes in the table.
///
/// Default Model for the LearndedHashMap is Linear regression.
/// In order to build a ordered HashMap, we need to make sure that the model is **monotonic**.
#[derive(Debug, Clone)]
pub struct LearnedHashMap<M, F> {
    hasher: LearnedHasher<M>,
    table: Table<Point<F>>,
    items: usize,
}

/// Default for the LearndedHashMap.
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
    F: Float + Default + AsPrimitive<u64> + FromPrimitive + Debug + Sum,
    M: Model<F = F> + Default + Clone,
{
    /// Returns a default LearnedHashMap with Model and Float type.
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel};
    /// let map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a default LearnedHashMap with Model and Float type.
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
    #[inline]
    pub fn with_hasher(hasher: LearnedHasher<M>) -> Self {
        Self {
            hasher,
            table: Table::new(),
            items: 0,
        }
    }

    /// Returns a default LearnedHashMap with Model and Float type.
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
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            hasher: Default::default(),
            table: Table::with_capacity(capacity),
            items: 0,
        }
    }

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
    #[inline]
    pub fn with_data(data: &[[F; 2]]) -> Result<(Self, Vec<Point<F>>), Error> {
        use crate::helper::convert_to_points;
        let mut map = LearnedHashMap::with_capacity(data.len());
        let mut ps = convert_to_points(data).unwrap();
        match map.batch_insert(&mut ps) {
            Ok(()) => Ok((map, ps)),
            Err(err) => Err(err),
        }
    }

    /// Returns Option<Point<F>>  with given point data.
    ///
    /// # Arguments
    /// * `p` - A array slice containing two points for querying
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, LearnedHasher};
    /// let point_data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
    /// let (mut map, points) = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&point_data).unwrap();
    ///
    /// assert_eq!(map.get(&[1., 1.]).is_some(), true);
    /// ```
    #[inline]
    pub fn get(&mut self, p: &[F; 2]) -> Option<&Point<F>> {
        let hash = make_hash_point(&mut self.hasher, p) as usize;
        if hash > self.table.capacity() {
            return None;
        }
        self.find_by_hash(hash, p)
    }

    /// Returns Option<Point<F>> by hash index, if it exists in the map.
    ///
    /// # Arguments
    /// * `hash` - An usize hash value
    /// * `p` - A array slice containing two points for querying
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, LearnedHasher};
    /// let point_data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
    /// let (mut map, points) = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&point_data).unwrap();
    ///
    /// assert_eq!(map.find_by_hash(0, &[1., 1.]).is_some(), true);
    /// assert_eq!(map.find_by_hash(1, &[1., 1.]).is_none(), true);
    /// ```
    #[inline]
    pub fn find_by_hash(&self, hash: usize, p: &[F; 2]) -> Option<&Point<F>> {
        self.table[hash]
            .iter()
            .find(|&ep| ep.x == p[0] && ep.y == p[1])
    }

    /// Returns bool.
    ///
    /// # Arguments
    /// * `p` - A array slice containing two points for querying
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, LearnedHasher};
    /// let point_data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
    /// let (mut map, points) = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&point_data).unwrap();
    ///
    /// assert_eq!(map.contains_points(&[1., 1.]), true);
    /// assert_eq!(map.contains_points(&[0., 1.]), false);
    /// ```
    #[inline]
    pub fn contains_points(&mut self, p: &[F; 2]) -> bool {
        self.get(p).is_some()
    }

    /// Returns Option<Point<F>> if the map contains a point and successful remove it from the map.
    ///
    /// # Arguments
    /// * `p` - A Point data
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, LearnedHasher};
    /// let point_data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
    /// let (mut map, points) = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&point_data).unwrap();
    ///
    /// let p = points[0];
    /// assert_eq!(map.remove(&p).unwrap(), p);
    /// ```
    #[inline]
    pub fn remove(&mut self, p: &Point<F>) -> Option<Point<F>> {
        let hash = make_hash_point(&mut self.hasher, &[p.x, p.y]);
        self.items -= 1;
        self.table.remove_entry(hash, *p)
    }

    /// Returns usize length.
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, LearnedHasher};
    /// let point_data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
    /// let (mut map, points) = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&point_data).unwrap();
    ///
    /// assert_eq!(map.len(), 4);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Returns usize number of items.
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, LearnedHasher};
    /// let point_data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
    /// let (mut map, points) = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&point_data).unwrap();
    ///
    /// assert_eq!(map.items(), 4);
    /// ```
    #[inline]
    pub fn items(&self) -> usize {
        self.items
    }

    /// Returns bool if the map is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, LearnedHasher};
    /// let point_data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
    /// let (mut map, points) = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&point_data).unwrap();
    ///
    /// assert_eq!(map.is_empty(), false);
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    /// Resize the map if needed, it will initialize the map to the INITIAL_NBUCKETS, otherwise it will double the capacity if table is not empty.
    fn resize(&mut self) {
        let target_size = match self.table.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };
        self.resize_with_capacity(target_size);
    }

    /// Resize the map if needed, it will resize the map to desired capacity.
    #[inline]
    fn resize_with_capacity(&mut self, target_size: usize) {
        let mut new_table = Table::with_capacity(target_size);
        new_table.extend((0..target_size).map(|_| Bucket::new()));

        for p in self.table.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let hash = make_hash_point(&mut self.hasher, &[p.x, p.y]) as usize;
            new_table[hash].push(p);
        }

        self.table = new_table;
    }

    /// Rehash the map.
    #[inline]
    fn rehash(&mut self) -> Result<(), Error> {
        let mut old_data = Vec::with_capacity(self.items());
        for p in self.table.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            old_data.push(p);
        }
        self.batch_insert(&mut old_data)
    }

    /// Inner function for insert a single point into the map
    #[inline]
    fn insert_inner(&mut self, p: Point<F>) -> Option<Point<F>> {
        // Resize if the table is empty or 3/4 size of the table is full
        if self.table.is_empty() || self.items() > 3 * self.table.len() / 4 {
            self.resize();
        }

        // Find where to put the key at second bucket
        let p_value = match self.hasher.sort_by_x() {
            true => p.x,
            false => p.y,
        };

        let hash = make_hash_point::<M, F>(&mut self.hasher, &[p.x, p.y]);
        self.insert_with_axis(p_value, p, hash)
    }

    /// Sequencial insert a point into the map.
    ///
    /// # Arguments
    /// * `p` - A Point<F> with float number
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, Point};
    /// let a: Point<f64> = Point::new(0., 1.);
    /// let b: Point<f64> = Point::new(1., 0.);

    /// let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    /// map.insert(a);
    /// map.insert(b);

    /// assert_eq!(map.items(), 2);
    /// assert_eq!(map.get(&[0., 1.]).unwrap(), &a);
    /// assert_eq!(map.get(&[1., 0.]).unwrap(), &b);
    /// ```
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

        let hash = make_hash_point::<M, F>(&mut self.hasher, &[p.x, p.y]);
        // resize if hash index is larger or equal to the table capacity
        if hash >= self.table.capacity() as u64 {
            self.resize_with_capacity(hash as usize * 2);
            self.insert_with_axis(p_value, p, hash);
            match self.rehash() {
                Ok(_) => None,
                Err(err) => {
                    eprintln!("{:?}", err);
                    None
                }
            }
        } else {
            self.insert_with_axis(p_value, p, hash)
        }
    }

    /// Insert a point into the map along the given axis.
    ///
    /// # Arguments
    /// * `p_value` - A float number represent the key of a 2d point
    #[inline]
    fn insert_with_axis(&mut self, p_value: F, p: Point<F>, hash: u64) -> Option<Point<F>> {
        let mut insert_index = 0;
        let bucket_index = self.table.bucket(hash);
        let bucket = &mut self.table[bucket_index];
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

    /// Fit the input data into the model of the hasher. Returns Error if error occurred during
    /// model fitting.
    ///
    /// # Arguments
    ///
    /// * `xs` - A list of tuple of floating number
    /// * `ys` - A list of tuple of floating number
    #[inline]
    pub fn model_fit(&mut self, xs: &[F], ys: &[F]) -> Result<(), Error> {
        self.hasher.model.fit(xs, ys)
    }

    /// Fit the input data into the model of the hasher. Returns Error if error occurred during
    /// model fitting.
    ///
    /// # Arguments
    /// * `data` - A list of tuple of floating number
    #[inline]
    pub fn model_fit_tuple(&mut self, data: &[(F, F)]) -> Result<(), Error> {
        self.hasher.model.fit_tuple(data)
    }

    /// Inner function for batch insert
    #[inline]
    fn batch_insert_inner(&mut self, ps: &[Point<F>]) {
        // Allocate table capacity before insert
        let n = ps.len();
        self.resize_with_capacity(n);
        for p in ps.iter() {
            self.insert_inner(*p);
        }
    }

    /// Batch insert a batch of 2d data into the map.
    ///
    /// # Arguments
    /// * `ps` - A list of point number
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel};
    /// let point_data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
    /// let (mut map, points) = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&point_data).unwrap();
    ///
    /// assert_eq!(map.get(&[1., 1.]).is_some(), true);
    /// ```
    #[inline]
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

            // Fit the data into model
            self.model_fit(trainer.train_x(), trainer.train_y())
                .unwrap();
            // Batch insert into the map
            self.batch_insert_inner(ps);
        }
        Ok(())
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
    #[inline]
    pub fn range_search(
        &mut self,
        bottom_left: &[F; 2],
        top_right: &[F; 2],
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
                if item.x >= bottom_left[0]
                    && item.x <= top_right[0]
                    && item.y >= bottom_left[1]
                    && item.y <= top_right[1]
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

    /// Returns Option<Vec<Point<F>>> if points are found in the map with given range
    ///
    /// # Arguments
    /// * `query_point` - A Point data for querying
    /// * `radius` - A radius value
    ///
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, LearnedHasher};
    /// let point_data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
    /// let (mut map, points) = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&point_data).unwrap();
    /// assert_eq!(map.range_search(&[0., 0.], &[3., 3.]).is_some(), true);
    /// ```
    #[inline]
    pub fn radius_range(&mut self, query_point: &[F; 2], radius: F) -> Option<Vec<Point<F>>> {
        self.range_search(
            &[query_point[0] - radius, query_point[1] - radius],
            &[query_point[0] + radius, query_point[1] + radius],
        )
    }

    /// Find the local minimum distance between query points and cadidates neighbors, then store
    /// the cadidates neighbors in the min_heap.
    ///
    ///
    /// # Arguments
    /// * `heap` - mutable borrow of an BinaryHeap
    /// * `local_hash` - A hash index of local bucket
    /// * `query_point` - A Point data
    /// * `min_d` - minimum distance
    /// * `nearest_neighbor` - mutable borrow of an point data, which is the nearest neighbor at
    /// search index bucket
    #[inline]
    fn local_min_heap(
        &self,
        heap: &mut BinaryHeap<NearestNeighborState<F>>,
        local_hash: u64,
        query_point: &[F; 2],
        min_d: &mut F,
        nearest_neighbor: &mut Point<F>,
    ) {
        let bucket = &self.table[local_hash as usize];
        if !bucket.is_empty() {
            for p in bucket.iter() {
                let d = Euclidean::distance(query_point, &[p.x, p.y]);
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

    /// Calculates the horizontal distance between query_point and bucket at index with given hash.
    ///
    /// # Arguments
    /// * `hash` - A hash index of the bucket
    /// * `query_point` - A Point data
    #[inline]
    fn horizontal_distance(&mut self, query_point: &[F; 2], hash: u64) -> F {
        let x = unhash(&mut self.hasher, hash);
        match self.hasher.sort_by_x() {
            true => Euclidean::distance(&[query_point[0], F::zero()], &[x, F::zero()]),
            false => Euclidean::distance(&[query_point[1], F::zero()], &[x, F::zero()]),
        }
    }

    /// Nearest neighbor search for the closest point for given query point
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
    /// # Examples
    ///
    /// ```
    /// use lsph::{LearnedHashMap, LinearModel, LearnedHasher};
    /// let point_data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
    /// let (mut map, points) = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&point_data).unwrap();
    /// assert_eq!(map.nearest_neighbor(&[2., 1.]).is_some(), true);
    /// ```
    #[inline]
    pub fn nearest_neighbor(&mut self, query_point: &[F; 2]) -> Option<Point<F>> {
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
        let a: Point<f64> = Point::new(0., 1.);
        let b: Point<f64> = Point::new(1., 0.);

        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        map.insert(a);
        map.insert(b);

        assert_eq!(map.items(), 2);
        assert_eq!(map.get(&[0., 1.]).unwrap(), &a);
        assert_eq!(map.get(&[1., 0.]).unwrap(), &b);
    }

    #[test]
    fn insert_repeated() {
        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        let a: Point<f64> = Point::new(0., 1.);
        let b: Point<f64> = Point::new(1., 0.);
        let res = map.insert(a);
        assert_eq!(map.items(), 1);
        assert_eq!(res, None);

        let res = map.insert(b);
        assert_eq!(map.items(), 2);
        assert_eq!(res, None);
    }

    #[test]
    fn with_data() {
        let data = vec![[1., 1.], [2., 1.], [3., 2.], [4., 4.]];
        let (mut map, _points) = LearnedHashMap::<LinearModel<f64>, f64>::with_data(&data).unwrap();
        assert_eq!(map.get(&[1., 1.]).is_some(), true);
    }

    #[test]
    fn fit_batch_insert() {
        let mut data: Vec<Point<f64>> = vec![
            Point::new(1., 1.),
            Point::new(3., 1.),
            Point::new(2., 1.),
            Point::new(3., 2.),
            Point::new(5., 1.),
        ];
        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        map.batch_insert(&mut data).unwrap();
        dbg!(&map);

        assert_delta!(1.02272, map.hasher.model.coefficient, 0.00001);
        assert_delta!(-0.86363, map.hasher.model.intercept, 0.00001);
        assert_eq!(Some(&Point::new(1., 1.)), map.get(&[1., 1.]));
        assert_eq!(Some(&Point::new(3., 1.,)), map.get(&[3., 1.]));
        assert_eq!(Some(&Point::new(5., 1.)), map.get(&[5., 1.]));

        assert_eq!(None, map.get(&[5., 2.]));
        assert_eq!(None, map.get(&[2., 2.]));
        assert_eq!(None, map.get(&[50., 10.]));
        assert_eq!(None, map.get(&[500., 100.]));
    }

    #[test]
    fn insert_after_batch_insert() {
        let mut data: Vec<Point<f64>> = vec![
            Point::new(1., 1.),
            Point::new(3., 1.),
            Point::new(2., 1.),
            Point::new(3., 2.),
            Point::new(5., 1.),
        ];
        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        map.batch_insert(&mut data).unwrap();
        dbg!(&map);

        let a: Point<f64> = Point::new(10., 10.);
        map.insert(a.clone());
        assert_eq!(Some(&a), map.get(&[10., 10.]));

        let b: Point<f64> = Point::new(100., 100.);
        map.insert(b.clone());
        assert_eq!(Some(&b), map.get(&[100., 100.]));
        assert_eq!(None, map.get(&[100., 101.]));
    }

    #[test]
    fn range_search() {
        let mut data: Vec<Point<f64>> = vec![
            Point::new(1., 1.),
            Point::new(2., 2.),
            Point::new(3., 3.),
            Point::new(4., 4.),
            Point::new(5., 5.),
        ];
        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
        map.batch_insert(&mut data).unwrap();
        // dbg!(&map);

        let found: Vec<Point<f64>> =
            vec![Point::new(1., 1.), Point::new(2., 2.), Point::new(3., 3.)];

        assert_eq!(Some(found), map.range_search(&[1., 1.], &[3.5, 3.]));

        let found: Vec<Point<f64>> = vec![Point::new(1., 1.)];

        assert_eq!(Some(found), map.range_search(&[1., 1.], &[3., 1.]));
        assert_eq!(None, map.range_search(&[4., 2.], &[5., 3.]));
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
                .nearest_neighbor(&[sample_point.x, sample_point.y])
                .unwrap();
            assert_eq!(nearest.unwrap(), &map_nearest);
            i = i + 1;
        }
    }
}
