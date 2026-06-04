#!/bin/bash

# Database Migration Script for Local Development
# This script runs database migrations

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}📋 $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Check if database is running
if ! docker ps | grep -q "anything-postgres"; then
    echo "❌ Error: Database container 'anything-postgres' is not running."
    echo "Please start your database first with: ./local_dev/start.sh"
    exit 1
fi

print_status "Running database migrations..."

# Run the initialization SQL script first to set up basic schema
if [ -f "core/anything-server/migrations/001_setup_pgsodium_and_auth.sql" ]; then
    print_status "Running initialization SQL script..."
    docker exec -i anything-postgres psql -U postgres -d anything < core/anything-server/migrations/001_setup_pgsodium_and_auth.sql
    print_success "Initialization SQL completed!"
fi

# Run Supabase migrations in order to set up the complete schema
if [ -d "supabase/migrations" ]; then
    print_status "Running Supabase migrations..."
    
    # Run migrations in chronological order (by filename)
    for migration_file in $(ls supabase/migrations/*.sql | sort); do
        if [ -f "$migration_file" ]; then
            print_status "Running migration: $(basename "$migration_file")"
            docker exec -i anything-postgres psql -U postgres -d anything < "$migration_file"
        fi
    done
    print_success "Supabase migrations completed!"
fi

# Try SeaORM migrations (optional - skip if it fails)
if [ -d "core/anything-server/migration" ]; then
    print_status "Attempting SeaORM migrations..."
    cd core/anything-server/migration
    
    # Ensure DATABASE_URL is set for the migration tool
    export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/anything"
    
    if cargo run up; then
        print_success "SeaORM migrations completed!"
    else
        print_warning "SeaORM migration failed, but continuing..."
    fi
    
    cd ../../..
else
    print_warning "No SeaORM migration directory found. Skipping migrations."
fi

print_success "Migrations completed!"
