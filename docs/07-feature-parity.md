# Feature parity inventory

NineSQL's long-term goal is **parity with the features DBeaver users rely on**, on the 20 engines of [ADR-0003](adr/0003-supported-engines-v1.md). This file is the inventory: each feature has a stable id (`F-<area>-<nn>`) that ROADMAP tasks and PRs quote.

- **Edition**: where the feature lives in DBeaver. `CE` = Community Edition, `PRO` = commercial editions only. NoSQL engines are PRO-only in DBeaver, so their parity target is DBeaver PRO's behavior. The column reflects DBeaver's public documentation as of late 2026 and is re-checked in P0-E; a row marked `?` is unverified.
- **Phase**: the ROADMAP phase that delivers it. MVP = phases 0 to 6.
- **Status**: `planned` · `in progress` · `done` · `partial` (with a note) · `won't do` (with a reason). Updated by `docs-keeper` after each merge.

Parity means the user can do the same job, not that the UI is a copy: NineSQL may solve a need differently (e.g. a command palette instead of a menu path).

## Connections (`CON`)
| Id | Feature | Edition | Phase | Status |
|---|---|---|---|---|
| F-CON-01 | Connection manager: create, edit, duplicate, delete, test connection | CE | 1 | planned |
| F-CON-02 | Folders and drag-and-drop organization | CE | 1 | planned |
| F-CON-03 | Connection types (Dev / Test / Prod, custom) with color, auto-commit, confirmations, read-only | CE | 1 | planned |
| F-CON-04 | Saved passwords in a secure store; "ask every time" mode | CE | 1 | planned |
| F-CON-05 | SSH tunnel: password, private key, agent; jump hosts | CE | 1 | planned |
| F-CON-06 | TLS: modes, CA, client certificates | CE | 1 | planned |
| F-CON-07 | SOCKS / HTTP proxy | CE | 8 | planned |
| F-CON-08 | Driver / connection properties (engine-specific options) | CE | 1 | planned |
| F-CON-09 | Bootstrap queries / init SQL, default schema / catalog | CE | 3 | planned |
| F-CON-10 | Keep-alive, auto-reconnect, close idle connections | CE | 3 | planned |
| F-CON-11 | Read-only connection | CE | 1 | planned |
| F-CON-12 | Connect by URL / connection string | CE | 1 | planned |
| F-CON-13 | Multiple projects | CE | 10 | planned |
| F-CON-14 | Import connections from DBeaver (connections, folders, types, SSH, TLS, passwords, scripts) | — | 2 | planned |
| F-CON-15 | Export connections to DBeaver format (no passwords) | — | 5 | planned |
| F-CON-16 | Local engine detection (SQLite / DuckDB files, local sockets) | ? | 10 | planned |

## Navigator and metadata (`NAV`)
| Id | Feature | Edition | Phase | Status |
|---|---|---|---|---|
| F-NAV-01 | Object tree: catalogs, schemas, tables, views, materialized views, columns | CE | 4 | planned |
| F-NAV-02 | Indexes, constraints, foreign keys, triggers, sequences, routines, types, partitions | CE | 4 | planned |
| F-NAV-03 | Lazy, async loading with cancel; refresh per node | CE | 4 | planned |
| F-NAV-04 | Navigator filters (include / exclude patterns per connection) | CE | 4 | planned |
| F-NAV-05 | Quick search of objects (fuzzy) across a connection | CE | 4 | planned |
| F-NAV-06 | Object editor: properties, columns, keys, indexes, DDL tab | CE | 4 | planned |
| F-NAV-07 | Generate DDL / SQL for objects (CREATE, SELECT, INSERT templates) | CE | 4 | planned |
| F-NAV-08 | Create / alter / drop objects through forms, with a script preview before apply | CE | 10 | planned |
| F-NAV-09 | Object dependencies (uses / used by) | CE | 10 | planned |
| F-NAV-10 | Users, roles and grants browser / editor | CE | 10 | planned |
| F-NAV-11 | Full-text data search across tables | CE | 10 | planned |
| F-NAV-12 | Schema compare (simple structure diff) | CE / PRO | 10 | planned |

## SQL editor and execution (`SQL`)
| Id | Feature | Edition | Phase | Status |
|---|---|---|---|---|
| F-SQL-01 | Syntax highlighting per dialect (tree-sitter) | CE | 3 | planned |
| F-SQL-02 | Autocompletion: keywords, objects, columns, aliases, joins on foreign keys | CE | 3 | planned |
| F-SQL-03 | Execute statement under cursor / selection / script; statement splitting per dialect | CE | 3 | planned |
| F-SQL-04 | Multiple result tabs; results in separate tabs or panels | CE | 3 | planned |
| F-SQL-05 | Cancel running query (server-side where supported) | CE | 3 | planned |
| F-SQL-06 | Query parameters and variables (`:name`, `?`, `${var}`) with a prompt | CE | 3 | planned |
| F-SQL-07 | Transaction modes: auto-commit, manual, smart commit; commit / rollback; pending transactions view | CE | 3 | planned |
| F-SQL-08 | Execution log and query history (searchable, per connection) | CE | 3 | planned |
| F-SQL-09 | SQL formatter (configurable) | CE | 5 | planned |
| F-SQL-10 | SQL templates / snippets | CE | 5 | planned |
| F-SQL-11 | Script files: save, open, folders, linked to a connection | CE | 3 | planned |
| F-SQL-12 | Error position highlighting from the server's error | CE | 3 | planned |
| F-SQL-13 | Explain plan: text and visual tree, with costs | CE | 10 | planned |
| F-SQL-14 | Outline of the script, go to definition of objects | CE | 10 | planned |
| F-SQL-15 | Find / replace, multi-cursor, comment toggle, code folding | CE | 3 | planned |
| F-SQL-16 | Execute script with "continue on error" / "stop on error" and a summary | CE | 5 | planned |
| F-SQL-17 | Visual query builder | PRO? | 11 | planned |
| F-SQL-18 | Destructive statement confirmation on production connections | CE (partial) | 3 | planned |

## Data viewer and editor (`DATA`)
| Id | Feature | Edition | Phase | Status |
|---|---|---|---|---|
| F-DATA-01 | Virtualized grid with streaming fetch, fetch next page / fetch all | CE | 3 | planned |
| F-DATA-02 | Record (form) view and text view | CE | 5 | planned |
| F-DATA-03 | Sort and filter: client-side and server-side (`WHERE` / `ORDER BY` rewriting), filter history | CE | 5 | planned |
| F-DATA-04 | In-cell editing, insert / duplicate / delete rows, save with generated DML preview | CE | 5 | planned |
| F-DATA-05 | Value viewers: JSON (tree, folding), XML, text, binary (hex), image | CE | 5 | planned |
| F-DATA-06 | Advanced copy: CSV, TSV, Markdown, SQL `INSERT`, JSON | CE | 5 | planned |
| F-DATA-07 | Navigate foreign keys (open referenced / referencing rows) | CE | 5 | planned |
| F-DATA-08 | Foreign key value lookup / dictionary editing | CE | 10 | planned |
| F-DATA-09 | Aggregate panel (count, sum, avg, min, max of the selection) | CE | 5 | planned |
| F-DATA-10 | Column statistics / metadata panel | CE | 5 | planned |
| F-DATA-11 | Grouping, pinning and hiding columns; color by value rules | CE | 10 | planned |
| F-DATA-12 | Spatial / GIS viewer (map for geometry columns) | CE | 10 | planned |
| F-DATA-13 | Auto-refresh of results | CE | 10 | planned |
| F-DATA-14 | Compare two result sets / data compare | PRO | 11 | planned |

## Data transfer (`XFER`)
| Id | Feature | Edition | Phase | Status |
|---|---|---|---|---|
| F-XFER-01 | Export results / tables to CSV, TSV, JSON, SQL `INSERT`, Markdown, HTML (streamed) | CE | 5 | planned |
| F-XFER-02 | Export to XLSX, XML, Parquet | CE | 10 | planned |
| F-XFER-03 | Import CSV into a table with column mapping and preview | CE | 5 | planned |
| F-XFER-04 | Table-to-table transfer across connections (data migration) with type mapping | CE | 10 | planned |
| F-XFER-05 | Saved tasks (re-runnable exports / transfers) | CE | 10 | planned |
| F-XFER-06 | Task scheduling | PRO | 11 | planned |
| F-XFER-07 | Mock data generation | PRO | 11 | planned |

## Database tools (`TOOL`)
| Id | Feature | Edition | Phase | Status |
|---|---|---|---|---|
| F-TOOL-01 | Backup / restore through native tools (`pg_dump` / `pg_restore`, `mysqldump` / `mariadb-dump`…) when installed | CE | 10 | planned |
| F-TOOL-02 | Session manager: list sessions, cancel / kill | CE | 10 | planned |
| F-TOOL-03 | Lock manager | CE | 10 | planned |
| F-TOOL-04 | Maintenance: vacuum / analyze / optimize / reindex | CE | 10 | planned |
| F-TOOL-05 | Server dashboards (connections, throughput, locks) | CE | 10 | planned |
| F-TOOL-06 | Execute SQL scripts from files against several connections | CE | 10 | planned |

## ER diagrams (`ERD`)
| Id | Feature | Edition | Phase | Status |
|---|---|---|---|---|
| F-ERD-01 | Auto-generated diagram for a schema / selected tables | CE | 10 | planned |
| F-ERD-02 | Custom diagrams: add tables, notes, layout, save in the project | CE | 10 | planned |
| F-ERD-03 | Export diagram as PNG / SVG | CE | 10 | planned |
| F-ERD-04 | Edit schema from the diagram | PRO? | 11 | planned |

## NoSQL engines (`NOSQL`)
| Id | Feature | Edition | Phase | Status |
|---|---|---|---|---|
| F-NOSQL-01 | MongoDB: databases / collections / indexes tree, document grid and JSON view, find / aggregate editor, edit documents | PRO | 8 | planned |
| F-NOSQL-02 | Redis / Valkey: key browser by pattern (SCAN), typed value editors (string, hash, list, set, zset, stream), TTL, command console | PRO | 8 | planned |
| F-NOSQL-03 | Elasticsearch / OpenSearch: indices tree, SQL endpoint and query DSL editor, document view | PRO | 9 | planned |
| F-NOSQL-04 | Cassandra: keyspaces / tables tree, CQL editor, paging | PRO | 9 | planned |
| F-NOSQL-05 | Neo4j: labels / relationship types, Cypher editor, table and graph result views | PRO | 9 | planned |

## Workspace and UX (`WS`)
| Id | Feature | Edition | Phase | Status |
|---|---|---|---|---|
| F-WS-01 | Tabs, split panes, dockable panels, multiple windows | CE | 1 | planned |
| F-WS-02 | Command palette and configurable key bindings | — | 1 | planned |
| F-WS-03 | Light / dark themes, font settings, HiDPI and fractional scaling on Wayland | CE | 1 | planned |
| F-WS-04 | Preferences UI (global and per connection) | CE | 3 | planned |
| F-WS-05 | Bookmarks on objects and scripts | CE | 10 | planned |
| F-WS-06 | Session restore (open tabs and scripts after restart) | CE | 3 | planned |
| F-WS-07 | Git-friendly projects (scripts and connections without secrets) | CE | 10 | planned |
| F-WS-08 | AI assistant (opt-in, user-provided endpoint, never sends data without confirmation) | CE | 11 | planned |
| F-WS-09 | Localization (English first, then community translations) | CE | 11 | planned |

## Deliberately different from DBeaver
- **Passwords** are never stored with a shared obfuscation key (ADR-0005).
- **Read-only** is enforced engine-side (ADR-0007).
- **No plugin marketplace** in v1: extensions may come later through a new ADR.
