use crate::primitives::Point;
use num_traits::float::Float;
use std::cmp::Ordering;

/// State for store nearest neighbors distances and points in min_heap
#[derive(Copy, Clone, PartialEq)]
pub struct NearestNeighborState<F>
where
    F: Float,
{
    pub distance: F,
    pub point: Point<F>,
}

impl<F: Float> Eq for NearestNeighborState<F> {}

impl<F> PartialOrd for NearestNeighborState<F>
where
    F: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // We flip the ordering on distance, so the queue becomes a min-heap
        other.distance.partial_cmp(&self.distance)
    }
}

impl<F> Ord for NearestNeighborState<F>
where
    F: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
// fn local_min_heap<'a, M, F>(
//     map: &'a LearnedHashMap<M, F>,
//     heap: &mut BinaryHeap<NearestNeighborState<'a, F>>,
//     local_hash: u64,
//     query_point: &(F, F),
// ) where
//     F: Float,
//     M: Model<F = F> + Default + Clone,
// {
//     let bucket = &map.table[local_hash as usize];
//     if !bucket.is_empty() {
//         for p in bucket.iter() {
//             let d = Euclidean::distance(query_point, &(p.x, p.y));
//             heap.push(NearestNeighborState {
//                 distance: d,
//                 point: p,
//             });
//         }
//     }
// }
//
// /// Nearest neighbor search for the cloest point for given query point
// pub fn nearest_neighbor<M, F>(
//     map: &mut LearnedHashMap<M, F>,
//     query_point: &(F, F),
// ) -> Option<Point<F>>
// where
//     F: Float + Default + AsPrimitive<u64> + FromPrimitive,
//     M: Model<F = F> + Default + Clone,
// {
//     let mut hash = make_hash_point(&mut map.hasher, query_point);
//     let max_capacity = map.table.capacity() as u64;
//
//     // if hash out of max bound, still search right most bucket
//     if hash > max_capacity {
//         hash = max_capacity - 1;
//     }
//
//     let mut heap = BinaryHeap::new();
//     let mut min_d = F::max_value();
//     let mut nearest_neighbor = Point::new();
//
//     // Searching at current hash index
//     let map_clone = map.clone();
//     local_min_heap(&map_clone, &mut heap, hash, query_point);
//
//     match heap.pop() {
//         Some(v) => {
//             let local_min_d = v.distance;
//             // Update the nearest neighbour and minimum distance
//             if local_min_d < min_d {
//                 nearest_neighbor = *v.point;
//                 min_d = local_min_d;
//             }
//         }
//         None => (),
//     }
//
//     // Measure left vertical distance from current bucket to left hash bucket
//     // left hash must >= 0
//     let mut left_hash = hash.saturating_sub(1);
//
//     // Unhash the left_hash, then calculate the vertical distance between
//     // left hash point and query point
//     let left_x = unhash(&mut map.hasher, left_hash);
//     let mut min_left_d = Euclidean::distance(&(query_point.0, F::zero()), &(left_x, F::zero()));
//
//     // Iterate over left
//     while min_left_d < min_d {
//         local_min_heap(&map_clone, &mut heap, left_hash, query_point);
//         match heap.pop() {
//             Some(v) => {
//                 min_left_d = v.distance;
//                 // Update the nearest neighbour and minimum distance
//                 if min_left_d < min_d {
//                     nearest_neighbor = *v.point;
//                     min_d = min_left_d;
//                 }
//             }
//             None => (),
//         }
//         // Move to next left bucket
//         left_hash = left_hash.saturating_sub(1);
//     }
//
//     // Measure right vertical distance from current bucket to right hash bucket
//     let mut right_hash = hash + 1;
//
//     // Unhash the right_hash, then calculate the vertical distance between
//     // right hash point and query point
//     let right_x = unhash(&mut map.hasher, right_hash);
//     let mut min_right_d = Euclidean::distance(&(query_point.0, F::zero()), &(right_x, F::zero()));
//
//     // Iterate over right
//     while min_right_d < min_d {
//         local_min_heap(map, &mut heap, right_hash, query_point);
//         match heap.pop() {
//             Some(v) => {
//                 min_right_d = v.distance;
//                 // Update the nearest neighbour and minimum distance
//                 if min_right_d < min_d {
//                     nearest_neighbor = *v.point;
//                     min_d = min_right_d;
//                 }
//             }
//             None => (),
//         }
//         // Move to next right bucket
//         right_hash += 1;
//     }
//
//     Some(nearest_neighbor)
// }
