#!/bin/bash

# Database Seeding Script for Local Development
# This script populates the database with test data

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

# Check if seed file exists
if [ ! -f "supabase/seed_seaorm.sql" ]; then
    echo "❌ Error: seed_seaorm.sql not found."
    exit 1
fi

print_status "Seeding database with test data..."

# Run the seed script
docker exec -i anything-postgres psql -U postgres -d anything < supabase/seed_seaorm.sql

print_success "Database seeded successfully!"

# Show a summary of what was created
echo ""
echo "📊 Test data created:"
echo "   • 4 test users (user1, user2, user3, admin)"
echo "   • 3 organizations (Acme Corp, Tech Startup, Freelance)"
echo "   • User-account relationships with different roles"
echo "   • Sample secrets, tasks, files, and billing data"
echo ""
print_warning "Note: Password hashes are placeholders - implement proper auth in your app"
