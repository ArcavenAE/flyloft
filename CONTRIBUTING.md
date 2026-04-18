# Contributing to flyloft

flyloft is in active development and accepts contributions. This file
describes how.

## Quick start

1. Fork the repo or clone it directly if you have write access.
2. Install the toolchain: `just setup` (installs clippy, nightly
   rustfmt, cargo-watch, cargo-deny).
3. Install git hooks: `just install-hooks`.
4. Run the full CI locally: `just ci`.
5. Make your change on a branch off `main`.
6. Open a pull request.

## Development workflow

- **Build:** `just build`
- **Test:** `just test`
- **Lint:** `just lint` (formats check + clippy + cargo-deny)
- **Format:** `just fmt` (nightly rustfmt) and `just fmt-toml` (taplo)
- **CI mirror:** `just ci`

All pull requests must pass CI: rustfmt, clippy with `-D warnings`,
cargo test, and cargo-deny (advisories, licenses, bans).

## Code style

See [`.claude/rules/rust.md`](.claude/rules/rust.md) for the Rust
conventions this project follows:

- No `unsafe` code (`#![forbid(unsafe_code)]`)
- No `unwrap()` outside tests
- Newtypes for IDs, thiserror for errors, `#[non_exhaustive]` on
  enums that will grow
- Workspace-level dependency declarations
- Table-driven tests, `tokio::test` for async

## Commit messages

See [`.claude/rules/git-commits.md`](.claude/rules/git-commits.md).

Conventional commits with kos actions mixed in:

```
feat(cli): add flyloft spike subcommand
fix(core): correct batten_id validation on empty input
probe(flyloft): hybrid retrieval — measuring rerank latency
finding(flyloft): reranker-latency — acceptable at k=20
```

All commits must be SSH-signed. No AI attribution in commit messages.

## The kos process

flyloft follows the kos process for design work:

1. Read [`charter.md`](charter.md) to orient
2. Capture pre-hypothesis ideas in `_kos/ideas/` (markdown)
3. Promote crystallized ideas to frontier question nodes in
   `_kos/nodes/frontier/`
4. Write a probe brief in `_kos/probes/`
5. Do the probe work (code, measurement, research)
6. Write a finding in `_kos/findings/`
7. Harvest — move nodes between confidence tiers, update the charter

This lives in each repo. Cross-repo questions belong in the orchestrator
(`aae-orc/_kos/`).

## Reporting bugs

File an issue with:
- A minimal reproduction
- Your platform (OS, Rust version)
- Expected vs actual behavior
- Log output (with `RUST_LOG=debug` if relevant)

Security issues: see [`SECURITY.md`](SECURITY.md) — private disclosure only.

## Code of conduct

Contributors must follow the [Code of Conduct](CODE_OF_CONDUCT.md).
