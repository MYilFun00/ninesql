# ADR-0009: Container-first test environment, runnable in CI and cloud agent sandboxes

- **Status**: Accepted
- **Date**: 2026-11-14

## Context
Twenty engines must be tested for real (ADR-0003). Much of the work is done by AI coding agents, locally and in Claude Code cloud sandboxes (Linux containers without a display or GPU, whose ability to run Docker and reach the network depends on the environment's configuration). GitHub Actions `ubuntu-latest` runners can run Docker.

## Decision
1. **`dev/compose.yaml`** declares one service per engine, using **official images pinned by digest**, bound to `127.0.0.1` only, with fake seed data (`dev/seed/<engine>/`) and a common schema (the "nine" sample database: customers, orders, products, a wide table, a table with every common type, a 1 000 000-row table generated at startup).
2. **Profiles** keep it usable on small machines:
   - `core`: PostgreSQL, MySQL, MariaDB (SQLite and DuckDB are files, no service).
   - `sql`: `core` + TimescaleDB, YugabyteDB, QuestDB, TiDB, ClickHouse, Trino, Firebird, InfluxDB 3.
   - `nosql`: MongoDB, Redis, Valkey.
   - `heavy`: Elasticsearch, OpenSearch, Cassandra, Neo4j.
   - `all`: everything.
3. **Ports and credentials** are fixed and documented in `dev/README.md` (dev-only passwords, never reused elsewhere). `dev/engines.json` lists, per engine id: service name, port, image, readiness check, profile. Drivers' integration tests read it.
4. **Skip, don't fail**: integration tests skip an unreachable engine, except those named in `NINESQL_REQUIRE_ENGINES` (CI sets it per job, so nothing is skipped silently in CI).
5. **CI** runs a matrix by profile on GitHub Actions; the `heavy` profile runs nightly and on demand, and on PRs that touch its drivers.
6. **Cloud sandboxes**: when Docker is available, agents use the same compose file; when it is not, an agent works on unit tests and SQLite / DuckDB (no server needed), and the PR relies on CI for the server engines. `dev/README.md` documents both paths, and the task report says which one was used.

## Consequences
- The dev environment is the integration contract for drivers: a driver task is done when its engine's CI job is green.
- Image digests need regular bumps (Dependabot `docker` ecosystem on `dev/`).
- The heavy profile needs about 8 GB of RAM; it is never required for a PR that does not touch those drivers.

## Rejected alternatives
- **testcontainers in each test**: convenient, but starts containers per test process, slower in CI, and harder in sandboxes with restricted Docker.
- **Mocks only**: cannot catch catalog, type mapping or protocol edge cases, which are most of a database client's bugs.
