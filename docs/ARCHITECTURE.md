# Architecture

## Design goals

1. **Git-backed, diffable, reviewable.** Source of truth is a git repository so curation operations (rig, strike, spike) are PR-able and auditable. Aligns with KOS's storage model.
2. **Agent-first interface.** MCP is the primary exposure; CLI is the flyperson's tool; REST is a fallback for integrations that need it.
3. **Hybrid retrieval.** Dense embeddings for semantic recall, sparse (BM25) for rare-token precision, reranker on top of the union. Dense alone misses proper nouns; sparse alone misses paraphrase.
4. **Provenance is non-negotiable.** Every batten traces to its source, ingestion time, contributor, and confidence signal. No orphaned chunks.
5. **Small-to-big retrieval.** Retrieve small (the matching chunk) but expose the path to big (the parent section, the full line set) on demand.
6. **Graph overlay, not graph replacement.** Entity extraction links battens to a lightweight graph; the graph does not replace vector retrieval, it augments it.
7. **KOS-aware from day one.** Flyloft knows about KOS and can both ingest from and promote to it.

## Layered architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Exposure                                                    │
│   MCP server  ·  CLI  ·  REST (optional)                     │
├─────────────────────────────────────────────────────────────┤
│  Query pipeline                                              │
│   rewrite → federated search → rerank → context assembly     │
├─────────────────────────────────────────────────────────────┤
│  Curation                                                    │
│   rig · strike · spike · dog · annotate · dispute · promote  │
├─────────────────────────────────────────────────────────────┤
│  Sources                                                     │
│   Stacks (local grid)  ·  Catalogs (external, adapter-wrapped)│
├─────────────────────────────────────────────────────────────┤
│  Index                                                       │
│   dense (vector)  ·  sparse (BM25)  ·  entity graph          │
├─────────────────────────────────────────────────────────────┤
│  Storage                                                     │
│   git repo (content + metadata)  ·  sqlite/lancedb (index)   │
│   .flyloft/cache (catalog responses)                         │
└─────────────────────────────────────────────────────────────┘
```

## Data model

### Batten

The atomic unit of indexed material. Held or cataloged; curation verbs treat both uniformly.

```rust
pub struct Batten {
    pub id: BattenId,               // content-addressable; blake3-derived
    pub line_set: LineSetId,        // the source this belongs to
    pub content: BattenContent,     // held text OR catalog pointer
    pub parent: Option<BattenId>,   // for small-to-big traversal
    pub position: Position,         // location within the line set
    pub provenance: Provenance,
    pub confidence: Confidence,     // bedrock | frontier | graveyard
    pub spiked: bool,                // flyperson-marked authoritative
    pub dogged: bool,                // frozen against modification
    pub annotations: Vec<Annotation>,
    pub entities: Vec<EntityRef>,    // graph overlay links
}

pub enum BattenContent {
    Held { text: String },
    Cataloged(CatalogPointer),       // catalog_id + external_id + snippet + cache policy
}
```

### Line set

A versioned group of battens from a single source.

```rust
pub struct LineSet {
    pub id: LineSetId,
    pub source: Source,              // file, url, api, transcript, etc.
    pub rigged_at: DateTime<Utc>,
    pub rigged_by: Contributor,
    pub struck: Option<DateTime<Utc>>,
    pub battens: Vec<BattenId>,
    pub parent_hierarchy: Option<Tree<BattenId>>,  // for structured sources
}
```

### Cue

A single retrieval event, logged to the cue sheet.

```rust
pub struct Cue {
    pub id: CueId,
    pub query: String,
    pub rewritten: Option<String>,
    pub requester: Requester,        // agent id or human
    pub flown: Vec<(BattenId, f32)>, // retrieved battens with scores
    pub cited: Vec<BattenId>,        // which battens actually got used (feedback loop)
    pub at: DateTime<Utc>,
}
```

## Storage layout

```
<flyloft-root>/
├── flyloft.toml              # configuration
├── grid/                     # the corpus
│   ├── <line-set-id>/
│   │   ├── manifest.yaml     # line set metadata
│   │   ├── source.<ext>      # original source, preserved
│   │   └── battens/
│   │       └── <batten-id>.yaml
├── index/                    # search indices (gitignored; rebuildable)
│   ├── dense.lancedb
│   ├── sparse.sqlite
│   └── entities.sqlite
├── cues/                     # retrieval telemetry
│   └── <yyyy-mm>.jsonl
└── .flyloft/                 # local state (gitignored)
    └── cache/
```

Battens as YAML files are human-readable, diffable, and PR-reviewable. The index is a derived artifact and rebuilds from the grid.

## Retrieval pipeline

1. **Query rewrite.** The raw query (from agent or human) is optionally rewritten. Strategies: HyDE (hypothetical document embedding), decomposition (split compound queries), entity expansion (add linked entities).
2. **Federated search.** Parallel retrieval against the stacks (dense + sparse) and every catalog in the default federation. Each source returns ranked candidates. Catalog results are treated as first-class but may carry adapter-specific score normalization.
3. **Merge.** Results are unioned with reciprocal-rank-fusion as a baseline merge across sources.
4. **Rerank.** A cross-encoder reranker (configurable; default to a small local model, pluggable to API) rescores the unioned set against the original query. Held and cataloged battens are reranked together — the reranker sees text (for held) or snippet (for cataloged).
5. **Context assembly.** Top-N battens after rerank are assembled. For cataloged battens, the caller decides whether to fetch full content or accept the snippet. Parent context, annotations, and provenance travel with the drop regardless of source.
6. **Cue logging.** The drop is written to the cue sheet. Later citation feedback (which battens the agent actually used) is attached if the caller reports it.

## Graph overlay

Entity extraction runs at rig time. Entities (people, products, concepts, identifiers) are stored in a sidecar graph. Two uses:

1. **Query expansion.** "Tell me about Cilium" can pull all battens linked to the Cilium entity, not just lexically matching ones.
2. **Navigation.** A flyperson reviewing a batten can see every other batten linked to the same entities.

The graph is explicitly lightweight. Not a full KG — that's KOS's job.

## Provenance and confidence

Every batten carries a `Provenance` (where it came from, when, who rigged it) and a `Confidence` state mirroring KOS:

- **Bedrock** — spiked, corroborated, stable.
- **Frontier** — default state for newly rigged material. Active, usable, provisional.
- **Graveyard** — struck but preserved. Not returned in retrieval.

This three-state model aligns Flyloft's lifecycle with KOS's and makes promotion a natural operation: promoting a Flyloft bedrock batten into KOS is a lossless transfer.

## Technology choices (initial)

- **Language:** Rust. Aligns with forestage.
- **Vector store:** LanceDB. Embedded, file-backed, columnar, plays well with git-ignored derived artifacts.
- **Sparse index:** Tantivy (Rust BM25).
- **Embeddings:** Pluggable. Default to a local small model (e.g. BGE-small via candle or onnx); optional API backends for larger.
- **Reranker:** Pluggable. Default to a local cross-encoder; optional Cohere/Voyage/API backends.
- **Entity extraction:** Pluggable. Default to a local NER; optional LLM-based for high-value rigs.
- **MCP:** Use the standard MCP Rust SDK.
- **Git integration:** `git2` crate.

All model backends are abstracted behind traits so the default stack can run fully local on a developer machine while production deployments can opt into hosted models.

## Non-goals (v1)

- Multi-tenant hosting (each team runs their own Flyloft).
- Real-time ingestion from arbitrary streams (batch rigging only).
- Access control beyond repository-level permissions.
- A custom UI (Spectacle can render views; Flyloft itself ships CLI + MCP).
