#!/bin/bash
# Helper script to obfuscate source code using rust-obfuscator

set -e

OBFUSCATOR_REPO="https://github.com/dronavallipranav/rust-obfuscator.git"
OBFUSCATOR_DIR="rust-obfuscator-temp"
OBFUSCATOR_BIN="rust-obfuscator"

echo "==================================="
echo "Funsomware Source Code Obfuscator"
echo "==================================="
echo ""

# Check if rust-obfuscator binary exists
if [ ! -f "$OBFUSCATOR_BIN" ]; then
    echo "rust-obfuscator not found. Installing..."
    
    # Clone the repository
    if [ -d "$OBFUSCATOR_DIR" ]; then
        echo "Removing old clone..."
        rm -rf "$OBFUSCATOR_DIR"
    fi
    
    echo "Cloning rust-obfuscator repository..."
    git clone "$OBFUSCATOR_REPO" "$OBFUSCATOR_DIR"
    
    # Build the tool
    echo "Building rust-obfuscator..."
    cd "$OBFUSCATOR_DIR"
    cargo build --release --bin rust-obfuscator
    
    # Copy binary to project root
    cp "target/release/$OBFUSCATOR_BIN" "../$OBFUSCATOR_BIN"
    cd ..
    
    # Cleanup
    echo "Cleaning up..."
    rm -rf "$OBFUSCATOR_DIR"
    
    echo "rust-obfuscator installed successfully!"
    echo ""
else
    echo "rust-obfuscator found."
    echo ""
fi

# Backup source code
echo "Creating backup of src/ directory..."
if [ -d "src_backup" ]; then
    echo "Warning: src_backup already exists. Removing old backup..."
    rm -rf src_backup
fi
cp -r src src_backup
echo "Backup created: src_backup/"
echo ""

# Run obfuscator
echo "Running rust-obfuscator on src/ directory..."
./"$OBFUSCATOR_BIN" src "$@"
echo ""

# Check and reorganize obfuscated code
if [ -d "obfuscated_code" ]; then
    echo "Checking obfuscated code structure..."
    
    # rust-obfuscator writes .rs files directly to obfuscated_code/, not obfuscated_code/src/
    # We need to create the src/ subdirectory structure
    if [ ! -d "obfuscated_code/src" ]; then
        echo "Reorganizing obfuscated files..."
        mkdir -p obfuscated_code/src
        mv obfuscated_code/*.rs obfuscated_code/src/ 2>/dev/null || true
    fi
    
    echo "Formatting obfuscated code..."
    # Create a temporary Cargo.toml in obfuscated_code for formatting
    cp Cargo.toml obfuscated_code/
    cd obfuscated_code
    cargo fmt --all 2>/dev/null || true
    rm Cargo.toml
    cd ..
    echo ""
    
    echo "Obfuscated code generated in: obfuscated_code/"
    echo ""
    echo "Next steps:"
    echo "  1. Review the obfuscated code in obfuscated_code/src/"
    echo "  2. To use it, run: rm -rf src && mv obfuscated_code/src src"
    echo "  3. To restore original: rm -rf src && mv src_backup src"
    echo ""
else
    echo "Error: obfuscated_code directory not created!"
    exit 1
fi

echo "Done!"
