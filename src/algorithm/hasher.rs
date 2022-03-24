use crate::algorithm::model::Model;

use num_traits::{
    cast::{AsPrimitive, FromPrimitive},
    float::Float,
};

/// Hasher
#[derive(Default)]
pub struct LearnedHasher<M: Model + Default> {
    state: u64,
    pub model: M,
    pub sort_by_lat: bool,
}

impl<M, F> LearnedHasher<M>
where
    F: Float + FromPrimitive + AsPrimitive<u64>,
    M: Model<F = F> + Default,
{
    pub fn new() -> LearnedHasher<M> {
        LearnedHasher {
            state: 0u64,
            model: Default::default(),
            sort_by_lat: true,
        }
    }

    fn write(&mut self, data: &(F, F)) {
        if self.sort_by_lat {
            self.state = self.model.predict(data.0).round().as_();
        } else {
            self.state = self.model.predict(data.1).round().as_();
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

pub fn make_hash<M, F>(hasher: &mut LearnedHasher<M>, p: &(F, F)) -> u64
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
    use crate::algorithm::linear::LinearModel;

    #[test]
    fn hasher_with_empty_model() {
        let mut hasher: LearnedHasher<LinearModel<f64>> = LearnedHasher::new();
        hasher.write(&(10f64, 10f64));
        assert_eq!(0u64, hasher.finish());
    }
}
