use crate::primitives::point::Point;
use num_traits::float::Float;

pub struct Line<T: Float> {
    y: Point<T>,
    x: Point<T>,
}
