use num_traits::{cast::FromPrimitive, float::Float};
use rayon::prelude::*;

pub fn mean(values: &Vec<f64>) -> f64 {
    if values.len() == 0 {
        return 0f64;
    }
    values.par_iter().sum::<f64>() / values.len() as f64
}

pub fn variance(values: &Vec<f64>) -> f64 {
    if values.len() == 0 {
        return 0f64;
    }
    let mean = mean(values);
    values
        .par_iter()
        .map(|x| f64::powf(x - mean, 2 as f64))
        .sum::<f64>()
        / values.len() as f64
}

pub fn covariance(x_values: &Vec<f64>, y_values: &Vec<f64>) -> f64 {
    if x_values.len() != y_values.len() {
        panic!("x_values and y_values must be of equal length.");
    }
    let length: usize = x_values.len();
    if length == 0usize {
        return 0f64;
    }
    let mean_x = mean(x_values);
    let mean_y = mean(y_values);

    x_values
        .iter()
        .zip(y_values.iter())
        .fold(0.0, |covariance, (x, y)| {
            covariance + (x - mean_x) * (y - mean_y)
        })
        / length as f64
}

pub fn mean_squared_error<F>(actual: &Vec<F>, predict: &Vec<F>) -> F
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

pub fn root_mean_squared_error<F>(actual: &Vec<F>, predict: &Vec<F>) -> F
where
    F: Float + FromPrimitive,
{
    mean_squared_error::<F>(&actual, &predict).sqrt()
}

#[cfg(test)]
mod tests {
    use crate::algorithm::stats::*;
    // use crate::algorithm::*;

    #[test]
    fn mean_empty_vec() {
        let values = vec![];
        assert_delta!(0f64, mean(&values), 0.00001);
    }

    #[test]
    fn mean_1_to_5() {
        let values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta!(3f64, mean(&values), 0.00001);
    }

    #[test]
    fn variance_empty_vec() {
        let values = vec![];
        assert_delta!(0f64, variance(&values), 0.00001);
    }

    #[test]
    fn variance_1_to_5() {
        let values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta!(2f64, variance(&values), 0.00001);
    }

    #[test]
    fn covariance_empty_vec() {
        let x_values = vec![];
        let y_values = vec![];
        assert_delta!(0f64, covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn covariance_1_to_5() {
        let x_values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y_values: Vec<f64> = vec![1.0, 3.0, 2.0, 3.0, 5.0];
        assert_delta!(1.6f64, covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn negative_covariance_1_to_5() {
        let x_values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y_values: Vec<f64> = vec![0.5, 4.0, 1.0, -5.0, 4.0];
        assert_delta!(-0.4f64, covariance(&x_values, &y_values), 0.00001);
    }

    #[test]
    fn mean_squared_error_test() {
        let actual: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predict: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_delta!(0f64, mean_squared_error(&actual, &predict), 0.00001);
    }

    #[test]
    fn mean_squared_error_test_2() {
        let actual: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predict: Vec<f64> = vec![1.0, 1.6, 3.0, 4.0, 5.0];
        assert_delta!(0.032f64, mean_squared_error(&actual, &predict), 0.00001);
    }
}