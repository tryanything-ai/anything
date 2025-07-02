#!/bin/bash

# Development startup script for Anything servers

set -e

echo "🚀 Starting Anything Development Servers"

# Set common environment variables
export RUST_LOG=info
export RUST_BACKTRACE=1
export JS_EXECUTOR_URL=http://localhost:50051

# Function to cleanup background processes
cleanup() {
    echo "🛑 Shutting down servers..."
    kill $JS_PID $MAIN_PID 2>/dev/null || true
    exit 0
}

# Trap cleanup function on script exit
trap cleanup EXIT INT TERM

echo "📦 Building JavaScript Executor (debug mode)..."
cd core/js-server
cargo build
echo "✅ JS Executor built successfully"

echo "📦 Building Main Server (debug mode)..."
cd ../anything-server
cargo build
echo "✅ Main Server built successfully"

echo "🟢 Starting JavaScript Executor on port 50051..."
cd ../js-server
cargo run &
JS_PID=$!

# Wait for JS executor to start
sleep 3

echo "🟢 Starting Main Server on port 3001..."
cd ../anything-server
cargo run &
MAIN_PID=$!

echo "✅ Both servers started successfully!"
echo "📍 Main Server: http://localhost:3001"
echo "📍 JS Executor: localhost:50051 (gRPC)"
echo "🔄 Press Ctrl+C to stop both servers"

# Wait for both processes
wait $JS_PID $MAIN_PID 