use crate::primitives::Point;
use num_traits::float::Float;
use std::marker::PhantomData;

pub trait Distance {
    type F;
    fn distance(x: Point<Self::F>, y: Point<Self::F>) -> Self::F;
}

pub struct Euclidean<F: Float> {
    _marker: PhantomData<F>,
}

impl<F> Distance for Euclidean<F>
where
    F: Float,
{
    type F = F;
    fn distance(a: Point<F>, b: Point<F>) -> F {
        F::sqrt((a.x - b.x).powi(2) + (a.y - b.y).powi(2))
    }
}

pub struct Manhattan<F: Float> {
    _marker: PhantomData<F>,
}

impl<F> Distance for Manhattan<F>
where
    F: Float,
{
    type F = F;
    fn distance(a: Point<F>, b: Point<F>) -> F {
        (a.x - b.x).abs() + (a.y - b.y).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_f32() {
        let a = Point::<f32> {
            id: 0,
            x: 0.,
            y: 0.,
        };
        let b = Point::<f32> {
            id: 0,
            x: 1.,
            y: 1.,
        };
        let d = Euclidean::distance(a, b);
        assert_delta_f32!(d, 1.4142135, 0.00001);
    }

    #[test]
    fn test_euclidean_f64() {
        let a = Point::<f64> {
            id: 0,
            x: 0.,
            y: 0.,
        };
        let b = Point::<f64> {
            id: 0,
            x: 1.,
            y: 1.,
        };
        let d = Euclidean::distance(a, b);
        assert_delta!(d, 1.4142135, 0.00001);
    }

    #[test]
    fn test_manhattan_f32() {
        let a = Point::<f32> {
            id: 0,
            x: 0.,
            y: 0.,
        };
        let b = Point::<f32> {
            id: 0,
            x: 1.,
            y: 1.,
        };
        let d = Manhattan::distance(a, b);
        assert_delta_f32!(d, 2., 0.00001);
    }

    #[test]
    fn test_manhattan_f64() {
        let a = Point::<f64> {
            id: 0,
            x: 0.,
            y: 0.,
        };
        let b = Point::<f64> {
            id: 0,
            x: 1.,
            y: 1.,
        };
        let d = Manhattan::distance(a, b);
        assert_delta!(d, 2., 0.00001);
    }
}
