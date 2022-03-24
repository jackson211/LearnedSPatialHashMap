use crate::algorithm::error::Error;

pub trait Model {
    type F;
    fn name(&self) -> String;
    fn fit(&mut self, xs: &[Self::F], ys: &[Self::F]) -> Result<(), Error>;
    fn fit_tuple(&mut self, xys: &[(Self::F, Self::F)]) -> Result<(), Error>;
    fn predict(&self, x: Self::F) -> Self::F;
    fn batch_predict(&self, xs: &Vec<Self::F>) -> Vec<Self::F>;
    fn evaluate(&self, x_test: &Vec<Self::F>, y_test: &Vec<Self::F>) -> Self::F;
}
