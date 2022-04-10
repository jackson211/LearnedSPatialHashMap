use crate::{
    algorithm::{model::*, stats::root_mean_squared_error},
    error::Error,
};
use std::fmt::Debug;

/// Simple Linear Regression
///
/// Returns `Ok(slope, intercept)` of the regression line.
///
pub fn slr<I>(xys: I, x_mean: f32, y_mean: f32) -> Result<(f32, f32), Error>
where
    I: Iterator<Item = (f32, f32)>,
{
    // compute the covariance of x and y as well as the variance of x
    let (sq_diff_sum, cov_diff_sum) = xys.fold((0., 0.), |(v, c), (x, y)| {
        let diff = x - x_mean;
        let sq_diff = diff * diff;
        let cov_diff = diff * (y - y_mean);
        (v + sq_diff, c + cov_diff)
    });
    let slope = cov_diff_sum / sq_diff_sum;
    if slope.is_nan() {
        return Err(Error::TooSteep);
    }
    let intercept = y_mean - slope * x_mean;
    Ok((slope, intercept))
}

/// Two-pass simple linear regression from slices.
///
/// Calculates the linear regression from two slices, one for x- and one for y-values, by
/// calculating the mean and then calling `lin_reg`.
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
/// * the number of elements cannot be represented as an `f32`
///
pub fn linear_regression(xs: &[f32], ys: &[f32]) -> Result<(f32, f32), Error>
where
{
    if xs.len() != ys.len() {
        return Err(Error::InputLenDif);
    }

    if xs.is_empty() {
        return Err(Error::Mean);
    }

    let n = xs.len() as f32;

    // compute the mean of x and y
    let x_sum: f32 = xs.iter().cloned().sum();
    let x_mean = x_sum / n;
    let y_sum: f32 = ys.iter().cloned().sum();
    let y_mean = y_sum / n;

    let data = xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y));

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
/// * the number of elements cannot be represented as an `f32`
pub fn linear_regression_tuple(xys: &[(f32, f32)]) -> Result<(f32, f32), Error> {
    if xys.is_empty() {
        return Err(Error::Mean);
    }
    // We're handrolling the mean computation here, because our generic implementation can't handle tuples.
    // If we ran the generic impl on each tuple field, that would be very cache inefficient
    let n = xys.len() as f32;
    let (x_sum, y_sum) = xys
        .iter()
        .cloned()
        .fold((0., 0.), |(sx, sy), (x, y)| (sx + x, sy + y));
    let x_mean = x_sum / n;
    let y_mean = y_sum / n;

    slr(xys.iter().map(|(x, y)| (*x, *y)), x_mean, y_mean)
}

#[derive(Copy, Clone, Debug, Default)]
pub struct LinearModel {
    pub coefficient: f32,
    pub intercept: f32,
}

impl LinearModel {
    pub fn new() -> Self {
        LinearModel {
            coefficient: 0.,
            intercept: 0.,
        }
    }
}

impl Model for LinearModel {
    fn name(&self) -> String {
        String::from("linear")
    }

    fn fit(&mut self, xs: &[f32], ys: &[f32]) -> Result<(), Error> {
        (self.coefficient, self.intercept) = linear_regression(xs, ys).unwrap();
        Ok(())
    }

    fn fit_tuple(&mut self, xys: &[(f32, f32)]) -> Result<(), Error> {
        let (coefficient, intercept): (f32, f32) = linear_regression_tuple(xys).unwrap();
        self.coefficient = coefficient;
        self.intercept = intercept;
        Ok(())
    }

    fn predict(&self, x: f32) -> f32 {
        x * self.coefficient + self.intercept
    }

    fn predict_bytes(&self, bytes: &[u8; 4]) -> f32 {
        let value: f32 = f32::from_ne_bytes(*bytes);
        value * self.coefficient + self.intercept
    }

    fn batch_predict(&self, xs: &[f32]) -> Vec<f32> {
        (0..xs.len()).map(|i| self.predict(xs[i])).collect()
    }

    fn evaluate(&self, x_test: &[f32], y_test: &[f32]) -> f32 {
        let y_predicted = self.batch_predict(x_test);
        root_mean_squared_error(y_test, &y_predicted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn should_panic_for_empty_vecs() {
        let x_values: Vec<f32> = vec![];
        let y_values: Vec<f32> = vec![];
        let mut model = LinearModel::new();
        model.fit(&x_values, &y_values).unwrap();

        assert_delta!(0.8, model.coefficient, 0.00001);
        assert_delta!(0.4, model.intercept, 0.00001);
    }

    #[test]
    fn should_fit_coefficients_correctly() {
        let x_values: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let y_values: Vec<f32> = vec![1., 3., 2., 3., 5.];
        let mut model = LinearModel::new();
        model.fit(&x_values, &y_values).unwrap();

        assert_delta!(0.8, model.coefficient, 0.00001);
        assert_delta!(0.4, model.intercept, 0.00001);
    }

    #[test]
    fn should_predict_correctly() {
        let x_values: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let y_values: Vec<f32> = vec![1., 3., 2., 3., 5.];
        let mut model = LinearModel::new();
        model.fit(&x_values, &y_values).unwrap();

        assert_delta!(1.2, model.predict(1.), 0.00001);
        assert_delta!(2., model.predict(2.), 0.00001);
        assert_delta!(2.8, model.predict(3.), 0.00001);
        assert_delta!(3.6, model.predict(4.), 0.00001);
        assert_delta!(4.4, model.predict(5.), 0.00001);
    }

    #[test]
    fn should_predict_list_correctly() {
        let x_values: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let y_values: Vec<f32> = vec![1., 3., 2., 3., 5.];
        let mut model = LinearModel::new();
        model.fit(&x_values, &y_values).unwrap();

        let predictions = model.batch_predict(&x_values);

        assert_delta!(1.2, predictions[0], 0.00001);
        assert_delta!(2., predictions[1], 0.00001);
        assert_delta!(2.8, predictions[2], 0.00001);
        assert_delta!(3.6, predictions[3], 0.00001);
        assert_delta!(4.4, predictions[4], 0.00001);
    }

    #[test]
    fn should_evaluate_correctly() {
        let x_values: Vec<f32> = vec![1., 2., 3., 4., 5.];
        let y_values: Vec<f32> = vec![1., 3., 2., 3., 5.];
        let mut model = LinearModel::new();
        model.fit(&x_values.clone(), &y_values.clone()).unwrap();

        let error = model.evaluate(&x_values, &y_values);
        assert_delta!(0.69282, error, 0.00001);
    }
}
