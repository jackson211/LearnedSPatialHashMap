use crate::geometry::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub type Seed = [u8; 32];

pub const SEED_1: &Seed = b"wPYxAkIiHcEmSBAxQFoXFrpYToCe1B71";
pub const SEED_2: &Seed = b"4KbTVjPT4DXSwWAsQM5dkWWywPKZRfCX";

pub fn create_random_points(num_points: usize, seed: &[u8; 32]) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(num_points);
    let mut rng = SmallRng::from_seed(*seed);
    for _ in 0..num_points {
        result.push((rng.random(), rng.random()));
    }
    result
}

pub fn create_random_point_type_points(num_points: usize, seed: &[u8; 32]) -> Vec<Point<f64>> {
    let result = create_random_points(num_points, seed);

    // result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    result
        .into_iter()
        .map(|(x, y)| Point { x, y })
        .collect::<Vec<_>>()
}
