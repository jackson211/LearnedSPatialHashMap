use crate::geometry::Point;
use num_traits::{cast::FromPrimitive, float::Float};

/// Extract all the x values from a Vec<Point<F>>
pub fn extract_x<F: Float>(ps: &[Point<F>]) -> Vec<F> {
    ps.iter().map(|p| p.x).collect()
}

/// Extract all the y values from a Vec<Point<F>>
pub fn extract_y<F: Float>(ps: &[Point<F>]) -> Vec<F> {
    ps.iter().map(|p| p.y).collect()
}

/// Extract all the id values from a Vec<Point<F>>
pub fn extract_id<F: Float + FromPrimitive>(ps: &[Point<F>]) -> Vec<F> {
    ps.iter().map(|p| F::from_usize(p.id).unwrap()).collect()
}

/// Sort a Vec<Point<F>> based on the x values
pub fn sort_by_x<F: Float>(ps: &mut [Point<F>]) {
    ps.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
}

/// Sort a Vec<Point<F>> based on the y values
pub fn sort_by_y<F: Float>(ps: &mut [Point<F>]) {
    ps.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
}

/// Reset the id for given Vec<Point<F>> based current order
pub fn reset_id<F: Float>(ps: &mut [Point<F>]) {
    ps.iter_mut().enumerate().for_each(|(i, p)| p.id = i);
}
