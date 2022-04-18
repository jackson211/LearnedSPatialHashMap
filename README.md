# LSPH - Learned SPatial HashMap

![Github Workflow](https://github.com/jackson211/lsph/actions/workflows/rust.yml/badge.svg)
![crates.io version](https://img.shields.io/crates/v/lsph)
![dos.io](https://img.shields.io/docsrs/lsph)

The original paper of LSPH can be found [here].

[here]: https://minerva-access.unimelb.edu.au/items/beb5c0ee-2a8d-5bd2-b349-1190a335ef1a

The LSPH uses a learned model such as a linear regression model as the hash function to predict the index in a hashmap. As a result, the learned model is more fitted to the data that stored in the hashmap, and reduces the
chance of hashing collisions. Moreover, if the learned model is monotonic function(e.g. linear regression), the hash indexes are increasing as the input data increases. This property can be used to create a sorted order
of buckets in a hashmap, which allow us to do range searchs in a hashmap.

The LSPH supports:

- Point Query
- Rectange Query
- Radius Range Query
- Nearest Neighbor Query

Example:
```
use lsph::{LearnedHashMap, LinearModel, Point};
let mut data: Vec<Point<f64>> = vec![
    Point::new(1, 1., 1.),
    Point::new(2, 3., 1.),
    Point::new(3, 2., 1.),
    Point::new(4, 3., 2.),
    Point::new(5, 5., 1.),
];
let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
map.batch_insert(&mut data).unwrap();

```

# License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
