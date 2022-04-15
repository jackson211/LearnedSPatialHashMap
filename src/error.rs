/// The kinds of errors that can occur when calculating a linear regression.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    /// The slope is too steep to represent, approaching infinity.
    SteepSlope,

    /// Different input lenses  
    DiffLen,

    /// Input was empty
    EmptyVal,
}
