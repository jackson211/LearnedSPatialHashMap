#!/bin/bash

# LSPH Interactive Demo Runner
# This script builds and runs the interactive demo

echo "🗺️  Building LSPH Interactive Demo..."
echo "======================================"

# Build the demo
if cargo build --release; then
    echo "✅ Build successful!"
    echo ""
    echo "🚀 Starting LSPH Interactive Demo..."
    echo "====================================="
    echo ""
    echo "📋 Demo Features:"
    echo "  • Manual Point Addition - Click to add points"
    echo "  • Random Generation - Auto-generate test data"
    echo "  • Nearest Neighbor Search - Find closest points"
    echo "  • Range Query - Search within radius"
    echo ""
    echo "💡 Tips:"
    echo "  • Start with 'Random Generation' mode"
    echo "  • Generate 100 points for best experience"
    echo "  • Try different search modes by clicking on canvas"
    echo "  • Enable grid for better spatial reference"
    echo ""
    echo "Press Ctrl+C to exit the demo"
    echo ""
    
    # Run the demo
    cargo run --release
else
    echo "❌ Build failed! Please check the error messages above."
    exit 1
fi