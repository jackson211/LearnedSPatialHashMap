use crate::geometry::*;
use rand::{Rng, SeedableRng};
use rand_hc::Hc128Rng;

pub type Seed = [u8; 32];

pub const SEED_1: &Seed = b"wPYxAkIiHcEmSBAxQFoXFrpYToCe1B71";
pub const SEED_2: &Seed = b"4KbTVjPT4DXSwWAsQM5dkWWywPKZRfCX";

pub fn create_random_point_type_points(num_points: usize, seed: &Seed) -> Vec<Point<f64>> {
    let mut result: Vec<(f64, f64)> = Vec::with_capacity(num_points);
    let mut rng = Hc128Rng::from_seed(*seed);
    for _ in 0..num_points {
        let x = rng.gen();
        let y = rng.gen();
        result.push((x, y));
    }

    result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    result
        .into_iter()
        .enumerate()
        .map(|(id, (x, y))| Point { id, x, y })
        .collect::<Vec<_>>()
}
