use crate::error::Error;
use core::fmt::Debug;

pub trait Model {
    fn name(&self) -> String;
    fn fit(&mut self, xs: &[f32], ys: &[f32]) -> Result<(), Error>;
    fn fit_tuple(&mut self, xys: &[(f32, f32)]) -> Result<(), Error>;
    fn predict(&self, x: f32) -> f32;
    fn predict_bytes(&self, bytes: &[u8; 4]) -> f32;
    fn batch_predict(&self, xs: &[f32]) -> Vec<f32>;
    fn evaluate(&self, x_test: &[f32], y_test: &[f32]) -> f32;
}

impl Debug for (dyn Model + 'static) {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Model {{{}}}", self.name())
    }
}
