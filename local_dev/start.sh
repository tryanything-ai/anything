#!/bin/bash

# Complete Local Development Startup Script
# This script sets up everything needed for local development

set -e

echo "🚀 Starting complete local development environment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}📋 $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "docker-compose.yml" ]; then
    print_error "docker-compose.yml not found. Make sure you're in the project root directory."
    exit 1
fi

# Check if required PostgreSQL files exist
if [ ! -f "Dockerfile.postgres" ]; then
    print_error "Dockerfile.postgres not found. Make sure you're in the project root directory."
    exit 1
fi

if [ ! -f "init-extensions.sql" ]; then
    print_error "init-extensions.sql not found. Make sure you're in the project root directory."
    exit 1
fi

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    print_error "Docker is not running. Please start Docker and try again."
    exit 1
fi

# Set common environment variables
export RUST_LOG=info
export RUST_BACKTRACE=1
export JS_EXECUTOR_URL=http://localhost:50051
export DATABASE_URL=postgresql://postgres:postgres@localhost:5432/anything
export ANYTHING_BASE_URL=http://localhost:3000
export JWT_SECRET=dev-jwt-secret-change-in-production

# Check if we need to build the database image
print_status "Checking if database image needs to be built..."

# Function to check if image needs building
needs_build() {
    # Check if the image exists
    if ! docker image inspect anything_postgres >/dev/null 2>&1; then
        echo "Image doesn't exist"
        return 0
    fi
    
    # Check if Dockerfile.postgres is newer than the image
    if [ "Dockerfile.postgres" -nt <(docker image inspect anything_postgres --format='{{.Created}}' 2>/dev/null | head -1) ]; then
        echo "Dockerfile.postgres is newer than image"
        return 0
    fi
    
    # Check if init-extensions.sql is newer than the image
    if [ "init-extensions.sql" -nt <(docker image inspect anything_postgres --format='{{.Created}}' 2>/dev/null | head -1) ]; then
        echo "init-extensions.sql is newer than image"
        return 0
    fi
    
    return 1
}

# Start the database
if needs_build; then
    print_status "Building PostgreSQL database with custom extensions..."
    print_warning "This may take a few minutes to build the custom PostgreSQL image with pgsodium..."
    docker-compose -f docker-compose.db.yml up -d --build postgres
else
    print_status "Using existing PostgreSQL database image..."
    docker-compose -f docker-compose.db.yml up -d postgres
fi

# Wait for database to be ready
print_status "Waiting for database to be ready..."
timeout=30
while [ $timeout -gt 0 ]; do
    if docker exec anything-postgres pg_isready -U postgres -d anything >/dev/null 2>&1; then
        print_success "Database is ready!"
        break
    fi
    sleep 1
    timeout=$((timeout-1))
done

if [ $timeout -le 0 ]; then
    print_error "Database failed to start"
    exit 1
fi

# Run migrations if needed
if [ -f "local_dev/run-migrations.sh" ]; then
    print_status "Running database migrations..."
    # Give the database a moment to fully initialize after pg_isready
    sleep 2
    
    # Retry migrations up to 3 times in case of connection issues
    retry_count=0
    max_retries=3
    
    while [ $retry_count -lt $max_retries ]; do
        if ./local_dev/run-migrations.sh; then
            break
        else
            retry_count=$((retry_count + 1))
            if [ $retry_count -lt $max_retries ]; then
                print_warning "Migration failed, retrying in 3 seconds... (attempt $retry_count/$max_retries)"
                sleep 3
            else
                print_error "Migration failed after $max_retries attempts"
                exit 1
            fi
        fi
    done
fi

# Seed the database
print_status "Seeding database with test data..."
if [ -f "supabase/seed_seaorm.sql" ]; then
    docker exec -i anything-postgres psql -U postgres -d anything < supabase/seed_seaorm.sql
    print_success "Database seeded successfully!"
else
    print_warning "No seed file found. Skipping seeding."
fi

# Function to cleanup background processes
cleanup() {
    echo "🛑 Shutting down servers..."
    
    # Kill the background processes if they exist
    if [[ -n "$JS_PID" ]]; then
        echo "🔄 Terminating JS Executor (PID: $JS_PID)..."
        kill $JS_PID 2>/dev/null || true
    fi
    if [[ -n "$MAIN_PID" ]]; then
        echo "🔄 Terminating Main Server (PID: $MAIN_PID)..."
        kill $MAIN_PID 2>/dev/null || true
    fi
    
    # Give processes a moment to shut down gracefully
    sleep 1
    
    # Kill any remaining processes on port 3001 (main server)
    echo "🔍 Checking for remaining processes on port 3001..."
    PORT_3001_PIDS=$(lsof -ti:3001 2>/dev/null || true)
    if [[ -n "$PORT_3001_PIDS" ]]; then
        echo "🗡️  Force killing remaining processes on port 3001: $PORT_3001_PIDS"
        echo "$PORT_3001_PIDS" | xargs kill -9 2>/dev/null || true
    else
        echo "✓ No remaining processes on port 3001"
    fi
    
    # Kill any remaining processes on port 50051 (JS executor)
    echo "🔍 Checking for remaining processes on port 50051..."
    PORT_50051_PIDS=$(lsof -ti:50051 2>/dev/null || true)
    if [[ -n "$PORT_50051_PIDS" ]]; then
        echo "🗡️  Force killing remaining processes on port 50051: $PORT_50051_PIDS"
        echo "$PORT_50051_PIDS" | xargs kill -9 2>/dev/null || true
    else
        echo "✓ No remaining processes on port 50051"
    fi
    
    # Stop the database
    echo "🗄️ Stopping PostgreSQL database..."
    docker-compose -f docker-compose.db.yml down --volumes
    
    echo "✅ Cleanup completed"
    exit 0
}

# Trap cleanup function on script exit
trap cleanup EXIT INT TERM

# Build and start the application servers
print_status "Building JavaScript Executor..."
cd core/js-server
cargo build
print_success "JS Executor built successfully"

print_status "Building Main Server..."
cd ../anything-server
cargo build
print_success "Main Server built successfully"

print_status "Starting JavaScript Executor on port 50051..."
cd ../js-server
cargo run &
JS_PID=$!

# Wait for JS executor to start
sleep 3

print_status "Starting Main Server on port 3001..."
cd ../anything-server
cargo run &
MAIN_PID=$!

print_success "Complete local development environment is ready!"
echo ""
echo "🎉 Everything is running:"
echo "   • Database: localhost:5432 (anything) - Custom PostgreSQL with pgsodium"
echo "   • Main Server: http://localhost:3001"
echo "   • JS Executor: localhost:50051 (gRPC)"
echo "   • Test data: 4 users, 3 organizations"
echo ""
echo "📊 Test Users:"
echo "   • user1@example.com (user1) - Owner of Acme Corp"
echo "   • user2@example.com (user2) - Owner of Tech Startup"
echo "   • user3@example.com (user3) - Member of Acme Corp"
echo "   • admin@example.com (admin) - Admin of all accounts"
echo ""
echo "🛠️  Next steps:"
echo "   • Start your web app: npm run dev (or similar)"
echo "   • Check logs: docker-compose logs -f"
echo "   • Stop everything: Ctrl+C or ./local_dev/stop.sh"
echo ""
echo "🔄 Press Ctrl+C to stop all servers and database"

# Wait for both processes
wait $JS_PID $MAIN_PID
