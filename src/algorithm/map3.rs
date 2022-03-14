use crate::algorithm::Point;
use core::mem;
use std::borrow::Borrow;

//Model
#[derive(Default)]
pub struct Model {
    param: (f64, f64),
}

impl Model {
    pub fn new() -> Self {
        Self { param: (0.0, 0.0) }
    }

    pub fn predict(&self, x: f64) -> u64 {
        (x * self.param.0 + self.param.1).round() as u64
    }
}
/// Hasher
#[derive(Default)]
pub struct LearnedHasher {
    state: u64,
    model: Model,
}

impl LearnedHasher {
    pub fn new() -> Self {
        Self {
            state: 0u64,
            model: Model::new(),
        }
    }

    fn write(&mut self, x: f64) {
        self.state = self.model.predict(x);
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

pub fn make_hash(hasher: &mut LearnedHasher, p: &(f64, f64)) -> u64 {
    //TODO decide which number to be hashed
    hasher.write(p.0);
    hasher.finish()
}

const INITIAL_NBUCKETS: usize = 1;
#[derive(Default)]
pub struct LearnedHashMap {
    hasher: LearnedHasher,
    table: Vec<Vec<Point<f64>>>,
    items: usize,
}

// impl Default for LearnedHashMap {
//     fn default() -> LearnedHashMap {
//         LearnedHashMap::new()
//     }
// }

impl LearnedHashMap {
    pub fn new() -> Self {
        Self {
            hasher: LearnedHasher::new(),
            table: Vec::new(),
            items: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            hasher: LearnedHasher::new(),
            table: Vec::with_capacity(capacity),
            items: 0,
        }
    }

    pub fn insert(&mut self, p: Point<f64>) -> Option<Point<f64>> {
        if self.table.is_empty() || self.items > 3 * self.table.len() / 4 {
            self.resize();
        }
        let hash = make_hash(&mut self.hasher, &p.value) as usize;
        println!("inserting with hash {:?}", hash);

        // Find the bucket at hash location
        let bucket = &mut self.table[hash];

        // Find the key at second bucket
        for &mut ref mut ep in bucket.iter_mut() {
            if ep == &p {
                return Some(mem::replace(ep, p));
            }
        }

        bucket.push(p);
        self.items += 1;
        None
    }

    pub fn get(&mut self, p: &(f64, f64)) -> Option<&Point<f64>> {
        let hash = make_hash(&mut self.hasher, p) as usize;
        // self.table.get(hash, equivalent_key(k))

        self.table[hash]
            .iter()
            .find(|&ep| ep.value.borrow() == p)
            .map(|i| i)
    }

    pub fn contains_key(&mut self, key: &(f64, f64)) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, p: &(f64, f64)) -> Option<Point<f64>> {
        let hash = make_hash(&mut self.hasher, p) as usize;
        let bucket = &mut self.table[hash];
        let i = bucket.iter().position(|&ref ek| ek.value.borrow() == p)?;
        self.items -= 1;
        Some(bucket.swap_remove(i))
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    fn resize(&mut self) {
        let target_size = match self.table.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };

        let mut new_table = Vec::with_capacity(target_size);
        new_table.extend((0..target_size).map(|_| Vec::new()));

        for p in self.table.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            //let mut hasher = DefaultHasher::new();
            //key.hash(&mut hasher);
            let hash = make_hash(&mut self.hasher, &p.value) as usize;
            new_table[hash].push(p);
        }

        self.table = new_table;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::Point;

    #[test]
    fn insert() {
        let mut map: LearnedHashMap = LearnedHashMap::new();
        let a: Point<f64> = Point {
            id: 1,
            value: (0., 1.),
        };

        let b: Point<f64> = Point {
            id: 2,
            value: (1., 0.),
        };
        map.insert(a);
        map.insert(b);
        assert_eq!(map.get(&(0., 1.)).unwrap(), &a);
        assert_eq!(map.get(&(1., 0.)).unwrap(), &b);
    }

    #[test]
    fn insert_repeated() {
        let mut map: LearnedHashMap = LearnedHashMap::new();
        let a: Point<f64> = Point {
            id: 1,
            value: (0., 1.),
        };

        let b: Point<f64> = Point {
            id: 2,
            value: (0., 1.),
        };
        let res = map.insert(a);
        assert_eq!(res, None);
        let res = map.insert(b);
        assert_eq!(res, None);
    }
}
