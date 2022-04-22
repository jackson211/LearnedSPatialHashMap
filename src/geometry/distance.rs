use crate::geometry::Point;
use num_traits::float::Float;
use std::marker::PhantomData;

/// Distance trait for measuring the distance between two points
pub trait Distance {
    type F;
    /// Distance between two points in tuple format
    fn distance(a: &[Self::F; 2], b: &[Self::F; 2]) -> Self::F;
    /// Distance between two points in points format
    fn distance_point(a: &Point<Self::F>, b: &Point<Self::F>) -> Self::F;
}

/// Euclidean Distance
pub struct Euclidean<F: Float> {
    _marker: PhantomData<F>,
}

impl<F> Distance for Euclidean<F>
where
    F: Float,
{
    type F = F;
    fn distance(a: &[F; 2], b: &[F; 2]) -> F {
        F::sqrt((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2))
    }

    fn distance_point(a: &Point<Self::F>, b: &Point<Self::F>) -> Self::F {
        Self::distance(&[a.x, a.y], &[b.x, b.y])
    }
}

/// Manhattan Distance
pub struct Manhattan<F: Float> {
    _marker: PhantomData<F>,
}

impl<F> Distance for Manhattan<F>
where
    F: Float,
{
    type F = F;
    fn distance(a: &[F; 2], b: &[F; 2]) -> F {
        (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
    }

    fn distance_point(a: &Point<Self::F>, b: &Point<Self::F>) -> Self::F {
        Self::distance(&[a.x, a.y], &[b.x, b.y])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_f32() {
        let a = Point::<f32> { x: 0., y: 0. };
        let b = Point::<f32> { x: 1., y: 1. };
        let d = Euclidean::distance_point(&a, &b);
        assert_delta_f32!(d, 1.4142135, 0.00001);
    }

    #[test]
    fn test_euclidean_f64() {
        let a = Point::<f64> { x: 0., y: 0. };
        let b = Point::<f64> { x: 1., y: 1. };
        let d = Euclidean::distance_point(&a, &b);
        assert_delta!(d, 1.4142135, 0.00001);
    }

    #[test]
    fn test_manhattan_f32() {
        let a = Point::<f32> { x: 0., y: 0. };
        let b = Point::<f32> { x: 1., y: 1. };
        let d = Manhattan::distance_point(&a, &b);
        assert_delta_f32!(d, 2., 0.00001);
    }

    #[test]
    fn test_manhattan_f64() {
        let a = Point::<f64> { x: 0., y: 0. };
        let b = Point::<f64> { x: 1., y: 1. };
        let d = Manhattan::distance_point(&a, &b);
        assert_delta!(d, 2., 0.00001);
    }
}
