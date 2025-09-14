# LSPH Geographic Data Demo

A comprehensive demonstration of the Learned Spatial HashMap (LSPH) capabilities using real-world geographic data from Melbourne, Australia.

## Overview

This demo showcases LSPH's spatial indexing and querying capabilities through:

- **Real-world Data**: 6,361 geographic points from Melbourne
- **Performance Benchmarking**: Detailed timing and throughput analysis
- **Interactive Exploration**: Command-line interface for custom queries
- **Comprehensive Metrics**: Memory usage, query performance, and scalability analysis

## Features

### 🗺️ Data Processing
- Loads CSV geographic data with latitude, longitude, and zone information
- Displays data distribution and zone statistics
- Robust error handling for malformed data

### 🏗️ Spatial Indexing
- Builds LSPH spatial index from geographic points
- Tracks insertion performance and memory usage
- Provides detailed indexing statistics

### 🎯 Nearest Neighbor Search
- Performs configurable number of nearest neighbor queries
- Measures query performance in microseconds
- Calculates real-world distances using Haversine formula
- Reports queries per second and success rates

### 🌐 Range Queries
- Tests multiple search radii (0.001° to 0.02°)
- Converts degrees to approximate meters for intuitive understanding
- Analyzes average results per query and performance metrics

### 📈 Performance Analysis
- Comprehensive timing analysis (min, max, average)
- Memory usage estimation
- Throughput calculations (points/second, queries/second)
- Detailed performance summaries

### 🎮 Interactive Mode
- Real-time nearest neighbor queries
- User-friendly coordinate input format
- Instant distance and timing feedback
- Graceful error handling

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Building

```bash
cd /Users/jackson/Documents/code/project/lsph/examples/demo
cargo build --release
```

## Usage

### Basic Demo

Run the complete demonstration with default settings:

```bash
cargo run --release
```

This will:
1. Load the Melbourne dataset (6,361 points)
2. Build the spatial index
3. Perform 100 nearest neighbor queries
4. Execute 25 range queries with different radii
5. Display comprehensive performance summary

### Command Line Options

```bash
# Custom data file
cargo run --release -- --data custom_data.csv

# Specify number of test queries
cargo run --release -- --queries 500

# Run in interactive mode after demo
cargo run --release -- --interactive

# Skip automated demo, go straight to interactive
cargo run --release -- --skip-demo

# Combine options
cargo run --release -- --queries 1000 --interactive
```

### Interactive Mode

In interactive mode, enter coordinates in the format `lat,lng`:

```
🔍 Query: -37.8136,144.9631
✅ Nearest point: (-37.81360, 144.96310)
📏 Distance: 0.00m | ⏱️  Query time: 12.34μs

🔍 Query: quit
👋 Goodbye!
```

## Data Format

The demo expects CSV data with three columns (no headers):

```csv
latitude,longitude,zone
-37.82341,144.98905,31
-37.82962,144.98793,31
-37.83119,144.98961,31
```

- **Latitude**: Decimal degrees (negative for Southern Hemisphere)
- **Longitude**: Decimal degrees (positive for Eastern Hemisphere)
- **Zone**: Integer category/zone identifier

## Performance Characteristics

### Expected Performance (Melbourne Dataset)

| Metric | Typical Value |
|--------|---------------|
| **Data Loading** | ~5-10ms for 6,361 points |
| **Index Building** | ~15-30ms |
| **Nearest Neighbor** | ~10-50μs per query |
| **Range Queries** | ~20-100μs per query |
| **Memory Usage** | ~0.5-1.0 MB |
| **Throughput** | 20,000-100,000 queries/sec |

*Performance varies based on hardware and data distribution*

### Scalability

LSPH demonstrates excellent scalability characteristics:

- **Sub-linear query time** with dataset size
- **Efficient memory usage** (~100-200 bytes per point)
- **Consistent performance** across different query patterns
- **Fast index construction** (200,000+ points/second)

## Architecture

### Core Components

```rust
// Main demo application
struct LSPHDemo {
    spatial_map: LearnedHashMap<LinearModel<f64>, f64>,
    points: Vec<GeoPoint>,
    stats: PerformanceStats,
}

// Geographic point representation
struct GeoPoint {
    latitude: f64,
    longitude: f64,
    zone: u32,
}

// Performance tracking
struct PerformanceStats {
    data_loading_time: Duration,
    index_building_time: Duration,
    nearest_neighbor_times: Vec<Duration>,
    range_query_times: Vec<Duration>,
    memory_usage_estimate: usize,
}
```

### Key Features

1. **Error Handling**: Comprehensive error handling for file I/O, parsing, and user input
2. **Performance Monitoring**: Detailed timing and memory usage tracking
3. **User Experience**: Colorized output, progress indicators, and clear formatting
4. **Flexibility**: Configurable parameters and multiple operation modes
5. **Documentation**: Extensive inline documentation and help text

## Example Output

```
🗺️  LSPH Geographic Data Demo
Learned Spatial HashMap Performance Demonstration
============================================================

🗺️  Loading Melbourne Geographic Data
==================================================
✅ Loaded 6361 points in 8.45ms

📊 Zone Distribution:
   Zone 31: 6361 points (100.0%)

🏗️  Building Spatial Index
==================================================
✅ Built spatial index in 23.12ms
📈 Successful insertions: 6361
💾 Estimated memory usage: 0.89 MB

🎯 Nearest Neighbor Search Demo
==================================================
🔍 Query 1: (-37.85234, 144.92156) → Nearest: (-37.85240, 144.92160) | Distance: 8.45m | Time: 15.23μs
🔍 Query 2: (-37.78945, 145.02341) → Nearest: (-37.78950, 145.02345) | Distance: 6.12m | Time: 12.67μs
...

📊 Nearest Neighbor Results:
   Successful queries: 100/100
   Average query time: 18.45μs
   Queries per second: 54,200

📈 Performance Summary
==================================================
🗂️  Data Processing:
   Total points processed: 6361
   Data loading time: 8.45ms
   Index building time: 23.12ms
   Points per second (indexing): 275,234

🎯 Nearest Neighbor Performance:
   Average time: 18.45μs
   Min time: 8.23μs
   Max time: 45.67μs
   Queries per second: 54,200

💾 Memory Usage:
   Estimated total: 0.89 MB
   Per point: 143.2 bytes

🎉 Demo completed successfully!
Thank you for exploring LSPH capabilities.
```

## Troubleshooting

### Common Issues

1. **File Not Found**
   ```
   Error: No such file or directory (os error 2)
   ```
   - Ensure `melbourne.csv` is in the demo directory
   - Use `--data` flag to specify custom file path

2. **Parse Errors**
   ```
   ⚠️ Error parsing line 42: invalid float literal
   ```
   - Check CSV format (no headers, three numeric columns)
   - Verify decimal separator (use `.` not `,`)

3. **Memory Issues**
   - For very large datasets, monitor system memory
   - Consider reducing query count with `--queries` flag

### Performance Tips

1. **Use Release Mode**: Always run with `--release` for accurate performance
2. **Warm-up Queries**: First few queries may be slower due to CPU caching
3. **Dataset Size**: Performance scales well, but very large datasets (>1M points) may require more memory
4. **Query Patterns**: Random queries provide good average-case performance

## Contributing

To extend or modify the demo:

1. **Add New Metrics**: Extend `PerformanceStats` struct
2. **Custom Data Sources**: Modify `load_data()` method
3. **Additional Query Types**: Add new demo methods
4. **Visualization**: Consider adding graphical output

## License

This demo is part of the LSPH project and is licensed under MIT OR Apache-2.0.

## Related

- [LSPH Main Documentation](../../README.md)
- [Interactive GUI Demo](../interactive_demo/README.md)
- [Performance Benchmarks](../../benches/)
- [LSPH Paper and Research](../../docs/)