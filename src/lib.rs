pub mod algorithm;
pub mod types;

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

pub fn read_config(config: Config) -> Option<String> {
    // let contents = fs::read_to_string(config.filename)?;
    // let slice = &contents[..10];
    // println!("With text:\n{}", slice);

    println!("Query Type: {}", config.query);
    println!("Filename: {}", config.filename);

    Some(config.filename.clone())
}
