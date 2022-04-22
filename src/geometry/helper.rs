use crate::geometry::Point;
use num_traits::float::Float;

/// Extract all the x values from a Vec<Point<F>>
pub fn extract_x<F: Float>(ps: &[Point<F>]) -> Vec<F> {
    ps.iter().map(|p| p.x).collect()
}

/// Extract all the y values from a Vec<Point<F>>
pub fn extract_y<F: Float>(ps: &[Point<F>]) -> Vec<F> {
    ps.iter().map(|p| p.y).collect()
}

/// Sort a Vec<Point<F>> based on the x values
pub fn sort_by_x<F: Float>(ps: &mut [Point<F>]) {
    ps.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
}

/// Sort a Vec<Point<F>> based on the y values
pub fn sort_by_y<F: Float>(ps: &mut [Point<F>]) {
    ps.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
}

/// Convert a Vec of [F; 2] to a Vec<Point<F>>
pub fn convert_to_points<F: Float>(ps: &[[F; 2]]) -> Option<Vec<Point<F>>> {
    Some(ps.iter().map(|p| Point::new(p[0], p[1])).collect())
}
