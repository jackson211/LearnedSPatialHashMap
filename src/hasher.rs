use crate::algorithm::{LinearModel, Model};
use num_traits::cast::AsPrimitive;
use std::hash::{BuildHasher, Hasher};

/// LearnedHasher takes a model and produces hash from the model
#[derive(Default, Debug, Clone)]
pub struct LearnedHasher<M: Model> {
    state: u64,
    pub model: M,
    pub sort_by_x: bool,
}

impl<M> LearnedHasher<M>
where
    M: Model + Default,
{
    pub fn new() -> LearnedHasher<M> {
        LearnedHasher {
            state: 0u64,
            model: Default::default(),
            sort_by_x: true,
        }
    }
}

impl<M> Hasher for LearnedHasher<M>
where
    M: Model,
{
    fn write(&mut self, bytes: &[u8]) {
        let length = bytes.len();
        let mut val: [u8; 4] = Default::default();
        match length {
            4 => {
                val.copy_from_slice(&bytes[0..4]);
                self.state = self.model.predict_bytes(&val).floor().as_();
            }
            // TODO: adding cases when the length is not 8
            _ => (),
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

/// Default model for LearnedHasher is LinearModel with f64
type DefaultModel = LinearModel;

/// Provides a LearnedHasher factory
///
#[derive(Default, Debug, Clone)]
pub struct LearnedHashbuilder<M>
where
    M: Model,
{
    pub model: M,
}

impl<M> LearnedHashbuilder<M>
where
    M: Model + Default,
{
    pub fn new() -> LearnedHashbuilder<M> {
        LearnedHashbuilder {
            model: Default::default(),
        }
    }
}

impl<M> BuildHasher for LearnedHashbuilder<M>
where
    M: Model,
{
    type Hasher = LearnedHasher<DefaultModel>;
    #[inline]
    fn build_hasher(&self) -> LearnedHasher<DefaultModel> {
        LearnedHasher::<LinearModel>::new()
    }
}

#[cfg(test)]
mod tests {
    use super::LearnedHasher;
    use crate::algorithm::LinearModel;
    use std::hash::Hasher;

    #[test]
    fn hasher_with_empty_model() {
        let mut hasher: LearnedHasher<LinearModel> = LearnedHasher::new();
        hasher.write(&10f64.to_ne_bytes());
        assert_eq!(0u64, hasher.finish());
    }
}
