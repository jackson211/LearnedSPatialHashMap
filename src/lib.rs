extern crate geo_types;
mod algorithm;

use crate::algorithm::linkedlist::List;
use geo_types::Point;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn read_config(config: Config) -> Result<Vec<Point<f64>>, Box<dyn Error>> {
    // let contents = fs::read_to_string(config.filename)?;
    // let slice = &contents[..10];
    // println!("With text:\n{}", slice);

    println!("Query Type: {}", config.query);
    println!("Filename: {}", config.filename);

    let filepath = config.filename.clone();
    load_data(&filepath)
}

fn load_data(filepath: &str) -> Result<Vec<Point<f64>>, Box<dyn Error>> {
    let fd = File::open(&filepath).expect(&format!(
        "[ ERROR ] Failed to open data file at {}",
        &filepath
    ));
    let reader = BufReader::new(fd);

    let mut count = 0;
    let mut points: Vec<Point<f64>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split(",").collect();
        let lat = tokens[0].parse::<f64>().unwrap();
        let lng = tokens[1].parse::<f64>().unwrap();
        //let key = tokens[2].parse::<f64>().unwrap();
        let p = Point::new(lat, lng);
        points.push(p);
        count += 1;
    }
    println!("Number of points: {}", count);

    Ok(points)
}
