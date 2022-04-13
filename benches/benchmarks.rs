#[macro_use]
extern crate criterion;

extern crate lsph;
extern crate rand;
extern crate rand_hc;

use rand::{Rng, SeedableRng};
use rand_hc::Hc128Rng;

use lsph::primitives::point::Point;
use lsph::{algorithm::LinearModel, map::LearnedHashMap};

use criterion::Criterion;

const SEED_1: &[u8; 32] = b"Gv0aHMtHkBGsUXNspGU9fLRuCWkZWHZx";
const SEED_2: &[u8; 32] = b"km7DO4GeaFZfTcDXVpnO7ZJlgUY7hZiS";

const DEFAULT_BENCHMARK_TREE_SIZE: usize = 2000;

fn bulk_load_baseline(c: &mut Criterion) {
    c.bench_function("Bulk load baseline", move |b| {
        let points: Vec<_> = create_random_point_type_points(DEFAULT_BENCHMARK_TREE_SIZE, SEED_1);
        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();

        b.iter(|| {
            map.fit_batch_insert(&points);
        });
    });
}

fn locate_successful(c: &mut Criterion) {
    let points: Vec<_> = create_random_point_type_points(100_000, SEED_1);
    let query_point = create_random_points(100_000, SEED_1)[500];

    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    map.fit_batch_insert(&points);
    c.bench_function("locate_at_point (successful)", move |b| {
        b.iter(|| map.get(&query_point).is_some())
    });
}

fn locate_unsuccessful(c: &mut Criterion) {
    let points: Vec<_> = create_random_point_type_points(100_000, SEED_1);

    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    map.fit_batch_insert(&points);
    let query_point = (0.7, 0.7);
    c.bench_function("locate_at_point (unsuccessful)", move |b| {
        b.iter(|| map.get(&query_point).is_none())
    });
}

fn nearest_neighbor(c: &mut Criterion) {
    const SIZE: usize = 100_000;
    let points: Vec<_> = create_random_point_type_points(SIZE, SEED_1);

    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    map.fit_batch_insert(&points);

    let query_points = create_random_points(100, SEED_2);
    c.bench_function("nearest_neigbor", move |b| {
        b.iter(|| {
            for query_point in &query_points {
                map.nearest_neighbor(&query_point).unwrap();
            }
        });
    });
}

criterion_group!(
    benches,
    bulk_load_baseline,
    locate_successful,
    locate_unsuccessful,
    nearest_neighbor,
);
criterion_main!(benches);

fn create_random_points(num_points: usize, seed: &[u8; 32]) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(num_points);
    let mut rng = Hc128Rng::from_seed(*seed);
    for _ in 0..num_points {
        result.push((rng.gen(), rng.gen()));
    }
    result
}

fn create_random_point_type_points(num_points: usize, seed: &[u8; 32]) -> Vec<Point<f64>> {
    let mut result: Vec<(f64, f64)> = Vec::with_capacity(num_points);
    let mut rng = Hc128Rng::from_seed(*seed);
    for _ in 0..num_points {
        let x = rng.gen();
        let y = rng.gen();
        result.push((x, y));
    }

    result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    result
        .into_iter()
        .enumerate()
        .map(|(id, (x, y))| Point { id, x, y })
        .collect::<Vec<_>>()
}
