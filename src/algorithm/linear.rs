use crate::algorithm::model::Model;
use crate::algorithm::stats::*;

use core::iter::Sum;
use num_traits::{cast::FromPrimitive, float::Float};

/// The kinds of errors that can occur when calculating a linear regression.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    /// The slope is too steep to represent, approaching infinity.
    TooSteep,
    /// Failed to calculate mean.
    ///
    /// This means the input was empty or had too many elements.
    Mean,
    /// Lengths of the inputs are different.
    InputLenDif,
    /// Can't compute linear regression of zero elements
    NoElements,
}

pub fn slr<I, F>(xys: I, x_mean: F, y_mean: F) -> Result<(F, F), Error>
where
    I: Iterator<Item = (F, F)>,
    F: Float,
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
/// * the number of elements cannot be represented as an `F`
///
pub fn linear_regression<X, Y, F>(xs: &[X], ys: &[Y]) -> Result<(F, F), Error>
where
    X: Clone + Into<F>,
    Y: Clone + Into<F>,
    F: Float + Sum,
{
    if xs.len() != ys.len() {
        return Err(Error::InputLenDif);
    }

    if xs.is_empty() {
        return Err(Error::Mean);
    }

    let n = F::from(xs.len()).ok_or(Error::Mean)?;
    // compute the mean of x and y

    let x_sum: F = xs.iter().cloned().map(Into::into).sum();
    let x_mean = x_sum / n;
    let y_sum: F = ys.iter().cloned().map(Into::into).sum();
    let y_mean = y_sum / n;

    let data = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| (x.clone().into(), y.clone().into()));

    slr(data.into_iter(), x_mean, y_mean)
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
pub fn linear_regression_of<X, Y, F>(xys: &[(X, Y)]) -> Result<(F, F), Error>
where
    X: Clone + Into<F>,
    Y: Clone + Into<F>,
    F: Float,
{
    if xys.is_empty() {
        return Err(Error::Mean);
    }
    // We're handrolling the mean computation here, because our generic implementation can't handle tuples.
    // If we ran the generic impl on each tuple field, that would be very cache inefficient
    let n = F::from(xys.len()).ok_or(Error::Mean)?;
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

pub struct LinearModel<F: Float> {
    pub coefficient: F,
    pub intercept: F,
}

impl<F> LinearModel<F>
where
    F: Float + Sum,
{
    pub fn new() -> LinearModel<F> {
        LinearModel {
            coefficient: F::zero(),
            intercept: F::zero(),
        }
    }

    fn fit(xs: &Vec<F>, ys: &Vec<F>) -> LinearModel<F> {
        let (coefficient, intercept): (F, F) = linear_regression(xs, ys).unwrap();
        LinearModel {
            coefficient: coefficient.into(),
            intercept: intercept.into(),
        }
    }
}

impl<F> Model for LinearModel<F>
where
    F: Float + FromPrimitive,
{
    type F = F;
    fn name(&self) -> String {
        String::from("linear")
    }

    fn predict(&self, x: F) -> F {
        x * self.coefficient + self.intercept
    }

    fn batch_predict(&self, xs: &Vec<F>) -> Vec<F> {
        (0..xs.len()).map(|i| self.predict(xs[i])).collect()
    }

    fn evaluate(&self, x_test: &Vec<F>, y_test: &Vec<F>) -> F {
        let y_predicted = self.batch_predict(x_test);
        return root_mean_squared_error(y_test, &y_predicted);
    }
}

// fn log_linear_fn(x_values: &Vec<f64>, y_values: &Vec<f64>) -> (f64, f64) {
//     let (x_log, y_log) = x_values
//         .iter()
//         .zip(y_values.iter())
//         .map(|(x, y)| (*x, y.ln()))
//         .filter(|(_, y)| y.is_finite())
//         .unzip();
//     linear_fn(&x_log, &y_log)
// }

// pub struct LogLinearModel {
//     pub params: (f64, f64),
// }

// impl LogLinearModel {
//     pub fn new(data: &ModelData) -> LogLinearModel {
//         LogLinearModel {
//             params: log_linear_fn(&data.get_all_x(), &data.get_all_y()),
//         }
//     }
// }

// impl Model for LogLinearModel {
//     fn name(&self) -> String {
//         String::from("loglinear")
//     }
//     fn predict(&self, x: f64) -> f64 {
//         let (coefficient, intercept) = self.params;
//         f64::exp(coefficient * x + intercept)
//     }

//     fn predict_list(&self, x_values: &Vec<f64>) -> Vec<f64> {
//         (0..x_values.len())
//             .map(|i| self.predict(x_values[i]))
//             .collect()
//     }
//     fn evaluate(&self, x_test: &Vec<f64>, y_test: &Vec<f64>) -> f64 {
//         let y_predicted = self.predict_list(x_test);
//         return root_mean_squared_error(y_test, &y_predicted);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn should_panic_for_empty_vecs() {
        let x_values = vec![];
        let y_values = vec![];
        let model: LinearModel<f64> = LinearModel::fit(&x_values, &y_values);

        assert_delta!(0.8f64, model.coefficient, 0.00001);
        assert_delta!(0.4, model.intercept, 0.00001);
    }

    #[test]
    fn should_fit_coefficients_correctly() {
        let x_values = vec![1f64, 2f64, 3f64, 4f64, 5f64];
        let y_values = vec![1f64, 3f64, 2f64, 3f64, 5f64];
        let model: LinearModel<f64> = LinearModel::fit(&x_values, &y_values);

        assert_delta!(0.8f64, model.coefficient, 0.00001);
        assert_delta!(0.4f64, model.intercept, 0.00001);
    }

    #[test]
    fn should_predict_correctly() {
        let x_values = vec![1f64, 2f64, 3f64, 4f64, 5f64];
        let y_values = vec![1f64, 3f64, 2f64, 3f64, 5f64];
        let model: LinearModel<f64> = LinearModel::fit(&x_values, &y_values);

        assert_delta!(1.2f64, model.predict(1f64), 0.00001);
        assert_delta!(2f64, model.predict(2f64), 0.00001);
        assert_delta!(2.8f64, model.predict(3f64), 0.00001);
        assert_delta!(3.6f64, model.predict(4f64), 0.00001);
        assert_delta!(4.4f64, model.predict(5f64), 0.00001);
    }

    #[test]
    fn should_predict_list_correctly() {
        let x_values = vec![1f64, 2f64, 3f64, 4f64, 5f64];
        let y_values = vec![1f64, 3f64, 2f64, 3f64, 5f64];
        let model: LinearModel<f64> = LinearModel::fit(&x_values.clone(), &y_values);

        let predictions = model.batch_predict(&x_values);

        assert_delta!(1.2f64, predictions[0], 0.00001);
        assert_delta!(2f64, predictions[1], 0.00001);
        assert_delta!(2.8f64, predictions[2], 0.00001);
        assert_delta!(3.6f64, predictions[3], 0.00001);
        assert_delta!(4.4f64, predictions[4], 0.00001);
    }

    #[test]
    fn should_evaluate_correctly() {
        let x_values = vec![1f64, 2f64, 3f64, 4f64, 5f64];
        let y_values = vec![1f64, 3f64, 2f64, 3f64, 5f64];
        let model: LinearModel<f64> = LinearModel::fit(&x_values.clone(), &y_values.clone());

        let error = model.evaluate(&x_values, &y_values);
        assert_delta!(0.69282f64, error, 0.00001);
    }

    // #[test]
    // fn transform_log_test() {
    //     let x_values = vec![1f64, 2f64, 3f64, 4f64, 5f64];
    //     let y_values = vec![1f64, 3f64, 2f64, 4f64, 5f64];
    //     let (w, b) = log_linear_fn(&x_values, &y_values);

    //     assert_delta!(0.35065, w, 0.00001);
    //     assert_delta!(-0.09446, b, 0.00001);
    // }
}
