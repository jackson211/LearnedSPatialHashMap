use crate::models::*;

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
        LinearModel {
            params: linear_fn(&data.get_all_x(), &data.get_all_y()),
        }
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
        assert_delta!(0.693, error, 0.00001);
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
