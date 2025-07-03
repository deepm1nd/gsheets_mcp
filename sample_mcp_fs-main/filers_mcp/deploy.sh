#!/bin/bash
# Example deployment script for filers_mcp
set -e

# Build the project
cargo build --release

# Create config directory if needed
mkdir -p /etc/filers_mcp

# Copy binary and config
cp target/release/filers_mcp /usr/local/bin/filers_mcp
cp example_config.toml /etc/filers_mcp/config.toml

echo "Deployment complete."
