#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $delta:expr) => {
        if (f64::abs($x - $y) > $delta) {
            panic!("{} != {}", $x, $y);
        }
    };
}

extern crate num_traits;
mod error;
mod hasher;
pub mod linear;
pub mod map3;
mod model;
mod stats;
