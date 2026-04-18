# Roadmap

A staged build. Each phase ships something usable; later phases extend capability without rework.

## Phase 0 — Grid and rigging

**Goal:** rig material, persist it as diffable YAML, serve it via CLI.

- Storage layout (`grid/`, `index/`, `cues/`).
- `flyloft init` and `flyloft status`.
- `flyloft rig` for files, directories, plain URLs.
- Chunking (semantic by default, fixed as fallback).
- Batten and line set YAML serialization.
- Git integration (commits on rig/strike/spike).
- `flyloft config`, `flyloft doctor`, `flyloft reindex`.

**Done when:** a user can `flyloft init`, rig a directory of markdown, commit the grid, and inspect a batten YAML by hand.

## Phase 1 — Hybrid retrieval

**Goal:** hybrid dense + sparse retrieval with a reranker, exposed via CLI and a first MCP tool.

- Embedding backend abstraction; default local model via candle/onnx.
- LanceDB dense index.
- Tantivy sparse index.
- Reranker abstraction; default local cross-encoder.
- Query pipeline (rewrite → hybrid → rerank → assemble).
- `flyloft fly` CLI command.
- `flyloft_fly` MCP tool.
- Cue sheet logging.

**Done when:** agents and humans can retrieve ranked battens against real queries with reasonable quality on a ThreeDoors-scale corpus.

## Phase 1.5 — Catalogs

**Goal:** extend retrieval beyond the local stacks to adapter-mediated external sources, with curation verbs working uniformly across held and cataloged battens.

- `Catalog` trait + `CatalogRegistry` in core.
- `BattenContent::Cataloged` variant flowing through persistence, retrieval, and rerank without special cases.
- First reference adapters: **GitHub** (issues, PRs, repo markdown), **generic REST/Confluence**, **peer Flyloft** (inter-flyloft loan).
- Federated query: merge stacks + `in_default_federation=true` catalogs before rerank.
- `flyloft_fetch` resolves cataloged battens transparently.
- Cache layer (`.flyloft/cache/`) with TTL, None, and Pinned policies.
- `flyloft catalog` CLI (`list`, `add`, `remove`, `health`).
- `flyloft adopt <batten-id>` to convert cataloged → held.
- `flyloft rig --catalog <id> --query "…"` to rig a catalog slice into the stacks permanently.

**Done when:** a flyperson can register a GitHub catalog and a peer Flyloft, run a single query that returns results from the stacks, GitHub, and the peer, and spike/strike/annotate results from any source identically.

## Phase 2 — Curation surface

**Goal:** the flyperson's full verb set.

- `flyloft spike`, `strike`, `dog`, `annotate`.
- Confidence state machine (bedrock/frontier/graveyard) enforced in retrieval.
- `flyloft contribute` MCP tool + review CLI.
- `flyloft dispute` MCP tool + resolution CLI.
- Agent-annotated vs flyperson-reviewed distinction.
- `flyloft cite` MCP tool and citation feedback logging.

**Done when:** a flyperson can run a meaningful grooming session with real data.

## Phase 3 — Grooming and telemetry

**Goal:** the flyperson's signal loop.

- `flyloft groom` interactive TUI.
- `flyloft weed` with stale, disused, superseded detection.
- `flyloft cue` subcommands (recent, gaps, coldspots, hotspots).
- Coverage metrics (what fraction of cues returned useful drops).
- Basic grooming dashboard via `flyloft serve --rest`.

**Done when:** running `flyloft groom` weekly produces measurable improvements in retrieval quality (visible in cue sheet coverage metrics).

## Phase 4 — Entity graph overlay

**Goal:** entity extraction and graph-aware retrieval.

- Entity extraction at rig time (pluggable; default local NER, optional LLM).
- Entity store and relationships.
- `flyloft_entities` MCP tool.
- Entity-based query expansion.
- `flyloft entity` CLI commands.

**Done when:** "tell me about Cilium" returns all Cilium-linked battens, not just lexical matches.

## Phase 5 — KOS promotion

**Goal:** the Flyloft → KOS promotion path.

- `[kos]` config block.
- `flyloft promote` command.
- Promotion writes KOS nodes, updates cross-references, records `promoted_to` in the batten.
- Optional KOS → Flyloft demotion hook.

**Done when:** a spiked batten can be promoted to a KOS bedrock node with full provenance and bidirectional references.

## Phase 6 — marvel integration

**Goal:** first-class provisioning under marvel-managed workloads.

- Auth delegation to marvel.
- Provisioning path (marvel-side automation — git repo, config, endpoint registration).
- REST surface for human operators hitting flyloft alongside their agent sessions.
- Team-scoped MCP endpoint registration so marvel-scheduled forestage sessions can reach flyloft.
- kos co-location and wiring via `flyloft.toml`.

**Done when:** a new marvel-managed team gets a working flyloft instance automatically on provisioning.

## Phase 7 — BYOA polish

**Goal:** make the standalone mode production-ready.

- Token system (`flyloft token issue/revoke/list`).
- Per-session MCP profile scoping.
- Docker image + systemd unit.
- Backup/restore tooling.
- Multi-grid composition (a single agent can query across multiple flylofts — e.g. personal + team).

**Done when:** a researcher can spin up a private Flyloft against their own Claude Code setup in under 10 minutes.

## Beyond

Deferred but on the horizon:

- **Real-time ingestion** from streaming sources (Slack, email, transcripts).
- **Multi-modal rigging** (images, PDFs, audio transcripts as first-class sources).
- **Collaborative grooming** (multiple flypersons working the same grid with conflict resolution).
- **Cross-flyloft federation** (searching across multiple team flylofts with trust boundaries).
- **Spec Cycle automation** (automated harvest→promote based on cue sheet hotspots).
