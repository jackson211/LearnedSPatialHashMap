use lsph::Config;
use std::env;
use std::process;

fn main() {
    println!("Starting ");
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let data = match lsph::read_config(config) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    };
    // println!("{:?}", data);
}
