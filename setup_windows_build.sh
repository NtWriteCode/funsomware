#!/bin/bash
# Setup script for cross-compiling to Windows from Linux

echo "==================================="
echo "Windows Cross-Compilation Setup"
echo "==================================="
echo ""

# Check if running on Linux
if [[ "$OSTYPE" != "linux-gnu"* ]]; then
    echo "This script is for Linux only."
    echo "On Windows, just run: cargo build --release"
    exit 1
fi

# Install MinGW cross-compiler
echo "Installing MinGW cross-compiler..."
if command -v apt &> /dev/null; then
    # Debian/Ubuntu
    sudo apt update
    sudo apt install -y mingw-w64
elif command -v dnf &> /dev/null; then
    # Fedora/RHEL
    sudo dnf install -y mingw64-gcc
elif command -v pacman &> /dev/null; then
    # Arch
    sudo pacman -S --noconfirm mingw-w64-gcc
else
    echo "Unsupported package manager. Please install mingw-w64 manually."
    exit 1
fi

echo ""
echo "Adding Windows target to Rust..."
rustup target add x86_64-pc-windows-gnu

echo ""
echo "==================================="
echo "Setup complete!"
echo "==================================="
echo ""
echo "You can now build for Windows with:"
echo "  cargo build --release --target x86_64-pc-windows-gnu"
echo ""
echo "The binary will be in:"
echo "  target/x86_64-pc-windows-gnu/release/funsomware.exe"
echo ""
