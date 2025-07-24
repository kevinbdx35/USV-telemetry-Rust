#!/bin/bash

# USV Telemetry System Build Script
echo "🚤 Building USV Telemetry System..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if rust is installed
if ! command -v rustc &> /dev/null; then
    print_error "Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if wasm32-unknown-unknown target is added
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    print_status "Adding wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
    if [ $? -eq 0 ]; then
        print_success "wasm32-unknown-unknown target added"
    else
        print_error "Failed to add wasm32-unknown-unknown target"
        exit 1
    fi
fi

# Build the project
print_status "Building Rust project for WebAssembly..."
cargo build --target wasm32-unknown-unknown --release

if [ $? -eq 0 ]; then
    print_success "Build completed successfully!"
    
    # Check if WASM file exists
    WASM_FILE="target/wasm32-unknown-unknown/release/usv_telemetry.wasm"
    if [ -f "$WASM_FILE" ]; then
        WASM_SIZE=$(du -h "$WASM_FILE" | cut -f1)
        print_success "WASM file generated: $WASM_FILE (${WASM_SIZE})"
    else
        print_error "WASM file not found!"
        exit 1
    fi
    
    # Check if all required files exist
    print_status "Checking required files..."
    
    files_to_check=("index.html" "assets/styles.css" "serve.py")
    all_files_exist=true
    
    for file in "${files_to_check[@]}"; do
        if [ -f "$file" ]; then
            print_success "✓ $file"
        else
            print_error "✗ $file (missing)"
            all_files_exist=false
        fi
    done
    
    if [ "$all_files_exist" = true ]; then
        print_success "All required files are present!"
        echo ""
        echo "🎉 Build completed successfully!"
        echo ""
        echo "📋 Next steps:"
        echo "   1. Run the development server: python3 serve.py"
        echo "   2. Open your browser to: http://localhost:8000"
        echo "   3. Enjoy your USV Telemetry Dashboard!"
        echo ""
        echo "📁 Project structure:"
        echo "   ├── index.html           (Main HTML file)"
        echo "   ├── assets/styles.css    (CSS styles)"
        echo "   ├── $WASM_FILE (WebAssembly binary)"
        echo "   └── serve.py             (Development server)"
        echo ""
    else
        print_error "Some required files are missing!"
        exit 1
    fi
    
else
    print_error "Build failed!"
    print_error "Please check the error messages above and fix any issues."
    exit 1
fi