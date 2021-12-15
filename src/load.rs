use crate::models::ModelData;
use geohash::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn load_data(filepath: &str) -> Result<ModelData, Error> {
    let fd = File::open(filepath).expect(&format!("Unable to open data file at {}", filepath));
    let reader = BufReader::new(fd);
    let mut x: Vec<f64> = vec![];
    let mut y: Vec<f64> = vec![];
    for line in reader.lines() {
        let line_string = match line {
            Ok(line_string) => line_string,
            Err(e) => return Err(e),
        };

        let tokens: Vec<&str> = line_string.split(",").collect();
        let lat = tokens[0].parse::<f64>().unwrap();
        let lng = tokens[1].parse::<f64>().unwrap();
        let key = tokens[2].parse::<f64>().unwrap();
        let hash_coor = encode_int(lat, lng) as f64;
        x.push(hash_coor);
        y.push(key);
    }
    let data = ModelData::new(x, y);
    Ok(data)
}
