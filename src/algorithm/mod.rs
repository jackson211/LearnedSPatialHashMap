#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $delta:expr) => {
        if (f64::abs($x - $y) > $delta) {
            panic!("{} != {}", $x, $y);
        }
    };
}

extern crate num_traits;
pub mod hasher;
mod linear;
// pub mod map;
// pub mod map2;
mod error;
pub mod map3;
mod model;
mod stats;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T> {
    id: usize,
    value: (T, T),
}
