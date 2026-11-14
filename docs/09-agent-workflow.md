# Working with AI coding agents

NineSQL is built largely by AI coding agents (Claude Code subagents of [`.claude/agents/`](../.claude/agents/), and others) driven by the maintainer. This page explains how work is split, and what can run where.

## Roles
| Subagent | Owns | Typical tasks |
|---|---|---|
| `ui-engineer` | `ninesql-ui`, `ninesql-grid`, `ninesql-editor` | GPUI spike, shell, dialogs, editor, grid, navigator |
| `core-engineer` | `ninesql-core`, `ninesql-storage`, `ninesql-transfer`, `ninesql-secrets`, `ninesql-ssh`, `packaging/` | Runtime bridge, storage, secrets, SSH, export, `.deb` |
| `driver-engineer` | `ninesql-driver-api`, `crates/drivers/`, `dev/` | Driver API, drivers, dialects, dev containers |
| `interop-engineer` | `ninesql-dbeaver` | DBeaver import / export |
| `security-reviewer` | read-only | Reviews of sensitive changes and of each phase |
| `docs-keeper` | `docs/`, root `*.md`, `.github/` | ROADMAP, parity inventory, capability matrix, ADRs |

Rules: one task = one branch = one worktree; two agents never work on the same owner directory at the same time; the driver API is the only synchronization point ([AGENTS.md](../AGENTS.md)).

## Where each kind of task can run
Cloud agent sandboxes (Claude Code on the web and similar) are Linux containers **without a display or GPU**. Whether they can run Docker and reach package registries depends on how the environment is configured. Local runs (the maintainer's machine) have a display.

| Task | Cloud sandbox | Local |
|---|---|---|
| Driver API, core, storage, secrets logic, DBeaver parser, transfer | ✅ build and unit tests | ✅ |
| Driver integration tests against server engines | ✅ if Docker is available in the environment, otherwise CI only | ✅ |
| SQLite / DuckDB integration tests | ✅ (no server needed) | ✅ |
| Secret Service backend tests | ◐ needs a D-Bus session and a keyring daemon in the sandbox; CI runs them | ✅ on a Linux desktop |
| GPUI build | ◐ possible with the system packages of [docs/10](10-supported-platforms.md); the first build is long | ✅ |
| GPUI view-model tests (`TestAppContext`) | ✅ | ✅ |
| Seeing the UI, screenshots, performance measurements | ❌ | ✅ (Linux for release measurements; macOS is fine for UI iteration) |
| `.deb` build and install tests | ✅ through CI | ✅ |

### Practical rules
- Phases 0-B, 0-C, 0-D, 2 (parser), the drivers of phases 4, 7, 8, 9 and most of 5-B / 5-C fit cloud sessions well.
- Phases 0-A, 1-A, 1-C, 3-A, 3-B, 3-E and every UI review need at least one local run. A cloud agent can still write the code and tests, then hand off with "needs local visual check" in the task report.
- The task report states which test path was used (Docker in sandbox, CI only, local).
- Agents never need real credentials: everything runs against `dev/` containers with dev-only passwords and synthetic DBeaver fixtures.

## Typical flow for a task
1. The maintainer picks a ROADMAP task and starts a session (local or cloud) with the owner subagent.
2. The agent sets the task to `[~]`, creates `feat/<id>-<slug>` from `dev` in its own worktree, and works within its owner directories.
3. It runs fmt, clippy, tests, `cargo deny`, and the link check; it writes the task report in the PR description.
4. Sensitive areas get a `security-reviewer` pass before the PR is opened.
5. The maintainer reviews and merges; `docs-keeper` updates ROADMAP, CONTEXT, parity inventory and capability matrix.
