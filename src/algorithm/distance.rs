use crate::primitives::Point;
use num_traits::float::Float;
use std::marker::PhantomData;

pub trait Distance {
    type F;
    fn distance(a: &(Self::F, Self::F), b: &(Self::F, Self::F)) -> Self::F;
    fn distance_point(a: &Point<Self::F>, b: &Point<Self::F>) -> Self::F;
}

pub struct Euclidean<F: Float> {
    _marker: PhantomData<F>,
}

impl<F> Distance for Euclidean<F>
where
    F: Float,
{
    type F = F;
    fn distance(a: &(F, F), b: &(F, F)) -> F {
        F::sqrt((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2))
    }

    fn distance_point(a: &Point<Self::F>, b: &Point<Self::F>) -> Self::F {
        Self::distance(&(a.x, a.y), &(b.x, b.y))
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
    fn distance(a: &(F, F), b: &(F, F)) -> F {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }

    fn distance_point(a: &Point<Self::F>, b: &Point<Self::F>) -> Self::F {
        Self::distance(&(a.x, a.y), &(b.x, b.y))
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
        let d = Euclidean::distance_point(&a, &b);
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
        let d = Euclidean::distance_point(&a, &b);
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
        let d = Manhattan::distance_point(&a, &b);
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
        let d = Manhattan::distance_point(&a, &b);
        assert_delta!(d, 2., 0.00001);
    }
}
