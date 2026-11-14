# CLAUDE.md

@CONTEXT.md
@AGENTS.md

## Claude Code specifics
- Project subagents: `.claude/agents/` (`ui-engineer`, `core-engineer`, `driver-engineer`, `interop-engineer`, `security-reviewer`, `docs-keeper`). Delegate according to the ROADMAP's "Owner" column.
- To parallelize: one worktree per task, one subagent per worktree. Never run two subagents on the same owner directory.
- Any task that touches `crates/ninesql-secrets/`, `crates/ninesql-ssh/`, `crates/ninesql-driver-api/`, `crates/ninesql-dbeaver/`, TLS settings or read-only enforcement ends with a `security-reviewer` review.
- If a request contradicts an invariant or an ADR, flag it and propose an ADR instead of implementing it.
- Cloud sessions (Claude Code on the web) have no display and no GPU: do backend, driver, interop and CI work there; UI work that needs to be seen runs locally. Details in [docs/09-agent-workflow.md](docs/09-agent-workflow.md).
