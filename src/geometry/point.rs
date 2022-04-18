use num_traits::float::Float;

/// Point struct contains id, x and y
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Point<T> {
    pub(crate) id: usize,
    pub(crate) x: T,
    pub(crate) y: T,
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
    pub fn new(id: usize, x: T, y: T) -> Self {
        Point { id, x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}
