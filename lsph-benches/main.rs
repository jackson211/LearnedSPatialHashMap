use lsph::algorithm::map;

fn create_random_points(num_points: usize, seed: &[u8; 32]) -> Vec<[f64; 2]> {
    let mut result = Vec::with_capacity(num_points);
    let mut rng = Hc128Rng::from_seed(*seed);
    for _ in 0..num_points {
        result.push(rng.gen());
    }
    result
}

fn main() {}
