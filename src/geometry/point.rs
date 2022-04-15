use num_traits::float::Float;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Point<T> {
    pub id: usize,
    pub x: T,
    pub y: T,
}

impl<T> Default for Point<T>
where
    T: Float,
{
    fn default() -> Self {
        Point {
            id: 0,
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T> Point<T>
where
    T: Float,
{
    pub fn new() -> Self {
        Self::default()
    }
}
