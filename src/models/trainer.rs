use crate::{
    error::Error,
    geometry::{helper::*, Axis, Point},
    models::{variance, Model},
};
use core::iter::Sum;
use num_traits::{cast::FromPrimitive, Float};

/// Preprocessing and prepare data for model training
///
#[derive(Debug, Clone)]
pub struct Trainer<F> {
    train_x: Vec<F>,
    train_y: Vec<F>,
    axis: Axis,
}

impl<F> Default for Trainer<F> {
    fn default() -> Self {
        Self {
            train_x: Vec::<F>::new(),
            train_y: Vec::<F>::new(),
            axis: Axis::X,
        }
    }
}

impl<F> Trainer<F>
where
    F: Float + Sized,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn train_x(&self) -> &Vec<F> {
        &self.train_x
    }

    pub fn train_y(&self) -> &Vec<F> {
        &self.train_y
    }

    pub fn set_train_x(&mut self, xs: Vec<F>) {
        self.train_x = xs
    }

    pub fn set_train_y(&mut self, ys: Vec<F>) {
        self.train_y = ys
    }

    pub fn set_axis(&mut self, axis: Axis) {
        self.axis = axis
    }

    /// Training with provided model
    ///
    /// Returns trained/fitted model on success, otherwise returns an error
    pub fn train<'a, M: Model<F = F> + 'a>(&self, model: &'a mut M) -> Result<&'a M, Error> {
        model.fit(&self.train_x, &self.train_y)?;
        Ok(model)
    }
}

impl<F> Trainer<F>
where
    F: Float + Sum + FromPrimitive,
{
    /// Initialize Trainer with two Vec<F>
    ///
    /// Returns prepared Trainer Ok((Trainer)) on success, otherwise returns an error
    pub fn with_data(xs: Vec<F>, ys: Vec<F>) -> Result<(Self, Vec<Point<F>>), Error> {
        assert_empty!(xs);
        assert_eq_len!(xs, ys);

        let mut trainer = Trainer::new();
        let data = trainer.preprocess(xs, ys)?;
        Ok((trainer, data))
    }

    /// Preprocess two Vec<F> that satisfy Trainer's requirements
    ///
    /// Returns sorted Ok(Vec<Point<F>>) on success, otherwise returns an error
    pub fn preprocess(&mut self, xs: Vec<F>, ys: Vec<F>) -> Result<Vec<Point<F>>, Error> {
        assert_empty!(xs);
        assert_eq_len!(xs, ys);

        let mut ps: Vec<Point<F>> = xs
            .iter()
            .zip(ys.iter())
            .enumerate()
            .map(|(id, (&x, &y))| Point { id, x, y })
            .collect();

        // set train_x to data with larger variance
        if variance(&xs) > variance(&ys) {
            // sort along x
            sort_by_x(&mut ps);
            reset_id(&mut ps);
            self.set_axis(Axis::X);
            self.set_train_x(extract_x(&ps));
        } else {
            // sort along y
            sort_by_y(&mut ps);
            reset_id(&mut ps);
            self.set_axis(Axis::Y);
            self.set_train_x(extract_y(&ps));
        };

        self.set_train_y(extract_id(&ps));
        Ok(ps)
    }

    /// Preprocess with Vec<Point<F>> that satisfy Trainer's requirements
    ///
    /// Returns prepared Trainer Ok((Trainer)) on success, otherwise returns an error
    pub fn with_points(ps: &mut [Point<F>]) -> Result<Self, Error> {
        let px: Vec<F> = extract_x(ps);
        let py: Vec<F> = extract_y(ps);
        assert_eq_len!(px, py);
        let x_variance = variance(&px);
        let y_variance = variance(&py);
        // set train_x to data with larger variance
        let (axis, train_x, train_y) = if x_variance > y_variance {
            // sort along x
            sort_by_x(ps);
            reset_id(ps);
            let id: Vec<F> = extract_id(ps);
            (Axis::X, px, id)
        } else {
            // sort along y
            sort_by_y(ps);
            reset_id(ps);
            let id: Vec<F> = extract_id(ps);
            (Axis::Y, py, id)
        };

        Ok(Self {
            train_x,
            train_y,
            axis,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_by() {
        let mut data: Vec<Point<f64>> = vec![
            Point {
                id: 1,
                x: 1.,
                y: 1.,
            },
            Point {
                id: 2,
                x: 3.,
                y: 1.,
            },
            Point {
                id: 3,
                x: 2.,
                y: 1.,
            },
            Point {
                id: 4,
                x: 3.,
                y: 2.,
            },
            Point {
                id: 5,
                x: 5.,
                y: 1.,
            },
        ];
        let data_sort_by_x: Vec<Point<f64>> = vec![
            Point {
                id: 1,
                x: 1.,
                y: 1.,
            },
            Point {
                id: 3,
                x: 2.,
                y: 1.,
            },
            Point {
                id: 2,
                x: 3.,
                y: 1.,
            },
            Point {
                id: 4,
                x: 3.,
                y: 2.,
            },
            Point {
                id: 5,
                x: 5.,
                y: 1.,
            },
        ];
        sort_by_x(&mut data);

        assert_eq!(data_sort_by_x, data);
    }

    #[test]
    fn train() {
        let mut data: Vec<Point<f64>> = vec![
            Point {
                id: 1,
                x: 1.,
                y: 1.,
            },
            Point {
                id: 2,
                x: 3.,
                y: 1.,
            },
            Point {
                id: 3,
                x: 2.,
                y: 1.,
            },
            Point {
                id: 4,
                x: 3.,
                y: 2.,
            },
            Point {
                id: 5,
                x: 5.,
                y: 1.,
            },
        ];
        let trainer = Trainer::with_points(&mut data).unwrap();
        let test_x = vec![1., 3., 2., 3., 5.];
        let test_y = vec![0., 1., 2., 3., 4.];

        assert_eq!(&test_x, trainer.train_x());
        assert_eq!(&test_y, trainer.train_y());
    }
}
