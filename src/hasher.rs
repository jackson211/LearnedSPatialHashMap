use crate::algorithm::Model;
use num_traits::cast::{AsPrimitive, FromPrimitive};
use num_traits::float::Float;

/// LearnedHasher takes a model and produces hash from the model
#[derive(Debug, Clone)]
pub struct LearnedHasher<M, F>
where
    F: Float,
    M: Model<F = F>,
{
    state: u64,
    pub model: M,
    pub sort_by_lat: bool,
}

impl<M, F> Default for LearnedHasher<M, F>
where
    F: Float,
    M: Model<F = F> + Default,
{
    fn default() -> Self {
        Self {
            state: 0,
            model: Default::default(),
            sort_by_lat: true,
        }
    }
}

impl<M, F> LearnedHasher<M, F>
where
    F: Float + FromPrimitive + AsPrimitive<u64>,
    M: Model<F = F> + Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    fn write(&mut self, data: &F) {
        self.state = self.model.predict(*data).floor().as_();
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

pub fn make_hash<M, F>(hasher: &mut LearnedHasher<M, F>, p: &F) -> u64
where
    F: Float + FromPrimitive + AsPrimitive<u64>,
    M: Model<F = F> + Default,
{
    hasher.write(p);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::LearnedHasher;
    use crate::algorithm::LinearModel;

    #[test]
    fn hasher_with_empty_model() {
        let mut hasher: LearnedHasher<LinearModel<f64>, f64> = LearnedHasher::new();
        hasher.write(&10f64);
        assert_eq!(0u64, hasher.finish());
    }
}
