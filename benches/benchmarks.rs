#[macro_use]
extern crate criterion;

extern crate lsph;
extern crate rand;
extern crate rand_hc;

use rand::{Rng, SeedableRng};
use rand_hc::Hc128Rng;

use lsph::{algorithm::linear::LinearModel, map::LearnedHashMap, primitives::point::Point};

use criterion::Criterion;

const SEED_1: &[u8; 32] = b"Gv0aHMtHkBGsUXNspGU9fLRuCWkZWHZx";
// const SEED_2: &[u8; 32] = b"km7DO4GeaFZfTcDXVpnO7ZJlgUY7hZiS";

const DEFAULT_BENCHMARK_SIZE: usize = 2000;

fn bulk_load_baseline(c: &mut Criterion) {
    c.bench_function("Bulk load baseline", move |b| {
        let points: Vec<_> = create_random_point_type_points(DEFAULT_BENCHMARK_SIZE, SEED_1);
        let mut map = LearnedHashMap::<LinearModel>::new();

        b.iter(|| {
            map.fit_batch_insert(&points);
        });
    });
}

// fn bulk_load_comparison(c: &mut Criterion) {
//     let mut group = c.benchmark_group("lsph and spade benchmarks");
//
//     group.bench_function("lsph sequential", |b| {
//         let points: Vec<_> = create_random_points(DEFAULT_BENCHMARK_SIZE, SEED_1);
//         b.iter(move || {
//             let mut rtree = lsph::RTree::new();
//             for point in &points {
//                 rtree.insert(*point);
//             }
//         });
//     });
//     group.finish();
// }

// fn tree_creation_quality(c: &mut Criterion) {
//     const SIZE: usize = 100_000;
//     let points: Vec<_> = create_random_points(SIZE, SEED_1);
//     let tree_bulk_loaded = RTree::<_, Params>::bulk_load_with_params(points.clone());
//     let mut tree_sequential = RTree::new();
//     for point in &points {
//         tree_sequential.insert(*point);
//     }
//
//     let query_points = create_random_points(100, SEED_2);
//     let query_points_cloned_1 = query_points.clone();
//     c.bench_function("bulk load quality", move |b| {
//         b.iter(|| {
//             for query_point in &query_points {
//                 tree_bulk_loaded.nearest_neighbor(&query_point).unwrap();
//             }
//         })
//     })
//     .bench_function("sequential load quality", move |b| {
//         b.iter(|| {
//             for query_point in &query_points_cloned_1 {
//                 tree_sequential.nearest_neighbor(&query_point).unwrap();
//             }
//         });
//     });
// }
//
fn locate_successful(c: &mut Criterion) {
    let points: Vec<_> = create_random_point_type_points(100_000, SEED_1);
    let query_point = create_random_points(100_000, SEED_1)[500];

    let mut map = LearnedHashMap::<LinearModel>::new();
    map.fit_batch_insert(&points);
    c.bench_function("locate_at_point (successful)", move |b| {
        b.iter(|| map.get(&query_point).is_some())
    });
}

fn locate_unsuccessful(c: &mut Criterion) {
    let points: Vec<_> = create_random_point_type_points(100_000, SEED_1);

    let mut map = LearnedHashMap::<LinearModel>::new();
    map.fit_batch_insert(&points);
    let query_point = (0.7, 0.7);
    c.bench_function("locate_at_point (unsuccessful)", move |b| {
        b.iter(|| map.get(&query_point).is_none())
    });
}

criterion_group!(
    benches,
    bulk_load_baseline,
    // bulk_load_comparison,
    // tree_creation_quality,
    locate_successful,
    locate_unsuccessful
);
criterion_main!(benches);

fn create_random_points(num_points: usize, seed: &[u8; 32]) -> Vec<(f32, f32)> {
    let mut result = Vec::with_capacity(num_points);
    let mut rng = Hc128Rng::from_seed(*seed);
    for _ in 0..num_points {
        result.push((rng.gen(), rng.gen()));
    }
    result
}

fn create_random_point_type_points(num_points: usize, seed: &[u8; 32]) -> Vec<Point<f32>> {
    let mut result = create_random_points(num_points, seed);
    result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    result
        .into_iter()
        .enumerate()
        .map(|(id, (x, y))| Point { id, x, y })
        .collect::<Vec<_>>()
}
