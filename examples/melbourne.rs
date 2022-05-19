use kiss3d::light::Light;
use kiss3d::window::Window;
use lsph::{LearnedHashMap, LinearModel, Point};
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
        let _key = tokens[2].parse::<f64>().unwrap();
        data.push((lat, lng));
    }

    Ok(data
        .into_iter()
        .map(|(x, y)| Point::new(x, y))
        .collect::<Vec<_>>())
}

fn main() {
    let mut map = LearnedHashMap::<LinearModel<f64>, f64>::new();
    if let Ok(mut data) = load_data("./examples/melbourne.csv") {
        map.batch_insert(&mut data).unwrap();
    };

    const WINDOW_WIDTH: u32 = 1024;
    const WINDOW_HEIGHT: u32 = 768;
    let mut window = Window::new_with_size("RStar demo", WINDOW_WIDTH, WINDOW_HEIGHT);
    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);
    window.set_point_size(4.);
}
