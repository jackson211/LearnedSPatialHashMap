# LSPH Interactive Demo

An interactive graphical demonstration of the Learned Spatial HashMap (LSPH) capabilities.

## Features

This demo showcases the core functionality of LSPH through an intuitive graphical interface:

### 🎯 Demo Modes

1. **Manual Point Addition**
   - Add points by clicking on the canvas or entering coordinates
   - Specify custom values for each point
   - Visual feedback with color-coded points

2. **Random Generation**
   - Generate random points automatically
   - Adjustable generation speed
   - Batch generation (10 or 100 points)
   - Auto-generation mode for continuous demonstration

3. **Nearest Neighbor Search**
   - Click anywhere to find the nearest point
   - Visual highlighting of the nearest neighbor
   - Real-time search performance metrics
   - Interactive query point positioning

4. **Range Query**
   - Define a search center and radius
   - Visual circle showing the search area
   - Highlight all points within the specified range
   - Adjustable search radius with slider

### 🎨 Visualization Features

- **Interactive Canvas**: Click to add points or perform searches
- **Color-coded Points**: Points are colored based on their values
- **Grid Overlay**: Optional grid for better spatial reference
- **Adjustable Point Size**: Customize point visualization
- **Real-time Highlighting**: Visual feedback for search results
- **Performance Metrics**: Display search times and point counts

### 📊 Statistics

- Total number of points in the spatial map
- Search operation timing (in milliseconds)
- Real-time updates during interactions

## Running the Demo

### Prerequisites

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

### Installation and Execution

1. Navigate to the demo directory:
   ```bash
   cd /Users/jackson/Documents/code/project/lsph/examples/interactive_demo
   ```

2. Run the demo:
   ```bash
   cargo run
   ```

The application will open in a new window with the interactive interface.

## How to Use

### Getting Started

1. **Start with Random Generation**: Select "Random Generation" mode and click "Generate 100 Points" to populate the canvas
2. **Try Nearest Neighbor**: Switch to "Nearest Neighbor Search" mode and click anywhere on the canvas
3. **Explore Range Queries**: Use "Range Query" mode to find all points within a specified radius
4. **Manual Addition**: Add specific points using "Manual Point Addition" mode

### Tips for Best Experience

- **Use the grid**: Enable "Show Grid" for better spatial reference
- **Adjust point size**: Increase point size for better visibility with many points
- **Try different modes**: Each mode demonstrates different LSPH capabilities
- **Watch performance**: Notice how search times remain fast even with many points
- **Interactive exploration**: Click around the canvas to see real-time search results

## Technical Details

### Dependencies

- **LSPH**: The core spatial hashmap library
- **eframe/egui**: Modern immediate-mode GUI framework for Rust
- **rand**: Random number generation for demo data
- **serde**: Serialization support (future feature)

### Architecture

The demo is built using the egui immediate-mode GUI framework, providing:
- Cross-platform compatibility (Windows, macOS, Linux)
- Smooth 60fps rendering
- Responsive user interface
- Real-time visualization updates

### Performance Characteristics

The demo demonstrates LSPH's key performance benefits:
- **Fast Insertions**: Add thousands of points quickly
- **Efficient Searches**: Nearest neighbor and range queries in sub-millisecond time
- **Memory Efficiency**: Compact spatial indexing
- **Scalability**: Performance remains consistent as data grows

## Educational Value

This demo is designed to help users understand:

1. **Spatial Data Structures**: How points are organized in 2D space
2. **Search Algorithms**: Visual representation of nearest neighbor and range queries
3. **Performance Benefits**: Real-time metrics showing LSPH's efficiency
4. **Interactive Learning**: Hands-on exploration of spatial algorithms

## Extending the Demo

The demo can be extended with additional features:

- **3D Visualization**: Extend to 3D point clouds
- **Data Import/Export**: Load real-world datasets
- **Algorithm Comparison**: Side-by-side comparison with other spatial structures
- **Advanced Queries**: k-nearest neighbors, polygon queries
- **Animation**: Animated insertions and searches
- **Benchmarking**: Built-in performance testing suite

## Troubleshooting

### Common Issues

1. **Compilation Errors**: Ensure you have the latest Rust toolchain
2. **Missing Dependencies**: Run `cargo update` to fetch latest dependencies
3. **Performance Issues**: Try reducing the number of points or disabling auto-generation

### System Requirements

- **Rust**: 1.70 or later
- **Graphics**: OpenGL 3.0+ support
- **Memory**: 512MB RAM minimum
- **Platform**: Windows 10+, macOS 10.15+, or Linux with X11/Wayland

## License

This demo is part of the LSPH project and is licensed under MIT OR Apache-2.0.