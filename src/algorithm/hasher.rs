use core::hash::{BuildHasher, Hasher};

pub struct LearnedHasher {
    state: u64,
}

impl LearnedHasher {
    pub fn new() -> LearnedHasher {
        LearnedHasher { state: 0u64 }
    }
}

impl Default for LearnedHasher {
    /// Creates a new `LearnedHasher` using [`new`].
    ///
    /// [`new`]: LearnedHasher::new
    fn default() -> LearnedHasher {
        LearnedHasher::new()
    }
}

impl Hasher for LearnedHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.rotate_left(8) ^ u64::from(byte);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

#[derive(Debug)]
pub struct LearnedHasherBuilder {
    param: (f64, f64),
}

impl LearnedHasherBuilder {
    pub fn new() -> LearnedHasherBuilder {
        LearnedHasherBuilder {
            param: (0f64, 0f64),
        }
    }
}

impl Default for LearnedHasherBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BuildHasher for LearnedHasherBuilder {
    type Hasher = LearnedHasher;
    #[inline]
    fn build_hasher(&self) -> LearnedHasher {
        LearnedHasher::new()
    }
}

#[cfg(test)]
mod tests {
    use super::LearnedHasher;
    use std::hash::Hasher;

    #[test]
    fn hasher() {
        let mut hasher = LearnedHasher::new();
        hasher.write(&10f64.to_ne_bytes());
        assert_eq!(2440u64, hasher.finish());
    }
}
