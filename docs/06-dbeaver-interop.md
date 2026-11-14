# DBeaver interoperability

Decision record: [ADR-0006](adr/0006-dbeaver-interoperability.md). Owner: `interop-engineer` (`crates/ninesql-dbeaver/`).

> **Verification status.** The format notes below come from DBeaver Community's public source and documentation. Each item marked *(to verify)* must be confirmed against real fixtures (P0-E) before the importer relies on it. Do not "fix" the importer to match this page: fix this page to match the fixtures.

## Where DBeaver keeps its data on Linux
| Install | Default workspace |
|---|---|
| `.deb` / tarball | `~/.local/share/DBeaverData/workspace6/` |
| Flatpak (`io.dbeaver.DBeaverCommunity`) | `~/.var/app/io.dbeaver.DBeaverCommunity/data/DBeaverData/workspace6/` *(to verify)* |
| Snap (`dbeaver-ce`) | `~/snap/dbeaver-ce/current/.local/share/DBeaverData/workspace6/` *(to verify)* |

Inside a workspace, each project is a folder (`General/` by default) with:
| File | Content |
|---|---|
| `.dbeaver/data-sources.json` (and `data-sources-*.json`) | Connections, folders, connection types, network handlers (SSH, proxy, TLS) |
| `.dbeaver/credentials-config.json` | Saved user names and passwords, encrypted |
| `.dbeaver/project-settings.json` | Project-level settings *(to verify)* |
| `Scripts/` | SQL scripts (`.sql`), with their connection association in project metadata *(to verify)* |

## `data-sources.json` (main keys)
- `folders`: map of folder name → `{ parent?, description? }`.
- `connection-types`: map of type id → `{ name, color, description, auto-commit, confirm-execute, confirm-data-change, … }` *(exact keys to verify)*.
- `connections`: map of connection id → object with:
  - `provider` (e.g. `postgresql`, `mysql`, `sqlite`, `generic`) and `driver` (e.g. `postgres-jdbc`, `mysql8`, `mariaDB`) → NineSQL engine id (table below);
  - `name`, `folder`, `save-password`, `read-only`;
  - `configuration`: `host`, `port`, `database`, `server`, `url`, `type` (connection type id), `auth-model`, `user` (when not in credentials), `bootstrap` (default catalog / schema, init queries, auto-commit), `provider-properties`, driver `properties`, `handlers` (network handlers: `ssh_tunnel`, proxy, `postgre_ssl` / `mysql_ssl`…, each with `enabled`, `save-password`, `properties`).
- Unknown keys are preserved in the parsed model's `extra` map and listed in the import report.

## `credentials-config.json`
- Encrypted with AES in CBC mode using a **fixed key present in DBeaver's source code**; the first 16 bytes of the file are the IV *(to verify per version)*. Anyone who can read the file can decrypt it: this is why NineSQL moves the secrets to the keyring ([ADR-0005](adr/0005-secrets-storage.md)).
- Decrypted content: JSON map of connection id → `{ "#connection": { user, password }, "network/<handler-id>": { user, password } }` *(to verify)*.
- DBeaver PRO can protect credentials with a master password / secure storage; NineSQL detects that the file is not in the Community format and imports connections without passwords.
- Decrypted values live only in `secrecy` / `zeroize` buffers and go straight to `ninesql-secrets` (I1).

## Engine mapping
| DBeaver `provider` / `driver` (examples) | NineSQL engine id | Phase |
|---|---|---|
| `postgresql` / `postgres-jdbc` | `postgresql` | MVP |
| `postgresql` with Timescale / Yugabyte drivers *(to verify ids)* | `timescaledb`, `yugabytedb` | MVP |
| `mysql` / `mysql8`, `mysql5` | `mysql` | MVP |
| `mysql` / `mariaDB` | `mariadb` | MVP |
| `generic` / `tidb` *(to verify)* | `tidb` | MVP |
| `generic` / `questdb` *(to verify)* | `questdb` | MVP |
| `sqlite` / `sqlite_jdbc` | `sqlite` | MVP |
| `duckdb` *(to verify)* | `duckdb` | v0.2 |
| `clickhouse` / `com_clickhouse` *(to verify)* | `clickhouse` | v0.2 |
| `generic` / `trino_jdbc` *(to verify)* | `trino` | v0.2 |
| `jaybird` / `firebird` *(to verify)* | `firebird` | v0.2 |
| MongoDB, Redis, Cassandra, Elasticsearch, Neo4j (PRO providers) | matching ids | v0.3 / v0.4 |
| Oracle, SQL Server, DB2, Snowflake, … | — (imported disabled, ADR-0003) | after v1 |

Unknown provider / driver pairs are imported disabled with their raw values kept, never dropped.

## Field mapping rules
- **JDBC URL**: when `url` is set and host / port are empty, parse the JDBC URL per engine (`jdbc:postgresql://host:port/db?params`, `jdbc:mysql://…`, `jdbc:sqlite:/path`). Parameters with a NineSQL equivalent (`sslmode`, `connectTimeout`, `currentSchema`…) are mapped; the others are kept in properties and reported.
- **Connection types**: DBeaver's `dev`, `test`, `prod` map to NineSQL's Development, Test, Production; custom types are created with their color and flags.
- **SSH**: host, port, user, auth type (password / public key / agent), key path, saved passphrase, jump hosts if present. The host key is **not** trusted automatically: the first connection asks (ADR-0011).
- **TLS**: mode, CA / client certificate paths. Connections that do not verify certificates are flagged in the report.
- **Scripts**: optional copy into the NineSQL project, keeping the folder structure and the connection association when it can be resolved.

## Import report
For each connection: status (imported / imported disabled / failed), fields mapped, DBeaver keys not supported, secrets moved (count only, never values), warnings (TLS not verified, unknown driver, SSH host key to confirm). Saved in the project as `imports/dbeaver-<timestamp>.json` and shown after the import.

## Export to DBeaver (P5-D)
Writes a **new** `data-sources.json` chosen by the user, for selected connections, without credentials and without `credentials-config.json`. NineSQL-only settings are omitted. The output is tested by importing it back with NineSQL's importer and, manually, into DBeaver.
