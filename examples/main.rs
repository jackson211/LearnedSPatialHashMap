use lsph::{geometry::Point, models::LinearModel, LearnedHashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn load_data(filepath: &str) -> Result<Vec<Point<f64>>, Error> {
    let fd = File::open(filepath).expect(&format!("Unable to open data file at {}", filepath));
    let reader = BufReader::new(fd);

    let mut data = Vec::new();
    for line in reader.lines() {
        let line_string = match line {
            Ok(line_string) => line_string,
            Err(e) => return Err(e),
        };
        let tokens: Vec<&str> = line_string.split(",").collect();
        let lat = tokens[0].parse::<f64>().unwrap();
        let lng = tokens[1].parse::<f64>().unwrap();
        let key = tokens[2].parse::<f64>().unwrap();
        data.push((lat, lng));
    }

    data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    Ok(data
        .into_iter()
        .enumerate()
        .map(|(id, (x, y))| Point::new(id, x, y))
        .collect::<Vec<_>>())
}

fn main() {
    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
}
