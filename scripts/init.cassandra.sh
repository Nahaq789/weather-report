#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CQL_FILE="${SCRIPT_DIR}/../cassandra/initdb.cql"

if [ ! -f "$CQL_FILE" ]; then
  echo "Error: File $CQL_FILE does not exist"
  exit 1
fi

echo "Found CQL file at $CQL_FILE"

if ! docker ps | grep -q cassandra; then
  echo "Error: Cassandra container is not running"
  exit 1
fi

echo "Copying initdb.cql to container..."
docker cp "$CQL_FILE" cassandra:/initdb.cql

echo "Waiting for Cassandra to be ready..."
until docker exec -i cassandra cqlsh -e "describe keyspaces" > /dev/null 2>&1; do
  echo "Waiting..."
  sleep 5
done

echo "Executing CQL script..."
docker exec -i cassandra cqlsh -f /initdb.cql

echo "Initialization complete"
