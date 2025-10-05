#!/bin/bash

# Exit on error
set -e

echo "Starting Tauri development environment setup..."

# Update package lists
echo "Updating package lists..."
sudo apt-get update

# Install system dependencies
echo "Installing system dependencies..."
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# Install Rust
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add cargo to PATH for this script
source "$HOME/.cargo/env"

# Install tauri-cli by building from source to avoid potential timeouts
echo "Cloning tauri repository to build tauri-cli from source..."
git clone https://github.com/tauri-apps/tauri.git
cd tauri/crates/tauri-cli

echo "Building and installing tauri-cli from source..."
cargo install --path .

# Clean up the cloned repository
echo "Cleaning up..."
cd ../../..
rm -rf tauri

echo "Tauri CLI installation complete."
echo "To get started, restart your shell or run the following command:"
echo "source \"\$HOME/.cargo/env\""
