use lsph::algorithm::map2::HashMap;
use lsph::algorithm::Point;
//use lsph::*;

extern crate ordered_float;
use ordered_float::OrderedFloat;

// use geo_types::Point;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::mem;

/// Simple Linear Regression Model
fn slr(data: Vec<(f64, f64)>) -> (f64, f64) {
    // compute the covariance of x and y as well as the variance of x in
    // a single pass.

    let mut mean_x = 0.0;
    let mut mean_y = 0.0;
    let mut c = 0.0;
    let mut n: u64 = 0;
    let mut m2 = 0.0;

    let mut data_size = 0;
    for (x, y) in data {
        n += 1;
        let dx = x - mean_x;
        mean_x += dx / (n as f64);
        mean_y += (y - mean_y) / (n as f64);
        c += dx * (y - mean_y);

        let dx2 = x - mean_x;
        m2 += dx * dx2;
        data_size += 1;
    }

    // special case when we have 0 or 1 items
    if data_size == 0 {
        return (0.0, 0.0);
    }

    if data_size == 1 {
        return (mean_y, 0.0);
    }

    let cov = c / ((n - 1) as f64);
    let var = m2 / ((n - 1) as f64);
    assert!(
        var >= 0.0,
        "variance of model with {} data items was negative",
        n
    );

    if var == 0.0 {
        // variance is zero. pick the mean (only) value.
        return (mean_y, 0.0);
    }

    let beta: f64 = cov / var;
    let alpha = mean_y - beta * mean_x;

    return (alpha, beta);
}

/// Hashmap Key: Point pair -> (f64, f64)
fn load_data(filepath: &str) -> Result<Vec<Point<f64>>, Box<dyn Error>> {
    let fd = File::open(&filepath).expect(&format!(
        "[ ERROR ] Failed to open data file at {}",
        &filepath
    ));
    let reader = BufReader::new(fd);

    let mut count: usize = 0;
    let mut points: Vec<Point<f64>> = Vec::new();
    let mut training_set: Vec<(f64, f64)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split(",").collect();
        let lat = tokens[0].parse::<f64>().unwrap();
        let lng = tokens[1].parse::<f64>().unwrap();
        //let key = tokens[2].parse::<f64>().unwrap();
        // let p = Point::new(lat, lng);
        training_set.push((lat, count as f64));
        points.push(Point {
            id: count,
            value: (lat, lng),
        });
        count += 1;
    }
    println!("Number of points: {}", count);

    // train model
    let (b, w) = slr(training_set);
    println!("Model param: {}, {}", b, w);

    // model prediction
    let prediction = |x: f64| (x * w + b + 0.5f64 - ((x < 0f64) as i32) as f64) as i64;

    // get all predicted indexes
    let predicted_indexes: Vec<i64> = points.iter().map(|i| prediction(i.value.0)).collect();
    let min = predicted_indexes.iter().min().unwrap();
    let max = predicted_indexes.iter().max().unwrap();
    println!("min {}", min);
    println!("max {}", max);

    // insert into map
    // init with an capacity
    println!("Map size: {}", count);
    let mut map: HashMap<(OrderedFloat<f64>, OrderedFloat<f64>), Point<f64>> =
        HashMap::with_capacity(count);

    // Batch insert
    for i in 0..10 {
        let lat = points[i].value.0;
        let lng = points[i].value.1;
        map.insert((OrderedFloat(lat), OrderedFloat(lng)), points[i]);
    }
    println!("Map : {:?}", map);

    // substract by the min value in Vec
    let predicted_indexes: Vec<i64> = predicted_indexes.iter().map(|x| x - min).collect();
    let min = predicted_indexes.iter().min().unwrap();
    let max = predicted_indexes.iter().max().unwrap();
    println!("min {}", min);
    println!("max {}", max);

    Ok(points)
}

fn main() {
    println!("Starting ");
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match read_config(config) {
        Some(filepath) => load_data(&filepath),
        None => {
            eprintln!("Can not read the file");
            process::exit(1);
        }
    };
}
