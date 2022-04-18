mod linear;
mod stats;
mod trainer;

pub use linear::*;
pub use stats::*;
pub use trainer::*;

use crate::error::Error;
use core::fmt::Debug;
use num_traits::float::Float;

/// Model representation, provides common functionalities for model training
pub trait Model {
    /// Associated type for float number representation
    type F;
    /// Prints the name of the model
    fn name(&self) -> String;
    /// Fit two slices of training data into the model
    fn fit(&mut self, xs: &[Self::F], ys: &[Self::F]) -> Result<(), Error>;
    /// Fit one slice of training data in tuple format into the model
    fn fit_tuple(&mut self, xys: &[(Self::F, Self::F)]) -> Result<(), Error>;
    /// Takes one value and returns the predictions of the model
    fn predict(&self, x: Self::F) -> Self::F;
    /// Takes slice of value and returns the batch predictions from the model
    fn batch_predict(&self, xs: &[Self::F]) -> Vec<Self::F>;
    /// Evaluate the predictions results from a pair of test sets
    fn evaluate(&self, x_test: &[Self::F], y_test: &[Self::F]) -> Self::F;
    /// Unpredict provides the ability of reversing the predict operation
    /// For a given target value, return the estimate input value
    fn unpredict(&self, y: Self::F) -> Self::F;
}

impl<F> Debug for (dyn Model<F = F> + 'static)
where
    F: Float,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Model {{{}}}", self.name())
    }
}
