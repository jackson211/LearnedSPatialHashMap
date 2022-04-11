use crate::error::Error;
use core::fmt::Debug;
use num_traits::float::Float;

pub trait Model {
    type F;
    fn name(&self) -> String;
    fn fit(&mut self, xs: &[Self::F], ys: &[Self::F]) -> Result<(), Error>;
    fn fit_tuple(&mut self, xys: &[(Self::F, Self::F)]) -> Result<(), Error>;
    fn predict(&self, x: Self::F) -> Self::F;
    fn batch_predict(&self, xs: &[Self::F]) -> Vec<Self::F>;
    fn evaluate(&self, x_test: &[Self::F], y_test: &[Self::F]) -> Self::F;
}

impl<F> Debug for (dyn Model<F = F> + 'static)
where
    F: Float,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Model {{{}}}", self.name())
    }
}
