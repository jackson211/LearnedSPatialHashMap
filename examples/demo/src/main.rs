//! # LSPH Demo Application
//!
//! This demo showcases the capabilities of the Learned Spatial HashMap (LSPH)
//! using real-world geographic data from Melbourne, Australia.
//!
//! Features demonstrated:
//! - Loading and processing geographic CSV data
//! - Spatial indexing with LSPH
//! - Nearest neighbor searches
//! - Range queries
//! - Performance benchmarking
//! - Interactive command-line interface

use clap::{Arg, Command};
use colored::*;
use csv::ReaderBuilder;
use lsph::{
    geometry::Point,
    map::LearnedHashMap,
    models::LinearModel,
};
use rand::Rng;
use serde::Deserialize;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    time::{Duration, Instant},
};

/// Represents a geographic point from the Melbourne dataset
#[derive(Debug, Deserialize, Clone)]
struct GeoPoint {
    #[serde(rename = "0")]
    latitude: f64,
    #[serde(rename = "1")]
    longitude: f64,
    #[serde(rename = "2")]
    zone: u32,
}

/// Statistics for performance analysis
#[derive(Debug, Default)]
struct PerformanceStats {
    data_loading_time: Duration,
    index_building_time: Duration,
    total_points: usize,
    nearest_neighbor_times: Vec<Duration>,
    range_query_times: Vec<Duration>,
    memory_usage_estimate: usize,
}

/// Main demo application
struct LSPHDemo {
    spatial_map: LearnedHashMap<LinearModel<f64>, f64>,
    points: Vec<GeoPoint>,
    stats: PerformanceStats,
}

impl LSPHDemo {
    /// Create a new demo instance
    fn new() -> Self {
        Self {
            spatial_map: LearnedHashMap::new(),
            points: Vec::new(),
            stats: PerformanceStats::default(),
        }
    }

    /// Load geographic data from CSV file
    fn load_data(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        println!(
            "{}\n{}",
            "🗺️  Loading Melbourne Geographic Data".bright_blue().bold(),
            "=".repeat(50).bright_blue()
        );

        let start_time = Instant::now();
        let file = File::open(file_path)?;
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        let mut loaded_points = Vec::new();
        let mut zone_counts: HashMap<u32, usize> = HashMap::new();

        for (index, result) in reader.deserialize().enumerate() {
            match result {
                Ok(point) => {
                    let geo_point: GeoPoint = point;
                    *zone_counts.entry(geo_point.zone).or_insert(0) += 1;
                    loaded_points.push(geo_point);
                }
                Err(e) => {
                    eprintln!(
                        "{} Error parsing line {}: {}",
                        "⚠️".yellow(),
                        index + 1,
                        e
                    );
                }
            }
        }

        self.points = loaded_points;
        self.stats.data_loading_time = start_time.elapsed();
        self.stats.total_points = self.points.len();

        println!(
            "✅ Loaded {} points in {:.2}ms",
            self.stats.total_points.to_string().bright_green(),
            self.stats.data_loading_time.as_secs_f64() * 1000.0
        );

        // Display zone distribution
        println!("\n📊 Zone Distribution:");
        let mut zones: Vec<_> = zone_counts.into_iter().collect();
        zones.sort_by_key(|&(zone, _)| zone);
        for (zone, count) in zones.iter().take(10) {
            let percentage = (*count as f64 / self.stats.total_points as f64) * 100.0;
            println!(
                "   Zone {}: {} points ({:.1}%)",
                zone.to_string().cyan(),
                count.to_string().bright_white(),
                percentage
            );
        }
        if zones.len() > 10 {
            println!("   ... and {} more zones", zones.len() - 10);
        }

        Ok(())
    }

    /// Build the spatial index
    fn build_index(&mut self) -> Result<(), Box<dyn Error>> {
        println!(
            "\n{}\n{}",
            "🏗️  Building Spatial Index".bright_blue().bold(),
            "=".repeat(50).bright_blue()
        );

        let start_time = Instant::now();
        let mut successful_insertions = 0;

        for geo_point in &self.points {
            let point = Point::new(geo_point.latitude, geo_point.longitude);

            match self.spatial_map.insert(point) {
                Some(_existing) => {
                    // Point already existed, this is fine
                    successful_insertions += 1;
                }
                None => {
                    // New point inserted successfully
                    successful_insertions += 1;
                }
            }
        }

        self.stats.index_building_time = start_time.elapsed();
        self.stats.memory_usage_estimate = self.estimate_memory_usage();

        println!(
            "✅ Built spatial index in {:.2}ms",
            self.stats.index_building_time.as_secs_f64() * 1000.0
        );
        println!(
            "📈 Successful insertions: {}",
            successful_insertions.to_string().bright_green()
        );
        println!(
            "💾 Estimated memory usage: {:.2} MB",
            self.stats.memory_usage_estimate as f64 / 1_048_576.0
        );

        Ok(())
    }

    /// Estimate memory usage of the spatial map
    fn estimate_memory_usage(&self) -> usize {
        // Rough estimation based on point count and structure overhead
        let point_size = std::mem::size_of::<Point<f64>>();
        let base_overhead = 1024; // Base structure overhead
        let per_point_overhead = 64; // Hash table and indexing overhead per point

        base_overhead + (self.stats.total_points * (point_size + per_point_overhead))
    }

    /// Perform nearest neighbor search demonstrations
    fn demo_nearest_neighbor(&mut self, num_queries: usize) {
        println!(
            "\n{}\n{}",
            "🎯 Nearest Neighbor Search Demo".bright_blue().bold(),
            "=".repeat(50).bright_blue()
        );

        let mut rng = rand::rng();
        let mut successful_queries = 0;

        for i in 0..num_queries {
            // Generate random query point within Melbourne bounds
            let query_lat = rng.random_range(-37.9..=-37.7);
            let query_lng = rng.random_range(144.8..=145.1);
            let query_point = [query_lat, query_lng];

            let start_time = Instant::now();
            let result = self.spatial_map.nearest_neighbor(&query_point);
            let query_time = start_time.elapsed();

            self.stats.nearest_neighbor_times.push(query_time);

            match result {
                Some(nearest) => {
                    successful_queries += 1;
                    if i < 5 {
                        // Show details for first few queries
                        let distance = self.calculate_distance(
                            query_lat,
                            query_lng,
                            nearest.x(),
                            nearest.y(),
                        );
                        println!(
                            "🔍 Query {}: ({:.5}, {:.5}) → Nearest: ({:.5}, {:.5}) | Distance: {:.2}m | Time: {:.2}μs",
                            (i + 1).to_string().cyan(),
                            query_lat,
                            query_lng,
                            nearest.x(),
                            nearest.y(),
                            distance,
                            query_time.as_nanos() as f64 / 1000.0
                        );
                    }
                }
                None => {
                    if i < 5 {
                        println!(
                            "❌ Query {}: ({:.5}, {:.5}) → No result found",
                            (i + 1).to_string().red(),
                            query_lat,
                            query_lng
                        );
                    }
                }
            }
        }

        let avg_time = self.stats.nearest_neighbor_times.iter().sum::<Duration>().as_nanos()
            / self.stats.nearest_neighbor_times.len() as u128;

        println!(
            "\n📊 Nearest Neighbor Results:"
        );
        println!(
            "   Successful queries: {}/{}",
            successful_queries.to_string().bright_green(),
            num_queries
        );
        println!(
            "   Average query time: {:.2}μs",
            avg_time as f64 / 1000.0
        );
        println!(
            "   Queries per second: {:.0}",
            1_000_000.0 / (avg_time as f64 / 1000.0)
        );
    }

    /// Perform range query demonstrations
    fn demo_range_queries(&mut self, num_queries: usize) {
        println!(
            "\n{}\n{}",
            "🌐 Range Query Demo".bright_blue().bold(),
            "=".repeat(50).bright_blue()
        );

        let mut rng = rand::rng();
        let radii = [0.001, 0.005, 0.01, 0.02]; // Different search radii in degrees

        for &radius in &radii {
            println!(
                "\n🔍 Testing radius: {:.3}° (~{:.0}m)",
                radius,
                radius * 111_000.0 // Rough conversion to meters
            );

            let mut total_results = 0;
            let mut query_times = Vec::new();

            for i in 0..num_queries {
                let query_lat = rng.random_range(-37.9..=-37.7);
                let query_lng = rng.random_range(144.8..=145.1);
                let query_point = [query_lat, query_lng];

                let start_time = Instant::now();
                let results = self.spatial_map.radius_range(&query_point, radius);
                let query_time = start_time.elapsed();

                query_times.push(query_time);

                match results {
                    Some(points) => {
                        total_results += points.len();
                        if i == 0 {
                            // Show details for first query
                            println!(
                                "   Sample query: ({:.5}, {:.5}) → {} points found in {:.2}μs",
                                query_lat,
                                query_lng,
                                points.len().to_string().bright_green(),
                                query_time.as_nanos() as f64 / 1000.0
                            );
                        }
                    }
                    None => {
                        if i == 0 {
                            println!(
                                "   Sample query: ({:.5}, {:.5}) → No results",
                                query_lat, query_lng
                            );
                        }
                    }
                }
            }

            let avg_time = query_times.iter().sum::<Duration>().as_nanos() / query_times.len() as u128;
            let avg_results = total_results as f64 / num_queries as f64;

            println!(
                "   Average results per query: {:.1}",
                avg_results
            );
            println!(
                "   Average query time: {:.2}μs",
                avg_time as f64 / 1000.0
            );

            self.stats.range_query_times.extend(query_times);
        }
    }

    /// Calculate approximate distance between two geographic points
    fn calculate_distance(&self, lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
        let dlat = (lat2 - lat1).to_radians();
        let dlng = (lng2 - lng1).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlng / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        6371000.0 * c // Earth radius in meters
    }

    /// Display comprehensive performance summary
    fn display_performance_summary(&self) {
        println!(
            "\n{}\n{}",
            "📈 Performance Summary".bright_blue().bold(),
            "=".repeat(50).bright_blue()
        );

        println!("🗂️  Data Processing:");
        println!(
            "   Total points processed: {}",
            self.stats.total_points.to_string().bright_green()
        );
        println!(
            "   Data loading time: {:.2}ms",
            self.stats.data_loading_time.as_secs_f64() * 1000.0
        );
        println!(
            "   Index building time: {:.2}ms",
            self.stats.index_building_time.as_secs_f64() * 1000.0
        );
        println!(
            "   Points per second (indexing): {:.0}",
            self.stats.total_points as f64 / self.stats.index_building_time.as_secs_f64()
        );

        if !self.stats.nearest_neighbor_times.is_empty() {
            let nn_avg = self.stats.nearest_neighbor_times.iter().sum::<Duration>().as_nanos()
                / self.stats.nearest_neighbor_times.len() as u128;
            let nn_min = self.stats.nearest_neighbor_times.iter().min().unwrap().as_nanos();
            let nn_max = self.stats.nearest_neighbor_times.iter().max().unwrap().as_nanos();

            println!("\n🎯 Nearest Neighbor Performance:");
            println!(
                "   Average time: {:.2}μs",
                nn_avg as f64 / 1000.0
            );
            println!(
                "   Min time: {:.2}μs",
                nn_min as f64 / 1000.0
            );
            println!(
                "   Max time: {:.2}μs",
                nn_max as f64 / 1000.0
            );
            println!(
                "   Queries per second: {:.0}",
                1_000_000.0 / (nn_avg as f64 / 1000.0)
            );
        }

        if !self.stats.range_query_times.is_empty() {
            let rq_avg = self.stats.range_query_times.iter().sum::<Duration>().as_nanos()
                / self.stats.range_query_times.len() as u128;

            println!("\n🌐 Range Query Performance:");
            println!(
                "   Average time: {:.2}μs",
                rq_avg as f64 / 1000.0
            );
            println!(
                "   Queries per second: {:.0}",
                1_000_000.0 / (rq_avg as f64 / 1000.0)
            );
        }

        println!("\n💾 Memory Usage:");
        println!(
            "   Estimated total: {:.2} MB",
            self.stats.memory_usage_estimate as f64 / 1_048_576.0
        );
        println!(
            "   Per point: {:.1} bytes",
            self.stats.memory_usage_estimate as f64 / self.stats.total_points as f64
        );
    }

    /// Run interactive mode
    fn run_interactive(&mut self) {
        println!(
            "\n{}\n{}",
            "🎮 Interactive Mode".bright_blue().bold(),
            "=".repeat(50).bright_blue()
        );
        println!("Enter coordinates to find nearest neighbors (format: lat,lng) or 'quit' to exit:");

        loop {
            print!("🔍 Query: ");
            use std::io::{self, Write};
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();
                    if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
                        break;
                    }

                    let coords: Vec<&str> = input.split(',').collect();
                    if coords.len() != 2 {
                        println!("❌ Invalid format. Use: lat,lng (e.g., -37.8136,144.9631)");
                        continue;
                    }

                    match (coords[0].trim().parse::<f64>(), coords[1].trim().parse::<f64>()) {
                        (Ok(lat), Ok(lng)) => {
                            let query_point = [lat, lng];
                            let start_time = Instant::now();
                            
                            match self.spatial_map.nearest_neighbor(&query_point) {
                                Some(nearest) => {
                                    let query_time = start_time.elapsed();
                                    let distance = self.calculate_distance(lat, lng, nearest.x(), nearest.y());
                                    
                                    println!(
                                        "✅ Nearest point: ({:.5}, {:.5})",
                                        nearest.x(), nearest.y()
                                    );
                                    println!(
                                        "📏 Distance: {:.2}m | ⏱️  Query time: {:.2}μs",
                                        distance,
                                        query_time.as_nanos() as f64 / 1000.0
                                    );
                                }
                                None => {
                                    println!("❌ No nearest neighbor found");
                                }
                            }
                        }
                        _ => {
                            println!("❌ Invalid coordinates. Use decimal format (e.g., -37.8136,144.9631)");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            }
        }

        println!("👋 Goodbye!");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("LSPH Demo")
        .version("0.1.0")
        .author("LSPH Contributors")
        .about("Demonstrates LSPH capabilities with Melbourne geographic data")
        .arg(
            Arg::new("data")
                .short('d')
                .long("data")
                .value_name("FILE")
                .help("Path to the CSV data file")
                .default_value("melbourne.csv")
        )
        .arg(
            Arg::new("queries")
                .short('q')
                .long("queries")
                .value_name("NUMBER")
                .help("Number of test queries to perform")
                .default_value("100")
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Run in interactive mode")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("skip-demo")
                .long("skip-demo")
                .help("Skip automated demo and go straight to interactive mode")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let data_file = matches.get_one::<String>("data").unwrap();
    let num_queries: usize = matches.get_one::<String>("queries").unwrap().parse()?;
    let interactive_mode = matches.get_flag("interactive");
    let skip_demo = matches.get_flag("skip-demo");

    println!(
        "{}\n{}\n{}",
        "🗺️  LSPH Geographic Data Demo".bright_blue().bold(),
        "Learned Spatial HashMap Performance Demonstration".bright_white(),
        "=".repeat(60).bright_blue()
    );

    let mut demo = LSPHDemo::new();

    // Load data
    demo.load_data(data_file)?;

    // Build spatial index
    demo.build_index()?;

    if !skip_demo {
        // Run performance demonstrations
        demo.demo_nearest_neighbor(num_queries);
        demo.demo_range_queries(num_queries / 4); // Fewer range queries as they're more expensive

        // Display comprehensive summary
        demo.display_performance_summary();
    }

    // Run interactive mode if requested
    if interactive_mode || skip_demo {
        demo.run_interactive();
    }

    println!(
        "\n{}\n{}",
        "🎉 Demo completed successfully!".bright_green().bold(),
        "Thank you for exploring LSPH capabilities.".bright_white()
    );

    Ok(())
}
