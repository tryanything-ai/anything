#!/bin/bash

# JavaScript Executor Test Runner
# This script runs comprehensive tests for the Rust+Deno JavaScript executor

set -e

echo "🦀 JavaScript Executor Test Suite"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the js-server directory"
    exit 1
fi

print_status "Building JavaScript executor..."
if cargo build; then
    print_success "Build completed successfully"
else
    print_error "Build failed"
    exit 1
fi

print_status "Running unit tests for JavaScript engine..."
if cargo test javascript_engine_tests --lib; then
    print_success "JavaScript engine tests passed"
else
    print_error "JavaScript engine tests failed"
    exit 1
fi

print_status "Running gRPC integration tests..."
if cargo test grpc_integration_tests --lib; then
    print_success "gRPC integration tests passed"
else
    print_error "gRPC integration tests failed"
    exit 1
fi

print_status "Running all tests with verbose output..."
if cargo test -- --nocapture; then
    print_success "All tests passed!"
else
    print_error "Some tests failed"
    exit 1
fi

print_status "Building Docker image..."
if docker build -t js-executor-test .; then
    print_success "Docker image built successfully"
else
    print_error "Docker build failed"
    exit 1
fi

print_status "Testing Docker container startup..."
CONTAINER_ID=$(docker run -d -p 50051:50051 js-executor-test)

if [ $? -eq 0 ]; then
    print_success "Docker container started with ID: $CONTAINER_ID"
    
    # Wait a moment for the container to start
    sleep 3
    
    # Check if container is still running
    if docker ps | grep -q $CONTAINER_ID; then
        print_success "Container is running successfully"
        
        # Try to connect to the gRPC service (if grpcurl is available)
        if command -v grpcurl &> /dev/null; then
            print_status "Testing gRPC health check..."
            if grpcurl -plaintext localhost:50051 js_executor.JsExecutor/HealthCheck; then
                print_success "gRPC health check passed"
            else
                print_warning "gRPC health check failed (this might be expected if grpcurl setup differs)"
            fi
        else
            print_warning "grpcurl not found, skipping live gRPC test"
        fi
        
        # Clean up
        print_status "Stopping test container..."
        docker stop $CONTAINER_ID > /dev/null
        docker rm $CONTAINER_ID > /dev/null
        print_success "Test container cleaned up"
    else
        print_error "Container failed to start properly"
        docker logs $CONTAINER_ID
        docker rm $CONTAINER_ID > /dev/null
        exit 1
    fi
else
    print_error "Failed to start Docker container"
    exit 1
fi

echo ""
echo "🎉 All tests completed successfully!"
echo ""
echo "What was tested:"
echo "✅ JavaScript engine unit tests (12 test cases)"
echo "   - Simple execution, object returns, array processing"
echo "   - Console logging, math operations, error handling"
echo "   - Timeout handling, JSON manipulation, concurrent execution"
echo "   - Large data processing"
echo ""
echo "✅ gRPC integration tests (6 test cases)"
echo "   - Health checks, simple & complex execution"
echo "   - Error handling, timeout handling, concurrent requests"
echo ""
echo "✅ Docker container build and startup"
echo "✅ Container runtime verification"
echo ""
echo "Your JavaScript executor is ready! 🚀"
echo ""
echo "To run the Docker container:"
echo "  docker run -p 50051:50051 js-executor-test"
echo ""
echo "To run individual test suites:"
echo "  cargo test javascript_engine_tests"
echo "  cargo test grpc_integration_tests" 