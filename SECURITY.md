# Security Policy

## Supported Versions

flyloft is in active development. Only the `main` branch and the most
recent tagged release (when releases exist) are supported.

## Reporting a Vulnerability

Please report security vulnerabilities privately via GitHub's security
advisory mechanism:

https://github.com/ArcavenAE/flyloft/security/advisories/new

Do not file public issues for security concerns.

## Scope

In scope:
- The flyloft binaries (`flyloft`, `flyloft-mcp`)
- The flyloft-core library
- Configuration parsing
- Catalog adapter code shipped in this repository
- Authentication and authorization paths

Out of scope:
- Third-party catalog targets (GitHub, Confluence, etc.) — report
  directly to those vendors
- External embedding and reranking backends
- End-user misconfiguration (e.g., committing secrets to `flyloft.toml`;
  the docs direct secrets to `flyloft.local.toml` which is gitignored)

## Response Expectations

flyloft is volunteer-maintained. We aim to acknowledge reports within 7
days and provide a remediation timeline within 30 days. Critical
vulnerabilities (unauthenticated remote compromise) will be prioritized.
