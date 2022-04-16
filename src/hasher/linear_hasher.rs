pub struct LinearHasher {
    coefficient: f64,
    intercept: f64,
    state: u64,
}
impl LinearHasher {
    pub fn new_with_params(coefficient: f64, intercept: f64) -> LinearHasher {
        LinearHasher {
            coefficient,
            intercept,
            state: 0u64,
        }
    }

    #[inline(always)]
    fn hash_in(&mut self, new_value: f64) {
        dbg!(new_value);
        self.state = (self.coefficient.mul_add(new_value, self.intercept)) as u64;
    }

    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, value: f64) {
        self.hash_in(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let mut hasher = LinearHasher::new_with_params(3., 2.);
        hasher.write(3f64);
        assert_eq!(hasher.finish(), 11u64);
    }
}
