#!/bin/bash
set -e

echo "NDP-Mutter Quickstart"
echo "====================="

# Check basics
if ! command -v cargo &> /dev/null; then
    echo "Rust/Cargo not found. Please install Rust: https://rustup.rs/"
    exit 1
fi

if ! command -v pkg-config &> /dev/null; then
    echo "pkg-config not found. Please install it."
    exit 1
fi

# Check libs (best effort)
if ! pkg-config --exists gstreamer-1.0; then
    echo "WARNING: gstreamer-1.0 development libraries missing."
    echo "You may need to install: libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev"
    echo "Build might fail."
fi

echo "Building workspace..."
cargo build

echo "Running Diagnostics..."
cargo run -p ndp-inspect

echo ""
echo "Setup complete (if no errors above)."
echo "See README.md for usage."
