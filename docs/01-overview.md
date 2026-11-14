# Overview

## Problem
DBeaver is the most used universal database client. Its breadth comes from the JVM, Eclipse RCP and JDBC, and so do its costs on Linux: high memory use, slow cold start, inconsistent HiDPI and Wayland behavior, and a UI that struggles with large result sets. Its Community Edition also stores saved passwords with a shared, public key.

## Goal
NineSQL is a native database client for Debian and Ubuntu that:
1. Covers the features DBeaver users rely on ([feature parity inventory](07-feature-parity.md)) on the 20 most used open-source engines ([ADR-0003](adr/0003-supported-engines-v1.md)).
2. Feels instant: native Rust, GPU rendering with GPUI, streamed results in bounded memory.
3. Is secure by default: keyring-backed secrets, verified TLS and SSH host keys, engine-side read-only, no telemetry ([docs/05-security.md](05-security.md)).
4. Lets DBeaver users switch in one click ([docs/06-dbeaver-interop.md](06-dbeaver-interop.md)).

## Users
- **Developers** who query local and staging databases all day and want speed and good SQL editing.
- **DBAs / SREs** who reach production through SSH bastions and need guard rails that actually hold.
- **Data analysts** who read large results, filter and export them.
- **Teams migrating from DBeaver**, who need their connections and habits to carry over.

## Non-goals (v1)
- Windows support (macOS builds exist for contributors, not as a release target).
- Proprietary and cloud-only engines (ADR-0003).
- A plugin marketplace.
- Web or server mode: NineSQL is a desktop application.
- Being a DBeaver UI clone: same jobs, NineSQL's own interaction design.

## Principles
- **Measure, don't promise**: performance claims appear in the docs only with a method, a machine and a commit.
- **Honest capabilities**: if an engine cannot do something, the UI says so (I8).
- **Safe defaults, explicit downgrades**: every weaker setting is visible.
