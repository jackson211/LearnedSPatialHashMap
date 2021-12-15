mod primitives;

use primitives::point::Point;
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

pub fn read_config(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string(config.filename)?;
    // let slice = &contents[..10];
    // println!("With text:\n{}", slice);

    let filepath = config.filename.clone();
    let fd = File::open(&filepath).expect(&format!(
        "[ ERROR ] Failed to open data file at {}",
        &filepath
    ));
    let reader = BufReader::new(fd);

    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split(",").collect();
        let lat = tokens[0].parse::<f64>().unwrap();
        let lng = tokens[1].parse::<f64>().unwrap();
        let key = tokens[2].parse::<f64>().unwrap();
        let p = Point { x: lat, y: lng };
        println!("{}", &p);
        println!("{}", &key);
    }

    Ok(())
}
