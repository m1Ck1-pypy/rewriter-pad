#!/bin/bash

# Build script for rewriter-pad for all platforms
# This script is designed to run on appropriate platforms for cross-compilation

set -e  # Exit on any error

echo "Building rewriter-pad for all platforms..."

# Create output directory
mkdir -p ./build-output

# Build for current platform (Windows from Windows)
echo "Building for Windows (x86_64-pc-windows-msvc)..."
cargo build --release --target=x86_64-pc-windows-msvc --bin RewriterPad
if [ $? -eq 0 ]; then
    cp ./target/x86_64-pc-windows-msvc/release/RewriterPad.exe ./build-output/
    echo "✓ Windows build completed successfully"
else
    echo "✗ Windows build failed"
fi

# Cross-compile for Linux
echo "Building for Linux (x86_64-unknown-linux-musl)..."
# This requires musl-tools to be installed on the system
if command -v musl-gcc &> /dev/null; then
    cargo build --release --target=x86_64-unknown-linux-musl --bin RewriterPad
    if [ $? -eq 0 ]; then
        mv ./target/x86_64-unknown-linux-musl/release/RewriterPad ./build-output/RewriterPad-linux
        echo "✓ Linux build completed successfully"
    else
        echo "✗ Linux build failed"
    fi
else
    echo "✗ musl-gcc not found. Install musl-tools to cross-compile for Linux."
    echo "  On Ubuntu/Debian: sudo apt install musl-tools"
    echo "  On Alpine: apk add musl-dev"
fi

# Cross-compile for macOS
echo "Building for macOS (x86_64-apple-darwin)..."
# This requires macOS SDK to be available
if xcrun --show-sdk-path &> /dev/null; then
    cargo build --release --target=x86_64-apple-darwin --bin RewriterPad
    if [ $? -eq 0 ]; then
        mv ./target/x86_64-apple-darwin/release/RewriterPad ./build-output/RewriterPad-macos
        echo "✓ macOS build completed successfully"
    else
        echo "✗ macOS build failed"
    fi
else
    echo "✗ macOS SDK not found. Cross-compiling for macOS requires Xcode tools."
fi

echo "Build process completed. Check ./build-output/ directory for binaries."