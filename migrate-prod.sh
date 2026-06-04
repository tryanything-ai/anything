#!/bin/bash

# Production Migration Runner for Anything
# This script applies database migrations to production PostgreSQL

set -e

echo "🚀 Production Migration Runner for Anything"

# Check required environment variables
if [ -z "$DATABASE_URL" ]; then
    echo "❌ DATABASE_URL environment variable is required"
    echo "   Set it to your production PostgreSQL connection string:"
    echo "   export DATABASE_URL='postgresql://user:password@host:port/database'"
    exit 1
fi

# Validate DATABASE_URL format
if [[ ! "$DATABASE_URL" =~ ^postgresql:// ]]; then
    echo "❌ DATABASE_URL must start with 'postgresql://'"
    exit 1
fi

echo "🔧 Using DATABASE_URL: ${DATABASE_URL%@*}@***" # Hide password in logs

# Check if psql is available
if ! command -v psql &> /dev/null; then
    echo "❌ psql command not found. Please install PostgreSQL client:"
    echo "   - Ubuntu/Debian: apt-get install postgresql-client"
    echo "   - CentOS/RHEL: yum install postgresql"
    echo "   - macOS: brew install postgresql"
    echo "   - Docker: Use postgres:15 image"
    exit 1
fi

# Test database connection
echo "🔍 Testing database connection..."
if ! psql "$DATABASE_URL" -c "SELECT 1;" >/dev/null 2>&1; then
    echo "❌ Cannot connect to database. Please check:"
    echo "   - Database server is running"
    echo "   - Connection string is correct"
    echo "   - Network connectivity"
    echo "   - Firewall settings"
    exit 1
fi

echo "✅ Database connection successful"

# Check if pgsodium extension is available
echo "🔍 Checking for pgsodium extension..."
if psql "$DATABASE_URL" -c "CREATE EXTENSION IF NOT EXISTS pgsodium;" >/dev/null 2>&1; then
    echo "✅ pgsodium extension is available"
else
    echo "⚠️  pgsodium extension not available. This is required for secrets encryption."
    echo "   Please install pgsodium on your PostgreSQL server:"
    echo "   - Cloud providers: Check if pgsodium is available as an extension"
    echo "   - Self-hosted: https://github.com/michelp/pgsodium#installation"
    echo ""
    read -p "Continue without pgsodium? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
    USE_SIMPLIFIED=true
fi

# Determine which migration to run
if [ "$USE_SIMPLIFIED" = true ]; then
    MIGRATION_FILE="core/anything-server/migrations/001_setup_simplified.sql"
    echo "⚠️  Using simplified migration (no pgsodium encryption)"
else
    MIGRATION_FILE="core/anything-server/migrations/001_setup_pgsodium_and_auth.sql"
    echo "🔐 Using full migration with pgsodium encryption"
fi

# Check if migration file exists
if [ ! -f "$MIGRATION_FILE" ]; then
    echo "❌ Migration file not found: $MIGRATION_FILE"
    echo "   Please run this script from the project root directory"
    exit 1
fi

# Show migration preview
echo ""
echo "📋 Migration to be applied:"
echo "   File: $MIGRATION_FILE"
echo "   Target: ${DATABASE_URL%@*}@***"
echo ""

# Confirm before applying
read -p "Apply migration to production database? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Migration cancelled"
    exit 1
fi

# Apply migration
echo "🚀 Applying migration..."
if psql "$DATABASE_URL" -f "$MIGRATION_FILE"; then
    echo "✅ Migration applied successfully!"
    echo ""
    echo "🎉 Your production database is ready!"
    echo ""
    echo "Next steps:"
    echo "1. Deploy your application with the same DATABASE_URL"
    echo "2. Set JWT_SECRET environment variable"
    echo "3. Test your application endpoints"
else
    echo "❌ Migration failed!"
    echo ""
    echo "Common issues:"
    echo "- Database permissions (need CREATE, ALTER privileges)"
    echo "- Extension availability (pgsodium)"
    echo "- Schema conflicts (tables already exist)"
    echo ""
    echo "Check the error messages above for details."
    exit 1
fi
