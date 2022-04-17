#[macro_use]
extern crate criterion;

use criterion::Criterion;
use lsph::geometry::Point;
use lsph::{map::LearnedHashMap, models::LinearModel};
use rand::{Rng, SeedableRng};
use rand_hc::Hc128Rng;

const SEED_1: &[u8; 32] = b"Gv0aHMtHkBGsUXNspGU9fLRuCWkZWHZx";
const SEED_2: &[u8; 32] = b"km7DO4GeaFZfTcDXVpnO7ZJlgUY7hZiS";

const DEFAULT_BENCHMARK_TREE_SIZE: usize = 2000;

fn bulk_load_baseline(c: &mut Criterion) {
    c.bench_function("Bulk load baseline", move |b| {
        let mut points: Vec<_> =
            create_random_point_type_points(DEFAULT_BENCHMARK_TREE_SIZE, SEED_1);
        let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();

        b.iter(|| {
            map.fit_batch_insert(&mut points);
        });
    });
}

fn locate_successful(c: &mut Criterion) {
    let mut points: Vec<_> = create_random_point_type_points(100_000, SEED_1);
    let query_point = (points[500].x(), points[500].y());

    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    map.fit_batch_insert(&mut points);
    c.bench_function("locate_at_point (successful)", move |b| {
        b.iter(|| map.get(&query_point).is_some())
    });
}

fn locate_unsuccessful(c: &mut Criterion) {
    let mut points: Vec<_> = create_random_point_type_points(100_000, SEED_1);
    let query_point = (0.7, 0.7);

    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    map.fit_batch_insert(&mut points);
    c.bench_function("locate_at_point (unsuccessful)", move |b| {
        b.iter(|| map.get(&query_point).is_none())
    });
}

fn nearest_neighbor(c: &mut Criterion) {
    const SIZE: usize = 100_000;
    let mut points: Vec<_> = create_random_point_type_points(SIZE, SEED_1);
    let query_points = create_random_points(100, SEED_2);

    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    map.fit_batch_insert(&mut points);

    c.bench_function("nearest_neigbor", move |b| {
        b.iter(|| {
            for query_point in &query_points {
                map.nearest_neighbor(&query_point).unwrap();
            }
        });
    });
}

fn radius_range(c: &mut Criterion) {
    const SIZE: usize = 100_000;
    let mut points: Vec<_> = create_random_point_type_points(SIZE, SEED_1);
    let query_points = create_random_points(100, SEED_2);

    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    map.fit_batch_insert(&mut points);

    let radiuses = vec![0.01, 0.1, 0.2];
    for radius in radiuses {
        let title = format!("radius_range_{radius}");
        c.bench_function(title.as_str(), |b| {
            b.iter(|| {
                for query_point in &query_points {
                    map.radius_range(&query_point, radius).unwrap();
                }
            });
        });
    }
}

criterion_group!(
    benches,
    bulk_load_baseline,
    locate_successful,
    locate_unsuccessful,
    radius_range,
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
    let result = create_random_points(num_points, seed);

    // result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    result
        .into_iter()
        .enumerate()
        .map(|(id, (x, y))| Point::new(id, x, y))
        .collect::<Vec<_>>()
}
