# Development environment

Test databases for NineSQL's drivers ([ADR-0009](../docs/adr/0009-container-first-test-environment.md)). **Status: planned in P0-C** — `compose.yaml`, `engines.json` and the seed data do not exist yet.

## Planned usage
```bash
docker compose -f dev/compose.yaml --profile core up -d --wait    # PostgreSQL, MySQL, MariaDB
docker compose -f dev/compose.yaml --profile sql up -d --wait     # + TimescaleDB, YugabyteDB, QuestDB, TiDB, ClickHouse, Trino, Firebird, InfluxDB 3
docker compose -f dev/compose.yaml --profile nosql up -d --wait   # MongoDB, Redis, Valkey
docker compose -f dev/compose.yaml --profile heavy up -d --wait   # Elasticsearch, OpenSearch, Cassandra, Neo4j (~8 GB RAM)
docker compose -f dev/compose.yaml --profile all down -v
```

## Rules
- Official images, pinned by digest; every port bound to `127.0.0.1` only.
- Fake data only (the "nine" sample dataset: customers, orders, products, an all-types table, a 1 M-row table generated at startup).
- Dev-only credentials, documented here once P0-C lands. Never reuse them anywhere else.
- `engines.json` is the source of truth for ports and readiness checks; driver integration tests read it.

## Without Docker (some cloud agent sandboxes)
Work on unit tests and the file-based engines (SQLite, DuckDB), and rely on CI for server engines. Say so in the task report ([docs/09-agent-workflow.md](../docs/09-agent-workflow.md)).

## Integration test switches
- Tests skip an unreachable engine by default.
- `NINESQL_REQUIRE_ENGINES=postgresql,mysql` turns skips into failures for the listed engines (CI sets it per job).
