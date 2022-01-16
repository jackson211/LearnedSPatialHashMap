use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::raw::{Bucket, RawTable};

pub struct LearnedHashMap<K, V, H = DefaultHashBuilder> {
    hash_builder: H,
    table: RawTable<(K, V)>,
}

// impl LearnedHashMap<K, V, H> {
//     pub fn new() -> Self {
//         LearnedHashMap{
//         }
//     }
// }
