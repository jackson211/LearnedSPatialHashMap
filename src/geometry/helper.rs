use crate::geometry::Point;
use num_traits::{cast::FromPrimitive, float::Float};

pub fn extract_x<F: Float>(ps: &[Point<F>]) -> Vec<F> {
    ps.iter().map(|p| p.x).collect()
}

pub fn extract_y<F: Float>(ps: &[Point<F>]) -> Vec<F> {
    ps.iter().map(|p| p.y).collect()
}
pub fn extract_id<F: Float + FromPrimitive>(ps: &[Point<F>]) -> Vec<F> {
    ps.iter().map(|p| F::from_usize(p.id).unwrap()).collect()
}

pub fn sort_by_x<F: Float>(ps: &mut [Point<F>]) {
    ps.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
}

pub fn sort_by_y<F: Float>(ps: &mut [Point<F>]) {
    ps.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
}

pub fn reset_id<F: Float>(ps: &mut [Point<F>]) {
    ps.iter_mut().enumerate().for_each(|(i, p)| p.id = i);
}
