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
            map.batch_insert(&mut points).unwrap();
        });
    })
    .bench_function("Bulk load baseline with f32", move |b| {
        let mut points: Vec<_> =
            create_random_point_type_points_f32(DEFAULT_BENCHMARK_TREE_SIZE, SEED_1);
        let mut map = LearnedHashMap::<LinearModel<f32>, f32>::new();

        b.iter(|| {
            map.batch_insert(&mut points).unwrap();
        });
    });
}

fn locate_successful(c: &mut Criterion) {
    let mut points: Vec<_> = create_random_point_type_points(100_000, SEED_1);
    let mut points_f32: Vec<_> = create_random_point_type_points_f32(100_000, SEED_1);
    let query_point = [points[500].x(), points[500].y()];
    let query_point_f32 = [points_f32[500].x(), points_f32[500].y()];

    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    let mut map_f32 = LearnedHashMap::<LinearModel<f32>, f32>::new();
    map.batch_insert(&mut points).unwrap();
    map_f32.batch_insert(&mut points_f32).unwrap();
    c.bench_function("locate_at_point (successful)", move |b| {
        b.iter(|| map.get(&query_point).is_some())
    })
    .bench_function("locate_at_point_f32 (successful)", move |b| {
        b.iter(|| map_f32.get(&query_point_f32).is_some())
    });
}

fn locate_unsuccessful(c: &mut Criterion) {
    let mut points: Vec<_> = create_random_point_type_points(100_000, SEED_1);
    let mut points_f32: Vec<_> = create_random_point_type_points_f32(100_000, SEED_1);
    let query_point: [f64; 2] = [0.7, 0.7];
    let query_point_f32: [f32; 2] = [0.7, 0.7];

    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    let mut map_f32 = LearnedHashMap::<LinearModel<f32>, f32>::new();
    map.batch_insert(&mut points).unwrap();
    map_f32.batch_insert(&mut points_f32).unwrap();
    c.bench_function("locate_at_point (unsuccessful)", move |b| {
        b.iter(|| map.get(&query_point).is_none())
    })
    .bench_function("locate_at_point_f32 (successful)", move |b| {
        b.iter(|| map_f32.get(&query_point_f32).is_some())
    });
}

fn nearest_neighbor(c: &mut Criterion) {
    const SIZE: usize = 100_000;
    let mut points: Vec<_> = create_random_point_type_points(SIZE, SEED_1);
    let query_points = create_random_points(100, SEED_2);

    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    map.batch_insert(&mut points).unwrap();

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
    map.batch_insert(&mut points).unwrap();

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

fn create_random_points(num_points: usize, seed: &[u8; 32]) -> Vec<[f64; 2]> {
    let mut result = Vec::with_capacity(num_points);
    let mut rng = Hc128Rng::from_seed(*seed);
    for _ in 0..num_points {
        result.push([rng.gen(), rng.gen()]);
    }
    result
}

fn create_random_point_type_points(num_points: usize, seed: &[u8; 32]) -> Vec<Point<f64>> {
    let result = create_random_points(num_points, seed);

    // result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    result
        .into_iter()
        .enumerate()
        .map(|(id, [x, y])| Point::new(id, x, y))
        .collect::<Vec<_>>()
}

fn create_random_point_type_points_f32(num_points: usize, seed: &[u8; 32]) -> Vec<Point<f32>> {
    let result = create_random_points(num_points, seed);

    // result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    result
        .into_iter()
        .enumerate()
        .map(|(id, [x, y])| Point::new(id, x as f32, y as f32))
        .collect::<Vec<_>>()
}
