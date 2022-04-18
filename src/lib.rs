//! This crate implements the Learned SPatial HashMap(LSPH), a high performance
//! spatial index uses HashMap with learned model.    
//!
//! The original paper of LSPH can be found [here].
//!
//! [here]: https://minerva-access.unimelb.edu.au/items/beb5c0ee-2a8d-5bd2-b349-1190a335ef1a
//!
//! The LSPH uses a learned model such as a linear regression model as the hash
//! function to predict the index in a hashmap. As a result, the learned model
//! is more fitted to the data that stored in the hashmap, and reduces the
//! chance of hashing collisions. Moreover, if the learned model is monotonic
//! function(e.g. linear regression), the hash indexes are increasing as the
//! input data increases. This property can be used to create a sorted order
//! of buckets in a hashmap, which allow us to do range searchs in a hashmap.
//!
//! The LSPH supports:
//! - Point Query
//! - Rectange Query
//! - Radius Range Query
//! - Nearest Neighbor Query
//!
//! Example:
//! ```
//! use lsph::{LearnedHashMap, LinearModel, Point};
//! let mut data: Vec<Point<f64>> = vec![
//!     Point {
//!         id: 1,
//!         x: 1.,
//!         y: 1.,
//!     },
//!     Point {
//!         id: 2,
//!         x: 3.,
//!         y: 1.,
//!     },
//!     Point {
//!         id: 3,
//!         x: 2.,
//!         y: 1.,
//!     },
//!     Point {
//!         id: 4,
//!         x: 3.,
//!         y: 2.,
//!     },
//!     Point {
//!         id: 5,
//!         x: 5.,
//!         y: 1.,
//!     },
//! ];
//! let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
//! map.batch_insert(&mut data).unwrap();
//!
//! ```

#[macro_use]
mod macros;
mod error;
pub mod geometry;
pub mod hasher;
pub mod map;
pub mod models;
#[cfg(test)]
pub mod test_utilities;

pub use geometry::*;
pub use hasher::*;
pub use map::*;
pub use models::*;
