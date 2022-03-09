use core::hash::{BuildHasher, Hasher};
pub struct LearnedHasher {
    state: u64,
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

pub struct BuildLearnedHasher;

impl BuildHasher for BuildLearnedHasher {
    type Hasher = LearnedHasher;
    fn build_hasher(&self) -> LearnedHasher {
        LearnedHasher { state: 0 }
    }
}
