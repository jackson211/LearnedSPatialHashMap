use crate::geometry::Point;
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
