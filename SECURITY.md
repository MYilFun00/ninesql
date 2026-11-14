# Security policy

## Supported versions
| Version | Supported |
|---------|-----------|
| No release yet | — |
| Unreleased commits of `main` / `dev` | No |

From v0.1.0, the latest `0.x` minor version receives security fixes as patch releases.

## Reporting a vulnerability
**Do not open a public issue, discussion or pull request.** Report privately through GitHub Security Advisories: in this repository, **Security** tab → **Report a vulnerability**.

Please include: the affected version or commit, the component (UI, a driver, secrets, SSH / TLS, DBeaver import, packaging), reproduction steps and the estimated impact. Never send real credentials or real DBeaver workspaces: reproduce with the `dev/` containers and synthetic files.

## Commitment
- Acknowledgment within 72 h
- Initial assessment within 7 days
- Fix and advisory coordinated with the reporter, published together with the patch release

## Scope
In scope: the `ninesql` binary and its crates, the stored workspace files, the encrypted vault, the SSH / TLS behavior, the DBeaver importer / exporter, the `.deb`, the APT repository metadata and the Flatpak manifest.

Out of scope: vulnerabilities of the database engines themselves, the `dev/` test environment (dev-only passwords by design), and setups that turn off a documented safeguard (TLS `require` / `disable`, accepted unknown SSH host keys, read-only off on a production connection).

## Release signing
Release assets are signed from the tag-push workflow (cosign keyless). The APT repository signing key fingerprint will be published here with v0.1.0.
