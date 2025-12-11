#!/bin/bash

# Database setup script for Chunk API

echo "Creating database..."
psql -U postgres -c "CREATE DATABASE chunk_db;"

echo "Running migrations..."
psql -U postgres -d chunk_db -f migrations/001_create_chunks_table.sql

echo "Database setup complete!"
