use crate::{
    error::*,
    models::{stats::root_mean_squared_error, Model},
};

use core::fmt::Debug;
use core::iter::Sum;
use num_traits::{cast::FromPrimitive, float::Float};

/// Simple linear regression from tuples.
///
/// Calculates the simple linear regression from array of tuples, and their means.
///
/// # Arguments
///
/// * `xys` -  An array of tuples of training data that contains Xs and Ys.
///
/// * `x_mean` - The mean of Xs training data.  
///
/// * `y_mean` - The mean of Ys target values.  
///
/// Returns `Ok(slope, intercept)` or Err(Error).
///
/// # Errors
///
/// Returns an error if
///
/// * `xs` and `ys` differ in length
/// * `xs` or `ys` are empty
/// * the slope is too steep to represent, approaching infinity
/// * the number of elements cannot be represented as an `F`
fn slr<I, F>(xys: I, x_mean: F, y_mean: F) -> Result<(F, F), Error>
where
    I: Iterator<Item = (F, F)>,
    F: Float + Debug,
{
    // compute the covariance of x and y as well as the variance of x
    let (sq_diff_sum, cov_diff_sum) = xys.fold((F::zero(), F::zero()), |(v, c), (x, y)| {
        let diff = x - x_mean;
        let sq_diff = diff * diff;
        let cov_diff = diff * (y - y_mean);
        (v + sq_diff, c + cov_diff)
    });
    let slope = cov_diff_sum / sq_diff_sum;
    if slope.is_nan() {
        return Err(Error::SteepSlope);
    }
    let intercept = y_mean - slope * x_mean;
    Ok((slope, intercept))
}

/// Two-pass simple linear regression from slices.
///
/// Calculates the linear regression from two slices, one for x- and one for y-values, by
/// calculating the mean and then calling `lin_reg`.
///
/// # Arguments
///
/// * `xs` -  An array of tuples of training data.
///
/// * `ys` -  An array of tuples of targeting data.
///
/// Returns `Ok(slope, intercept)` of the regression line.
///
/// # Errors
///
/// Returns an error if
///
/// * `xs` and `ys` differ in length
/// * `xs` or `ys` are empty
/// * the slope is too steep to represent, approaching infinity
/// * the number of elements cannot be represented as an `F`
fn linear_regression<X, Y, F>(xs: &[X], ys: &[Y]) -> Result<(F, F), Error>
where
    X: Clone + Into<F>,
    Y: Clone + Into<F>,
    F: Float + Sum + Debug,
{
    assert_empty!(xs);
    assert_empty!(ys);
    assert_eq_len!(xs, ys);

    let n = F::from(xs.len()).ok_or(Error::EmptyVal)?;

    // compute the mean of x and y
    let x_sum: F = xs.iter().cloned().map(Into::into).sum();
    let x_mean = x_sum / n;
    let y_sum: F = ys.iter().cloned().map(Into::into).sum();
    let y_mean = y_sum / n;

    let data = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| (x.clone().into(), y.clone().into()));

    slr(data, x_mean, y_mean)
}

/// Two-pass linear regression from tuples.
///
/// Calculates the linear regression from a slice of tuple values by first calculating the mean
/// before calling `lin_reg`.
///
/// Returns `Ok(slope, intercept)` of the regression line.
///
/// # Errors
///
/// Returns an error if
///
/// * `xys` is empty
/// * the slope is too steep to represent, approaching infinity
/// * the number of elements cannot be represented as an `F`
fn linear_regression_tuple<X, Y, F>(xys: &[(X, Y)]) -> Result<(F, F), Error>
where
    X: Clone + Into<F>,
    Y: Clone + Into<F>,
    F: Float + Debug,
{
    assert_empty!(xys);

    // We're handrolling the mean computation here, because our generic implementation can't handle tuples.
    // If we ran the generic impl on each tuple field, that would be very cache inefficient
    let n = F::from(xys.len()).ok_or(Error::EmptyVal)?;
    let (x_sum, y_sum) = xys
        .iter()
        .cloned()
        .fold((F::zero(), F::zero()), |(sx, sy), (x, y)| {
            (sx + x.into(), sy + y.into())
        });
    let x_mean = x_sum / n;
    let y_mean = y_sum / n;

    slr(
        xys.iter()
            .map(|(x, y)| (x.clone().into(), y.clone().into())),
        x_mean,
        y_mean,
    )
}

/// Linear regression model
#[derive(Copy, Clone, Debug, Default)]
pub struct LinearModel<F> {
    pub coefficient: F,
    pub intercept: F,
}

impl<F> LinearModel<F>
where
    F: Float,
{
    pub fn new() -> LinearModel<F> {
        LinearModel {
            coefficient: F::zero(),
            intercept: F::zero(),
        }
    }
}

impl<F> Model for LinearModel<F>
where
    F: Float + FromPrimitive + Sum + Debug + Sized,
{
    type F = F;

    fn name(&self) -> String {
        String::from("linear")
    }

    fn fit(&mut self, xs: &[F], ys: &[F]) -> Result<(), Error> {
        let (coefficient, intercept): (F, F) = linear_regression(xs, ys).unwrap();
        self.coefficient = coefficient;
        self.intercept = intercept;
        Ok(())
    }
    fn fit_tuple(&mut self, xys: &[(F, F)]) -> Result<(), Error> {
        let (coefficient, intercept): (F, F) = linear_regression_tuple(xys).unwrap();
        self.coefficient = coefficient;
        self.intercept = intercept;
        Ok(())
    }

    fn predict(&self, x: F) -> F {
        x * self.coefficient + self.intercept
    }
    fn batch_predict(&self, xs: &[F]) -> Vec<F> {
        (0..xs.len()).map(|i| self.predict(xs[i])).collect()
    }

    fn evaluate(&self, x_test: &[F], y_test: &[F]) -> F {
        let y_predicted = self.batch_predict(x_test);
        root_mean_squared_error(y_test, &y_predicted)
    }

    fn unpredict(&self, y: F) -> F {
        (y - self.intercept) / self.coefficient
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn should_panic_for_empty_vecs() {
        let x_values: Vec<f64> = vec![];
        let y_values: Vec<f64> = vec![];
        let mut model = LinearModel::new();
        model.fit(&x_values, &y_values).unwrap();

        assert_delta!(0.8f64, model.coefficient, 0.00001);
        assert_delta!(0.4, model.intercept, 0.00001);
    }

    #[test]
    fn fit_coefficients() {
        let x_values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let y_values: Vec<f64> = vec![1., 3., 2., 3., 5.];
        let mut model = LinearModel::new();
        model.fit(&x_values, &y_values).unwrap();

        assert_delta!(0.8f64, model.coefficient, 0.00001);
        assert_delta!(0.4f64, model.intercept, 0.00001);
    }

    #[test]
    fn fit_coefficients_f32() {
        let x_values: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let y_values: Vec<f32> = vec![1., 3., 2., 3., 5.];
        let mut model = LinearModel::new();
        model.fit(&x_values, &y_values).unwrap();

        assert_delta_f32!(0.8, model.coefficient, 0.00001);
        assert_delta_f32!(0.4, model.intercept, 0.00001);
    }

    #[test]
    fn predict() {
        let x_values = vec![1f64, 2f64, 3f64, 4f64, 5f64];
        let y_values = vec![1f64, 3f64, 2f64, 3f64, 5f64];
        let mut model = LinearModel::new();
        model.fit(&x_values, &y_values).unwrap();

        assert_delta!(1.2f64, model.predict(1f64), 0.00001);
        assert_delta!(2f64, model.predict(2f64), 0.00001);
        assert_delta!(2.8f64, model.predict(3f64), 0.00001);
        assert_delta!(3.6f64, model.predict(4f64), 0.00001);
        assert_delta!(4.4f64, model.predict(5f64), 0.00001);
    }

    #[test]
    fn predict_list() {
        let x_values = vec![1f64, 2f64, 3f64, 4f64, 5f64];
        let y_values = vec![1f64, 3f64, 2f64, 3f64, 5f64];
        let mut model = LinearModel::new();
        model.fit(&x_values, &y_values).unwrap();

        let predictions = model.batch_predict(&x_values);

        assert_delta!(1.2f64, predictions[0], 0.00001);
        assert_delta!(2f64, predictions[1], 0.00001);
        assert_delta!(2.8f64, predictions[2], 0.00001);
        assert_delta!(3.6f64, predictions[3], 0.00001);
        assert_delta!(4.4f64, predictions[4], 0.00001);
    }

    #[test]
    fn evaluate() {
        let x_values = vec![1f64, 2f64, 3f64, 4f64, 5f64];
        let y_values = vec![1f64, 3f64, 2f64, 3f64, 5f64];
        let mut model = LinearModel::new();
        model.fit(&x_values.clone(), &y_values.clone()).unwrap();

        let error = model.evaluate(&x_values, &y_values);
        assert_delta!(0.69282f64, error, 0.00001);
    }
}
