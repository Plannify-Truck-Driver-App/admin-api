#!/bin/bash

# Script pour appliquer les migrations dans Docker
set -e

echo "Attente de la disponibilite de PostgreSQL..."
until pg_isready -h postgres -U plannify_user -d plannify_db; do
  echo "PostgreSQL nest pas encore pret, attente..."
  sleep 2
done

echo "PostgreSQL est pret, execution des migrations..."
echo "Verification de la structure de la base..."
psql -h postgres -U plannify_user -d plannify_db -c '\dt'

echo "Application de toutes les migrations..."
for migration_file in /migrations/*.sql; do
  if [ -f "$migration_file" ]; then
    echo "Application de la migration: $(basename "$migration_file")"
    result=$(psql -h postgres -U plannify_user -d plannify_db -f "$migration_file")
    echo "Migration $(basename "$migration_file") appliquee avec succes"
    echo "$result"
  fi
done

echo "Toutes les migrations ont ete executees !"
echo "Verification finale des tables..."
psql -h postgres -U plannify_user -d plannify_db -c '\dt'
