extern crate num_traits;
// Hepler macro for assert Float values within a delta range, panic if the
// difference between two numbers is exceeds the given threshold.
#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $delta:expr) => {
        assert!(
            ($x - $y).abs() <= num_traits::NumCast::from($delta).unwrap(),
            "{} is not within {} of {}",
            stringify!($x),
            $delta,
            stringify!($y)
        );
    };
}

#[macro_export]
macro_rules! assert_eq_len {
    ($a:expr, $b:expr) => {
        if $a.len() != $b.len() {
            return Err(Error::DiffLen);
        }
    };
}

#[macro_export]
macro_rules! assert_empty {
    ($a:expr) => {
        if $a.is_empty() {
            return Err(Error::EmptyVal);
        }
    };
}
