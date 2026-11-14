# Tech stack

Starting choices. Driver client crates are confirmed (or replaced) by their driver task and recorded in [docs/08-engine-capabilities.md](08-engine-capabilities.md). Every crate must pass the license policy of [ADR-0002](adr/0002-licensing-and-dependency-policy.md).

| Area | Choice | Notes |
|---|---|---|
| Language | Rust stable, edition 2024 | `rust-toolchain.toml` |
| UI | `gpui` (git revision of `zed-industries/zed`), `gpui-component` (matching revision) | ADR-0001; never Zed's GPL crates |
| Syntax | `tree-sitter` + SQL grammars per dialect | Completion from our own metadata cache |
| Async runtime | `tokio` (multi-thread, bounded workers) for driver and network I/O | ADR-0004 |
| Results | `arrow-array`, `arrow-schema` (not the full `arrow` crate) | Tabular batches; documents as an ordered value tree |
| TLS | `rustls`, `rustls-platform-verifier` / `rustls-native-certs` | No OpenSSL (ADR-0011) |
| SSH | `russh`, `russh-keys` | Jump hosts, agent, known_hosts |
| Secrets | `keyring` (Secret Service, pure-Rust D-Bus backend), `argon2`, `chacha20poly1305`, `secrecy`, `zeroize` | ADR-0005 |
| Storage | `serde`, `serde_json`, `toml`, `directories` (XDG), `rusqlite` for history | ADR-0008 |
| HTTP (for HTTP-based engines) | `reqwest` with `rustls-tls`, no default features | ClickHouse, Trino, InfluxDB, Elasticsearch / OpenSearch |
| Export formats | `csv`, `serde_json`, `parquet` (phase 10), `rust_xlsxwriter` (phase 10) | Streamed |
| Errors / logs | `thiserror`, `tracing`, `tracing-subscriber`, `tracing-appender` | No values in logs |
| Testing | `proptest`, `insta` (snapshots without secrets), `cargo-fuzz` for parsers | |
| Supply chain | `cargo-deny` (licenses, bans, sources, advisories) | Required CI check |
| Packaging | `cargo-deb`, Flatpak, cosign, CycloneDX SBOM | ADR-0010 |

## Driver clients (initial)
| Family | Crate |
|---|---|
| pg-wire | `tokio-postgres`, `tokio-postgres-rustls` |
| mysql-wire | `mysql_async` (rustls) |
| SQLite | `rusqlite` (bundled) |
| DuckDB | `duckdb` (bundled) |
| ClickHouse | `clickhouse` or HTTP via `reqwest` |
| Trino | HTTP via `reqwest` (client protocol) |
| Firebird | `rsfbclient` (pure-Rust feature) |
| InfluxDB 3 | HTTP via `reqwest` |
| MongoDB | `mongodb` |
| Redis / Valkey | `redis` |
| Elasticsearch / OpenSearch | HTTP via `reqwest` |
| Cassandra | `scylla` |
| Neo4j | `neo4rs` |
