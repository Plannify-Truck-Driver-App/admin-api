#!/bin/bash

set -e

echo "Waiting for PostgreSQL to be ready..."
until pg_isready -h postgres -U plannify_user -d plannify_db; do
  echo "PostgreSQL is not ready yet, waiting..."
  sleep 2
done

echo "PostgreSQL is ready, running migrations..."
echo "Checking database structure..."
psql -h postgres -U plannify_user -d plannify_db -c '\dt'

echo "Applying all migrations..."
for migration_file in /migrations/*.sql; do
  if [ -f "$migration_file" ]; then
    echo "Applying migration: $(basename "$migration_file")"
    result=$(psql -h postgres -U plannify_user -d plannify_db -f "$migration_file")
    echo "Migration $(basename "$migration_file") applied successfully"
    echo "$result"
  fi
done

echo "All migrations have been executed!"
echo "Final table verification..."
psql -h postgres -U plannify_user -d plannify_db -c '\dt'
