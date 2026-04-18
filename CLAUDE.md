# flyloft

Curated, graph-aware retrieval substrate for human-AI teams. A RAG system
with a reranker, a graph overlay, and a curation surface — and a role for
the "flyperson" who tends the collection the rest of the team draws from.

Part of the Arcaven theatrical stack. Flyloft is the *casebook* (raw,
discoverable corpus); KOS is the *promptbook* (distilled, authoritative
spec graph). Material flows Flyloft → KOS via the `promote` verb.

## Build / Run / Test

Requires: Rust 1.85+ (Edition 2024), `just`.

```sh
just build          # cargo build --workspace
just test           # cargo test --workspace
just run -- <args>  # flyloft CLI
just mcp            # flyloft MCP server
just lint           # fmt + clippy + deny (pre-commit mirror)
just ci             # full CI mirror
just fmt            # cargo +nightly fmt --all
```

## Architecture

```
crates/
  flyloft-core/       Library: types, traits, storage, retrieval pipeline
    src/
      lib.rs          Public API re-exports
      batten.rs       Batten + BattenContent (held | cataloged)
      line_set.rs     LineSet (versioned group of battens from one source)
      cue.rs          Cue (retrieval event, logged to the cue sheet)
      grid.rs         Grid layout + git-backed storage
      catalog.rs      Catalog trait + registry (external source adapter)
      provenance.rs   Provenance + Confidence tiers
  flyloft-cli/        Binary: the flyperson's CLI (rig, spike, strike, groom, …)
  flyloft-mcp/        Binary: MCP server (flyloft_fly, flyloft_contribute, …)

docs/                 ARCHITECTURE, CONCEPTS, CATALOGS, MCP, CLI, INTEGRATION, ROADMAP
examples/grid/        Example line sets, battens, cues (for tests and demos)
flyloft.example.toml  Example configuration
```

### Data model (see docs/ARCHITECTURE.md for detail)

- **Batten** — atomic unit of indexed material. Held (text in grid) or
  cataloged (pointer to external source). Same curation surface for both.
- **Line set** — versioned group of battens from one source.
- **Cue** — a single retrieval event, logged to the cue sheet.
- **Grid** — the overall corpus. Git-backed, YAML-serialized, diffable.
- **Catalog** — an external source exposed through an adapter (GitHub,
  Confluence, peer flyloft, REST, KOS-as-catalog).

### Confidence tiers (mirror KOS)

- **bedrock** — spiked, corroborated, stable
- **frontier** — default state for newly rigged material, provisional
- **graveyard** — struck but preserved, not returned in retrieval

Aligning with KOS's tiers makes `promote` a lossless transfer.

@.claude/rules/_index.md

## Conventions

- **Language:** Rust, edition 2024, MSRV 1.85.
- **No unsafe:** `#![forbid(unsafe_code)]` in every crate.
- **Config format:** TOML (`flyloft.toml`).
- **Storage:** git repo (grid content) + derived indices (LanceDB, Tantivy,
  SQLite) that rebuild from the grid. Derived artifacts are gitignored.
- **No file deletion:** Never delete user files. Overwrite only with
  explicit intent. Strike preserves material in the graveyard tier; it
  does not delete.
- **Parallel-safe:** Each session gets a UUID. No shared mutable state
  beyond the storage layer.
- **Git workflow:** trunk-based on `main` until distribution channel
  exists. Flip to gitflow (`develop` default) when alpha/stable split
  is needed for releases.

## Values

- **Portability:** Single static binary per role (CLI, MCP server).
  No runtime dependencies beyond the grid.
- **Composability:** CLI, MCP, REST are separate surfaces over the same
  core. All optional.
- **User sovereignty:** All config is local. No phone-home. Telemetry
  (cue sheet) is local-first, flyperson-owned.
- **Curator authority:** Agents propose (contribute, annotate, dispute);
  the flyperson commits. Privileged verbs (strike, spike, dog, promote)
  are flyperson-only.
- **Provenance is non-negotiable:** Every batten traces to source,
  ingestion time, contributor, and confidence signal.

## How to Work Here (kos Process)

### Re-introduction
Read charter.md before any substantive work. It contains:
- Current bedrock (what's committed)
- Current frontier (what's under exploration)
- Current graveyard (what's been ruled out)

### Session Protocol
1. Read charter.md (orient)
2. Identify the highest-value open question — or capture new ideas in _kos/ideas/
3. Write an Exploration Brief in _kos/probes/
4. Do the probe work
5. Write a finding in _kos/findings/
6. Harvest: update affected nodes, move files if confidence changed
7. Update charter.md if bedrock changed

Cross-repo questions belong in the orchestrator's _kos/, not here.

### Ideas (pre-hypothesis brainstorming)
Ideas live in _kos/ideas/ as markdown files. Generative, possibly
contradictory, no commitment. When an idea crystallizes, extract into a
frontier question + brief.

### Node Files
Nodes live in _kos/nodes/[confidence]/[id].yaml
Schema follows kos schema v0.3.
One node per file. Filename = node id.

### Confidence Changes
Moving a file between confidence directories IS the promotion.
Always accompany with a commit message explaining the evidence.

### Harvest Verification
Before starting the next cycle, verify:
- [ ] Finding written and committed
- [ ] Charter updated if bedrock changed
- [ ] Frontier questions updated (closed, opened, or revised)
- [ ] Exploration briefs marked complete or carried forward
