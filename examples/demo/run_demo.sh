#!/bin/bash

# LSPH Geographic Data Demo Runner
# This script builds and runs the geographic data demonstration

echo "🗺️  LSPH Geographic Data Demo"
echo "=============================="
echo ""

# Check if melbourne.csv exists
if [ ! -f "melbourne.csv" ]; then
    echo "⚠️  Warning: melbourne.csv not found in current directory"
    echo "   The demo will attempt to load the file, but may fail."
    echo "   Make sure the CSV file is in the demo directory."
    echo ""
fi

# Build the demo
echo "🔨 Building demo application..."
if cargo build --release; then
    echo "✅ Build successful!"
    echo ""
else
    echo "❌ Build failed! Please check the error messages above."
    exit 1
fi

# Display usage options
echo "🚀 Starting LSPH Demo"
echo "===================="
echo ""
echo "Available options:"
echo "  1. Full demo (default)"
echo "  2. Interactive mode only"
echo "  3. Custom parameters"
echo ""
read -p "Choose option (1-3) or press Enter for default: " choice

case $choice in
    2)
        echo "🎮 Starting interactive mode..."
        cargo run --release -- --skip-demo
        ;;
    3)
        echo "📝 Custom parameters:"
        read -p "Number of queries (default 100): " queries
        read -p "Run interactive mode after? (y/n): " interactive
        
        args=""
        if [ ! -z "$queries" ]; then
            args="$args --queries $queries"
        fi
        if [ "$interactive" = "y" ] || [ "$interactive" = "Y" ]; then
            args="$args --interactive"
        fi
        
        echo "🚀 Running with custom parameters..."
        cargo run --release -- $args
        ;;
    *)
        echo "🚀 Running full demo..."
        cargo run --release
        ;;
esac

echo ""
echo "🎉 Demo completed!"
echo "Thank you for exploring LSPH capabilities."