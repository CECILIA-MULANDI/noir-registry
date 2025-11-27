#!/bin/bash
# Script to run database migrations using sqlx-cli

# Load environment variables if .env exists
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "Error: DATABASE_URL environment variable is not set"
    echo "Please set it in your .env file or export it:"
    echo "  export DATABASE_URL='postgresql://user:password@host:port/database'"
    exit 1
fi

# Run migrations using sqlx-cli
echo "Running database migrations..."
sqlx migrate run

if [ $? -eq 0 ]; then
    echo "✅ Migrations completed successfully!"
else
    echo "❌ Migration failed!"
    exit 1
fi

