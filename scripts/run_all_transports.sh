#!/bin/bash
# Run all MCP transports in parallel for integration testing/demo
set -e

echo "[INFO] Starting HTTP server..."
cargo run --bin gsheets_mcp_http &
HTTP_PID=$!

echo "[INFO] Starting stdio server..."
cargo run --bin gsheets_mcp_stdio &
STDIO_PID=$!

echo "[INFO] Starting SEE server..."
cargo run --bin gsheets_mcp_see &
SEE_PID=$!

trap "kill $HTTP_PID $STDIO_PID $SEE_PID" SIGINT SIGTERM
wait $HTTP_PID $STDIO_PID $SEE_PID
