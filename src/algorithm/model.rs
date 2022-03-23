pub trait Model {
    type F;
    fn name(&self) -> String;
    fn fit(&self, xs: &[Self::F], ys: &[Self::F]) -> Self;
    fn fit_tuple(&self, xys: &[(Self::F, Self::F)]) -> Self;
    fn predict(&self, x: Self::F) -> Self::F;
    fn batch_predict(&self, xs: &Vec<Self::F>) -> Vec<Self::F>;
    fn evaluate(&self, x_test: &Vec<Self::F>, y_test: &Vec<Self::F>) -> Self::F;
}
