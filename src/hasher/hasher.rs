use crate::models::Model;
use num_traits::cast::{AsPrimitive, FromPrimitive};
use num_traits::float::Float;

/// LearnedHasher takes a model and produces hash from the model
#[derive(Debug, Clone)]
pub struct LearnedHasher<M> {
    state: u64,
    pub model: M,
    sort_by_x: bool,
}

impl<M, F> Default for LearnedHasher<M>
where
    F: Float,
    M: Model<F = F> + Default,
{
    fn default() -> Self {
        Self {
            state: 0,
            model: Default::default(),
            sort_by_x: true,
        }
    }
}
impl<M, F> LearnedHasher<M>
where
    F: Float,
    M: Model<F = F>,
{
    pub fn with_model(model: M) -> Self {
        Self {
            state: 0,
            model,
            sort_by_x: true,
        }
    }
    fn finish(&self) -> u64 {
        self.state
    }

    pub fn sort_by_x(&self) -> bool {
        self.sort_by_x
    }

    pub fn set_sort_by_x(&mut self, x: bool) {
        self.sort_by_x = x;
    }
}

impl<M, F> LearnedHasher<M>
where
    F: Float,
    M: Model<F = F> + Default,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<M, F> LearnedHasher<M>
where
    F: Float + AsPrimitive<u64>,
    M: Model<F = F>,
{
    fn write(&mut self, data: &F) {
        self.state = self.model.predict(*data).floor().as_();
    }
}

impl<M, F> LearnedHasher<M>
where
    F: Float + FromPrimitive,
    M: Model<F = F> + Default,
{
    fn unwrite(&mut self, hash: u64) -> F {
        let hash = FromPrimitive::from_u64(hash).unwrap();
        self.model.unpredict(hash)
    }
}

pub fn make_hash<M, F>(hasher: &mut LearnedHasher<M>, p: &F) -> u64
where
    F: Float + FromPrimitive + AsPrimitive<u64>,
    M: Model<F = F> + Default,
{
    hasher.write(p);
    hasher.finish()
}

pub fn make_hash_point<M, F>(hasher: &mut LearnedHasher<M>, p: &(F, F)) -> u64
where
    F: Float + FromPrimitive + AsPrimitive<u64>,
    M: Model<F = F> + Default,
{
    if hasher.sort_by_x {
        make_hash(hasher, &p.0)
    } else {
        make_hash(hasher, &p.1)
    }
}

/// Reverse the hash function, which it takes a hash and returns float
pub fn unhash<M, F>(hasher: &mut LearnedHasher<M>, hash: u64) -> F
where
    F: Float + FromPrimitive + AsPrimitive<u64>,
    M: Model<F = F> + Default,
{
    hasher.unwrite(hash)
}

#[cfg(test)]
mod tests {
    use super::LearnedHasher;
    use crate::models::LinearModel;

    #[test]
    fn hasher_with_empty_model() {
        let mut hasher: LearnedHasher<LinearModel<f64>> = LearnedHasher::new();
        hasher.write(&10f64);
        assert_eq!(0u64, hasher.finish());
    }

    #[test]
    fn unhash() {
        let mut hasher: LearnedHasher<LinearModel<f64>> = LearnedHasher::with_model(LinearModel {
            coefficient: 3.,
            intercept: 2.,
        });
        hasher.write(&10.5);
        assert_eq!(33u64, hasher.finish());
        assert_delta!(10.33f64, hasher.unwrite(33u64), 0.01);
    }
}