#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $delta:expr) => {
        if (f32::abs($x - $y) > $delta) {
            panic!("{} != {}", $x, $y);
        }
    };
}

pub mod algorithm;
mod error;
pub mod map;
pub mod primitives;
