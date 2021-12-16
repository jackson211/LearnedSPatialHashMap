use num_traits::float::Float;
use std::fmt;

#[derive(Debug)]
pub struct Point<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x: x, y: y }
    }
}

impl<T: Float> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_point_debug() {
//         assert!();
//     }
// }
