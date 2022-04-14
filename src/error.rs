/// The kinds of errors that can occur when calculating a linear regression.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    /// The slope is too steep to represent, approaching infinity.
    SteepSlopeError,
    /// Failed to calculate mean.

    /// Different input lenses  
    DiffLenError,

    /// Input was empty
    EmptyValError,
}
