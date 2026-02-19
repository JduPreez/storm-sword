#!/bin/bash
set -e

TARGET="x86_64-unknown-linux-gnu"

echo "Building workspace for Lambda (Linux)..."

# Install cargo-zigbuild if not already installed
if ! command -v cargo-zigbuild &> /dev/null; then
    echo "Installing cargo-zigbuild..."
    cargo install cargo-zigbuild
fi

# Build using zigbuild for cross-compilation
cargo zigbuild --release --target $TARGET

# Create lambda directories
mkdir -p services/apps/public-api/target/lambda/public-api
mkdir -p services/apps/events/target/lambda/events

# Copy binaries and rename to bootstrap
cp target/$TARGET/release/public-api services/apps/public-api/target/lambda/public-api/bootstrap
cp target/$TARGET/release/events services/apps/events/target/lambda/events/bootstrap

touch sst.config.ts

echo "Build complete!"