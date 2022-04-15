//! This crate implements the Learned SPatial HashMap(LSPH), a high performance
//! spatial index uses HashMap with learned model.    
//!
//! The original paper of LSPH can be found [here].
//!
//! [here]:{https://minerva-access.unimelb.edu.au/items/beb5c0ee-2a8d-5bd2-b349-1190a335ef1a}

#[macro_use]
mod macros;

pub use map::*;
pub mod algorithm;
mod distance;
mod error;
pub mod geometry;
mod hasher;
pub mod map;
mod nn;

#[cfg(test)]
pub mod test_utilities;
