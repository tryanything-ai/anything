#!/bin/bash

# SeaORM Migration CLI for Anything
# This provides a Rust-based migration runner using SeaORM

set -e

echo "🦀 SeaORM Migration CLI for Anything"

# Check if we're in the right directory
if [ ! -f "migration/Cargo.toml" ]; then
    echo "❌ Please run this script from core/anything-server directory"
    exit 1
fi

# Check required environment variables
if [ -z "$DATABASE_URL" ]; then
    echo "❌ DATABASE_URL environment variable is required"
    echo "   Set it to your PostgreSQL connection string:"
    echo "   export DATABASE_URL='postgresql://user:password@host:port/database'"
    exit 1
fi

echo "🔧 Using DATABASE_URL: ${DATABASE_URL%@*}@***" # Hide password in logs

# Parse command
COMMAND=${1:-"help"}

case $COMMAND in
    "up")
        echo "🚀 Running migrations..."
        cd migration && cargo run -- up
        ;;
    "down")
        echo "⬇️  Rolling back last migration..."
        cd migration && cargo run -- down
        ;;
    "status")
        echo "📊 Checking migration status..."
        cd migration && cargo run -- status
        ;;
    "fresh")
        echo "🔄 Dropping all tables and re-running migrations..."
        cd migration && cargo run -- fresh
        ;;
    "reset")
        echo "🔄 Rolling back all migrations and re-running them..."
        cd migration && cargo run -- reset
        ;;
    "generate")
        if [ -z "$2" ]; then
            echo "❌ Please provide a migration name"
            echo "   Usage: ./migrate-cli.sh generate create_users_table"
            exit 1
        fi
        echo "📝 Generating new migration: $2"
        cd migration && sea-orm-cli migrate generate "$2"
        ;;
    "help"|*)
        echo ""
        echo "Usage: ./migrate-cli.sh <command>"
        echo ""
        echo "Commands:"
        echo "  up        - Run pending migrations"
        echo "  down      - Rollback the last migration"
        echo "  status    - Show migration status"
        echo "  fresh     - Drop all tables and re-run migrations"
        echo "  reset     - Rollback all migrations and re-run them"
        echo "  generate  - Generate a new migration file"
        echo "  help      - Show this help message"
        echo ""
        echo "Examples:"
        echo "  ./migrate-cli.sh up"
        echo "  ./migrate-cli.sh generate add_user_preferences"
        echo "  ./migrate-cli.sh status"
        echo ""
        ;;
esac
