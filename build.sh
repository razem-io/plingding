#!/bin/bash

# Ensure the script exits if any command fails
set -e

# Create a directory for the releases
mkdir -p releases

# Build for Linux (current platform)
echo "Building for Linux..."
cargo build --release
cp target/release/plingding releases/plingding-linux
cp plingding.yaml.example releases/plingding-linux.yaml.example

# Build for Windows
echo "Building for Windows..."
cross build --target x86_64-pc-windows-gnu --release
cp target/x86_64-pc-windows-gnu/release/plingding.exe releases/plingding-windows.exe
cp plingding.yaml.example releases/plingding-windows.yaml.example

echo "Build complete. Binaries and example configuration files are in the 'releases' directory."
