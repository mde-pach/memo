#!/bin/bash

# Exit on errors
set -e

# Project name (change this if your binary name is different)
PROJECT_NAME="memo"

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if Rust and Cargo are installed
if ! command_exists cargo; then
    echo "Error: Cargo (Rust) is not installed."
    echo "Please install Rust from https://rustup.rs/."
    exit 1
fi

# Build the project in release mode
echo "Building the project in release mode..."
cargo build --release

# Check if the binary was created successfully
if [[ ! -f "target/release/$PROJECT_NAME" ]]; then
    echo "Error: Build failed or binary not found!"
    exit 1
fi

# Copy the binary to /usr/local/bin (requires sudo)
echo "Installing the binary to /usr/local/bin..."
sudo cp "target/release/$PROJECT_NAME" /usr/local/bin/

# Set executable permissions (just in case)
sudo chmod +x /usr/local/bin/$PROJECT_NAME

# Clean up build artifacts (optional)
echo "Cleaning up build artifacts..."
cargo clean

echo "$PROJECT_NAME has been successfully installed!"
echo "You can now run the project with the command: $PROJECT_NAME"
