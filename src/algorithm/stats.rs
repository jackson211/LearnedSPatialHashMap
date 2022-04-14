use core::iter::Sum;
use num_traits::{cast::FromPrimitive, float::Float};

pub fn mean<F>(values: &[F]) -> F
where
    F: Float + Sum,
{
    if values.is_empty() {
        return F::zero();
    }
    let sum: F = values.iter().cloned().map(Into::into).sum();
    sum / F::from(values.len()).unwrap()
}

pub fn variance<F>(values: &[F]) -> F
where
    F: Float + Sum,
{
    if values.is_empty() {
        return F::zero();
    }
    let mean = mean(values);

    let diff_sum: F = values
        .iter()
        .cloned()
        .map(|x| (x - mean).powf(F::from(2.0).unwrap()))
        .sum();
    diff_sum / F::from(values.len()).unwrap()
}

pub fn covariance<F>(x_values: &[F], y_values: &[F]) -> F
where
    F: Float + Sum,
{
    if x_values.len() != y_values.len() {
        panic!("x_values and y_values must be of equal length.");
    }
    let length: usize = x_values.len();
    if length == 0usize {
        return F::zero();
    }
    let mean_x = mean(x_values);
    let mean_y = mean(y_values);

    x_values
        .iter()
        .zip(y_values.iter())
        .fold(F::zero(), |covariance, (&x, &y)| {
            covariance + (x - mean_x) * (y - mean_y)
        })
        / F::from(length).unwrap()
}

pub fn mean_squared_error<F>(actual: &[F], predict: &[F]) -> F
where
    F: Float + FromPrimitive,
{
    if actual.len() != predict.len() {
        panic!("actual and predict must be of equal length.");
    }

    actual
        .iter()
        .zip(predict.iter())
        .fold(F::from_f64(0.0).unwrap(), |sum, (&x, &y)| {
            sum + (x - y).powf(F::from_f64(2.0).unwrap())
        })
        / F::from_usize(actual.len()).unwrap()
}

pub fn root_mean_squared_error<F>(actual: &[F], predict: &[F]) -> F
where
    F: Float + FromPrimitive,
{
    mean_squared_error::<F>(actual, predict).sqrt()
}

#[cfg(test)]
mod tests {
    use crate::algorithm::stats::*;
    // use crate::algorithm::*;

    #[test]
    fn mean_empty_vec() {
        let values: Vec<f64> = vec![];
        assert_delta!(0., mean(&values), 0.00001);
    }

    #[test]
    fn mean_empty_vec_f32() {
        let values: Vec<f32> = vec![];
        assert_delta_f32!(0., mean(&values), 0.00001);
    }

    #[test]
    fn mean_1_to_5() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta!(3., mean(&values), 0.00001);
    }

    #[test]
    fn mean_1_to_5_f32() {
        let values: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta_f32!(3., mean(&values), 0.00001);
    }

    #[test]
    fn variance_empty_vec() {
        let values: Vec<f64> = vec![];
        assert_delta!(0., variance(&values), 0.00001);
    }

    #[test]
    fn variance_empty_vec_f32() {
        let values: Vec<f32> = vec![];
        assert_delta_f32!(0., variance(&values), 0.00001);
    }

    #[test]
    fn variance_1_to_5() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta!(2., variance(&values), 0.00001);
    }

    #[test]
    fn variance_1_to_5_f32() {
        let values: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta_f32!(2., variance(&values), 0.00001);
    }

    #[test]
    fn covariance_empty_vec() {
        let x_values: Vec<f64> = vec![];
        let y_values: Vec<f64> = vec![];
        assert_delta!(0., covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn covariance_empty_vec_f32() {
        let x_values: Vec<f32> = vec![];
        let y_values: Vec<f32> = vec![];
        assert_delta_f32!(0., covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn covariance_1_to_5() {
        let x_values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y_values = vec![1.0, 3.0, 2.0, 3.0, 5.0];
        assert_delta!(1.6, covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn covariance_1_to_5_f32() {
        let x_values: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y_values: Vec<f32> = vec![1.0, 3.0, 2.0, 3.0, 5.0];
        assert_delta_f32!(1.6, covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn negative_covariance_1_to_5() {
        let x_values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y_values = vec![0.5, 4.0, 1.0, -5.0, 4.0];
        assert_delta!(-0.4, covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn negative_covariance_1_to_5_f32() {
        let x_values: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y_values: Vec<f32> = vec![0.5, 4.0, 1.0, -5.0, 4.0];
        assert_delta_f32!(-0.4, covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn mean_squared_error_test() {
        let actual = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predict = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta!(0., mean_squared_error(&actual, &predict), 0.00001);
    }

    #[test]
    fn mean_squared_error_test_f32() {
        let actual: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predict: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta_f32!(0., mean_squared_error(&actual, &predict), 0.00001);
    }

    #[test]
    fn mean_squared_error_test_2() {
        let actual = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predict = vec![1.0, 1.6, 3.0, 4.0, 5.0];
        assert_delta!(0.032, mean_squared_error(&actual, &predict), 0.00001);
    }

    #[test]
    fn mean_squared_error_test_2_f32() {
        let actual: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predict: Vec<f32> = vec![1.0, 1.6, 3.0, 4.0, 5.0];
        assert_delta_f32!(0.032, mean_squared_error(&actual, &predict), 0.00001);
    }
}
