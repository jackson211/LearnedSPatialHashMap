use crate::algorithm::stats::*;
use crate::algorithm::*;

fn slr<T: Iterator<Item = (f64, f64)>>(loc_data: T) -> (f64, f64) {
    // compute the covariance of x and y as well as the variance of x in
    // a single pass.

    let mut mean_x = 0.0;
    let mut mean_y = 0.0;
    let mut c = 0.0;
    let mut n: u64 = 0;
    let mut m2 = 0.0;

    let mut data_size = 0;
    for (x, y) in loc_data {
        n += 1;
        let dx = x - mean_x;
        mean_x += dx / (n as f64);
        mean_y += (y - mean_y) / (n as f64);
        c += dx * (y - mean_y);

        let dx2 = x - mean_x;
        m2 += dx * dx2;
        data_size += 1;
    }

    // special case when we have 0 or 1 items
    if data_size == 0 {
        return (0.0, 0.0);
    }

    if data_size == 1 {
        return (mean_y, 0.0);
    }

    let cov = c / ((n - 1) as f64);
    let var = m2 / ((n - 1) as f64);
    assert!(
        var >= 0.0,
        "variance of model with {} data items was negative",
        n
    );

    if var == 0.0 {
        // variance is zero. pick the mean (only) value.
        return (mean_y, 0.0);
    }

    let beta: f64 = cov / var;
    let alpha = mean_y - beta * mean_x;

    return (beta, alpha);
}

fn linear_fn(x_values: &Vec<f64>, y_values: &Vec<f64>) -> (f64, f64) {
    let coefficient: f64 = covariance(x_values, y_values) / variance(x_values);
    let intercept: f64 = mean(y_values) - coefficient * mean(x_values);
    (coefficient, intercept)
}

pub struct LinearModel {
    pub params: (f64, f64),
}

impl LinearModel {
    pub fn new(data: &ModelData) -> LinearModel {
        let params = slr(data.clone().map(|(inp, offset)| (inp, offset as f64)));
        LinearModel { params }
    }
}

impl Model for LinearModel {
    fn name(&self) -> String {
        String::from("linear")
    }

    fn predict(&self, x: f64) -> f64 {
        let (coefficient, intercept) = self.params;
        coefficient * x + intercept
    }

    fn predict_list(&self, x_values: &Vec<f64>) -> Vec<f64> {
        (0..x_values.len())
            .map(|i| self.predict(x_values[i]))
            .collect()
    }

    fn evaluate(&self, x_test: &Vec<f64>, y_test: &Vec<f64>) -> f64 {
        let y_predicted = self.predict_list(x_test);
        return root_mean_squared_error(y_test, &y_predicted);
    }
}

fn log_linear_fn(x_values: &Vec<f64>, y_values: &Vec<f64>) -> (f64, f64) {
    let (x_log, y_log) = x_values
        .iter()
        .zip(y_values.iter())
        .map(|(x, y)| (*x, y.ln()))
        .filter(|(_, y)| y.is_finite())
        .unzip();
    linear_fn(&x_log, &y_log)
}

pub struct LogLinearModel {
    pub params: (f64, f64),
}

impl LogLinearModel {
    pub fn new(data: &ModelData) -> LogLinearModel {
        LogLinearModel {
            params: log_linear_fn(&data.get_all_x(), &data.get_all_y()),
        }
    }
}

impl Model for LogLinearModel {
    fn name(&self) -> String {
        String::from("loglinear")
    }
    fn predict(&self, x: f64) -> f64 {
        let (coefficient, intercept) = self.params;
        f64::exp(coefficient * x + intercept)
    }

    fn predict_list(&self, x_values: &Vec<f64>) -> Vec<f64> {
        (0..x_values.len())
            .map(|i| self.predict(x_values[i]))
            .collect()
    }
    fn evaluate(&self, x_test: &Vec<f64>, y_test: &Vec<f64>) -> f64 {
        let y_predicted = self.predict_list(x_test);
        return root_mean_squared_error(y_test, &y_predicted);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn should_panic_for_empty_vecs() {
        let x_values = vec![];
        let y_values = vec![];
        let data = ModelData::new(x_values, y_values);
        let model = LinearModel::new(&data);

        assert_delta!(0.8f64, model.params.0, 0.00001);
        assert_delta!(0.4, model.params.1, 0.00001);
    }

    #[test]
    fn should_fit_coefficients_correctly() {
        let x_values = vec![1f64, 2f64, 3f64, 4f64, 5f64];
        let y_values = vec![1f64, 3f64, 2f64, 3f64, 5f64];
        let data = ModelData::new(x_values, y_values);
        let model = LinearModel::new(&data);

        assert_delta!(0.8f64, model.params.0, 0.00001);
        assert_delta!(0.4f64, model.params.1, 0.00001);
    }

    #[test]
    fn should_predict_correctly() {
        let x_values = vec![1f64, 2f64, 3f64, 4f64, 5f64];
        let y_values = vec![1f64, 3f64, 2f64, 3f64, 5f64];
        let data = ModelData::new(x_values, y_values);
        let model = LinearModel::new(&data);

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
        let x_values_clone = x_values.clone();
        let data = ModelData::new(x_values, y_values);
        let model = LinearModel::new(&data);

        let predictions = model.predict_list(&x_values_clone);

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
        let x_values_clone = x_values.clone();
        let y_values_clone = y_values.clone();
        let data = ModelData::new(x_values, y_values);
        let model = LinearModel::new(&data);

        let error = model.evaluate(&x_values_clone, &y_values_clone);
        assert_delta!(0.69282f64, error, 0.00001);
    }

    #[test]
    fn transform_log_test() {
        let x_values = vec![1f64, 2f64, 3f64, 4f64, 5f64];
        let y_values = vec![1f64, 3f64, 2f64, 4f64, 5f64];
        let (w, b) = log_linear_fn(&x_values, &y_values);

        assert_delta!(0.35065, w, 0.00001);
        assert_delta!(-0.09446, b, 0.00001);
    }
}
