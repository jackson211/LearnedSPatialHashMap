use num_traits::{cast::FromPrimitive, float::Float};
use rayon::prelude::*;

pub fn mean(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0f32;
    }
    values.par_iter().sum::<f32>() / values.len() as f32
}

pub fn variance(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0f32;
    }
    let mean = mean(values);
    values
        .par_iter()
        .map(|x| f32::powf(x - mean, 2f32))
        .sum::<f32>()
        / values.len() as f32
}

pub fn covariance(x_values: &[f32], y_values: &[f32]) -> f32 {
    if x_values.len() != y_values.len() {
        panic!("x_values and y_values must be of equal length.");
    }
    let length: usize = x_values.len();
    if length == 0usize {
        return 0f32;
    }
    let mean_x = mean(x_values);
    let mean_y = mean(y_values);

    x_values
        .iter()
        .zip(y_values.iter())
        .fold(0.0, |covariance, (x, y)| {
            covariance + (x - mean_x) * (y - mean_y)
        })
        / length as f32
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
        .fold(F::from_f32(0.0).unwrap(), |sum, (&x, &y)| {
            sum + (x - y).powf(F::from_f32(2.0).unwrap())
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
        let values = vec![];
        assert_delta!(0., mean(&values), 0.00001);
    }

    #[test]
    fn mean_1_to_5() {
        let values: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta!(3., mean(&values), 0.00001);
    }

    #[test]
    fn variance_empty_vec() {
        let values = vec![];
        assert_delta!(0., variance(&values), 0.00001);
    }

    #[test]
    fn variance_1_to_5() {
        let values: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta!(2., variance(&values), 0.00001);
    }

    #[test]
    fn covariance_empty_vec() {
        let x_values = vec![];
        let y_values = vec![];
        assert_delta!(0., covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn covariance_1_to_5() {
        let x_values: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y_values: Vec<f32> = vec![1.0, 3.0, 2.0, 3.0, 5.0];
        assert_delta!(1.6, covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn negative_covariance_1_to_5() {
        let x_values: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y_values: Vec<f32> = vec![0.5, 4.0, 1.0, -5.0, 4.0];
        assert_delta!(-0.4, covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn mean_squared_error_test() {
        let actual: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predict: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta!(0., mean_squared_error(&actual, &predict), 0.00001);
    }

    #[test]
    fn mean_squared_error_test_2() {
        let actual: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predict: Vec<f32> = vec![1.0, 1.6, 3.0, 4.0, 5.0];
        assert_delta!(0.032, mean_squared_error(&actual, &predict), 0.00001);
    }
}
