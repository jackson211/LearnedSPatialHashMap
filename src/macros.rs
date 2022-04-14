// Hepler macro for assert Float values within a delta range, panic if the
// difference between two numbers is exceeds the given threshold.
#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $delta:expr) => {
        if (f64::abs($x - $y) > $delta) {
            panic!("{} != {}", $x, $y);
        }
    };
}

#[macro_export]
macro_rules! assert_delta_f32 {
    ($x:expr, $y:expr, $delta:expr) => {
        if (f32::abs($x - $y) > $delta) {
            panic!("{} != {}", $x, $y);
        }
    };
}

#[macro_export]
macro_rules! assert_eq_len {
    ($a:expr, $b:expr) => {
        if $a.len() != $b.len() {
            return Err(Error::DiffLenError);
        }
    };
}

#[macro_export]
macro_rules! assert_empty {
    ($a:expr) => {
        if $a.is_empty() {
            return Err(Error::EmptyValError);
        }
    };
}
