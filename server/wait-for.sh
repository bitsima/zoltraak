#!/bin/bash

# Wait for PostgreSQL to be available
until pg_isready -h db -U "$POSTGRES_USER" -d "$POSTGRES_DB"; do
    echo >&2 "PostgreSQL is unavailable - sleeping"
    sleep 1
done

echo >&2 "PostgreSQL is up - executing command"
exec "$@"
