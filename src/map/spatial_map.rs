// use crate::{
//     geometry::Point,
//     map::LearnedHashMap,
//     models::{Model, Trainer},
// };
// use core::iter::Sum;
// use num_traits::{
//     cast::{AsPrimitive, FromPrimitive},
//     float::Float,
// };
// use std::fmt::Debug;

// #[derive(Default, Debug, Clone)]
// pub struct LearnedSpatialHashMap<M, F> {
//     map: LearnedHashMap<M, F>,
//     trainer: Trainer<F>,
// }

// impl<M, F> LearnedSpatialHashMap<M, F>
// where
//     F: Float + AsPrimitive<u64> + FromPrimitive + Default + Debug + Sum,
//     M: Model<F = F>,
// {
//     pub fn new() -> Self {
//         Self {
//             map: LearnedHashMap::new(),
//             trainer: Trainer::new(),
//         }
//     }

//     pub fn insert(&mut self, p: Point<F>) -> Option<Point<F>> {
//         self.map.insert(p)
//     }

//     pub fn fit_batch_insert(&mut self, ps: &mut [F; 2]) {
//         self.trainer = Trainer::with_points(ps)?;
//         self.trainer.train(&mut self.map.model);
//     }
// }
