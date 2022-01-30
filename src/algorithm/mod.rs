#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $delta:expr) => {
        if (f64::abs($x - $y) > $delta) {
            panic!("{} != {}", $x, $y);
        }
    };
}

pub mod linear;
pub mod linkedlist;
pub mod map;
mod stats;
// mod linear;

// use crate::utils::stats::*;
// pub use linear::LinearModel;
// pub use linear::LogLinearModel;
// use log::*;
//
pub trait Model {
    fn name(&self) -> String;
    fn predict(&self, x: f64) -> f64;
    fn predict_list(&self, x_values: &Vec<f64>) -> Vec<f64>;
    fn evaluate(&self, x_test: &Vec<f64>, y_test: &Vec<f64>) -> f64;
}

#[derive(Clone, Debug)]
pub struct ModelData {
    // x_values: Vec<f64>,
    // y_values: Vec<f64>,
    idx: usize,
    data: Vec<(f64, f64)>,
    scaling_factor: f64,
}

impl ModelData {
    pub fn new(x_values: Vec<f64>, y_values: Vec<f64>) -> ModelData {
        if x_values.len() != y_values.len() {
            panic!(
                "lengths are not matched for x_values and y_values: x_values {}, y_values {}",
                x_values.len(),
                y_values.len()
            );
        }
        let mut data = x_values
            .into_iter()
            .zip(y_values.into_iter())
            .collect::<Vec<(f64, f64)>>();

        data.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        ModelData {
            data: data,
            idx: 0usize,
            scaling_factor: 1f64,
        }
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scaling_factor = scale;
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn get(&self, idx: usize) -> (f64, f64) {
        self.data[idx]
    }

    pub fn get_x(&self, idx: usize) -> f64 {
        self.data[idx].0
    }

    pub fn get_y(&self, idx: usize) -> f64 {
        self.data[idx].1
    }

    pub fn get_range(&self, start: usize, end: usize) -> ModelData {
        let mut range_x_value: Vec<f64> = vec![];
        let mut range_y_value: Vec<f64> = vec![];
        for i in start..end {
            let (x, y) = self.get(i);
            range_x_value.push(x);
            range_y_value.push(y);
        }
        let range_data = range_x_value
            .into_iter()
            .zip(range_y_value.into_iter())
            .collect::<Vec<(f64, f64)>>();

        ModelData {
            data: range_data,
            idx: self.idx,
            scaling_factor: self.scaling_factor,
        }
    }

    pub fn get_all_x(&self) -> Vec<f64> {
        self.data.iter().map(|(x, _)| *x).collect::<Vec<f64>>()
    }

    pub fn get_all_y(&self) -> Vec<f64> {
        self.data
            .iter()
            .map(|(_, y)| *y * self.scaling_factor)
            .collect::<Vec<f64>>()
    }
}

impl Iterator for ModelData {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len() {
            return None;
        }
        let tuple = self.get(self.idx);
        self.idx += 1usize;
        return Some(tuple);
    }
}
