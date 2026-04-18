# MCP Tool Surface

Flyloft's primary agent interface is an MCP server. Tools are organized around the theatrical verbs. The set is deliberately small: retrieval, contribution, annotation, and a peek at the cue sheet. Administrative operations (strike, spike, dog, promote, weed) are flyperson tools on the CLI, not agent tools, because they carry curation authority.

## Tools

### `flyloft_fly`

Fly material in for a query. The primary retrieval entry point. Queries the union of the local stacks and every catalog in the default federation; results are merged and reranked together. Held and cataloged battens arrive with the same shape — a `catalog` field is populated only for cataloged battens.

**Input**
```json
{
  "query": "string",
  "k": 8,
  "filters": {
    "line_set": "optional line-set id or glob",
    "confidence": ["bedrock", "frontier"],
    "entities": ["optional entity names"],
    "rigged_after": "optional ISO-8601 timestamp",
    "sources": "stacks | catalogs | all"
  },
  "expand_parents": true,
  "rewrite": "auto | off | hyde | decompose"
}
```

**Output**
```json
{
  "cue_id": "cue_01hxyz...",
  "drop": [
    {
      "batten_id": "bat_01hxyz...",
      "line_set_id": "ls_01hxyz...",
      "text": "string (held) OR snippet (cataloged)",
      "content_kind": "held | cataloged",
      "catalog": { "id": "arcaven-github", "external_id": "...", "url": "..." },
      "score": 0.84,
      "parent_text": "optional parent context if expand_parents and held",
      "provenance": { "source": "...", "rigged_at": "...", "rigged_by": "..." },
      "confidence": "bedrock | frontier",
      "spiked": true,
      "annotations": [ { "by": "...", "at": "...", "note": "..." } ],
      "entities": ["Cilium", "EKS"]
    }
  ]
}
```

For cataloged battens, `text` contains the snippet captured at rig (or cache) time; call `flyloft_fetch` with `resolve: true` to materialize full content.

### `flyloft_fetch`

Retrieve the full content and metadata of a specific batten, including parent context and siblings. For cataloged battens, this is the point at which the catalog is queried to resolve full text (unless a valid cache entry exists).

**Input**
```json
{
  "batten_id": "bat_01hxyz...",
  "include_siblings": false,
  "include_full_line_set": false,
  "resolve": true
}
```

`resolve: true` (default) will fetch full content from the catalog for cataloged battens. `resolve: false` returns only what's available without a network call (held text or cached snippet).

### `flyloft_contribute`

Submit new material to be rigged. Contributions enter a *pending* state and surface to the flyperson for review before being indexed.

**Input**
```json
{
  "source": {
    "type": "text | url | file",
    "content": "...",
    "title": "optional",
    "origin": "where this came from"
  },
  "rationale": "why this should be rigged",
  "suggested_entities": ["optional list"]
}
```

**Output**
```json
{
  "pending_id": "pending_01hxyz...",
  "status": "awaiting_review"
}
```

Rationale: agents can ingest, but cannot index unreviewed. This preserves the flyperson's curation authority and prevents corpus pollution from agents-ingesting-agents loops.

### `flyloft_annotate`

Attach a note to an existing batten. Useful when an agent discovers a correction, a clarification, or a cross-reference during use.

**Input**
```json
{
  "batten_id": "bat_01hxyz...",
  "note": "string",
  "kind": "correction | clarification | cross_reference | caution"
}
```

Annotations are immediately visible but flagged as agent-contributed until reviewed. A flyperson can promote, edit, or remove them during grooming.

### `flyloft_dispute`

Flag a batten as wrong, stale, or contested. Surfaces prominently in the flyperson's grooming queue.

**Input**
```json
{
  "batten_id": "bat_01hxyz...",
  "reason": "string",
  "evidence": "optional string or url"
}
```

A disputed batten stays retrievable (with a `disputed: true` flag) until the flyperson resolves it by striking, annotating, or dismissing the dispute.

### `flyloft_cite`

Report back that a previously flown batten was actually used in the agent's output. Closes the feedback loop for retrieval quality.

**Input**
```json
{
  "cue_id": "cue_01hxyz...",
  "cited_battens": ["bat_01hxyz...", "bat_02hxyz..."]
}
```

The cue sheet uses citation telemetry to distinguish *retrieved* from *useful*. This is the primary signal for the flyperson's weeding operations.

### `flyloft_entities`

Get entities linked to a query or batten, for graph-aware follow-up.

**Input (query mode)**
```json
{ "query": "string" }
```

**Input (batten mode)**
```json
{ "batten_id": "bat_01hxyz..." }
```

**Output**
```json
{
  "entities": [
    {
      "name": "Cilium",
      "kind": "technology",
      "batten_count": 47,
      "representative_battens": ["bat_...", "bat_..."]
    }
  ]
}
```

## Tools *not* exposed over MCP

The following are flyperson operations, exposed only via CLI:

- `strike` — retiring material
- `spike` — marking authoritative
- `dog` — freezing a line set
- `promote` — moving to KOS
- `weed` — identifying retirement candidates
- `groom` — the interactive curation session

Agents *propose* changes (via `contribute`, `annotate`, `dispute`); the flyperson *commits* them. This is the architectural embodiment of the curator role.

## Authentication and scoping

- Each MCP connection is authenticated via a token associated with a *role* (agent, flyperson, observer).
- Agent tokens can call the agent-facing tools but never the flyperson operations.
- Tokens can be scoped to a subset of the grid (e.g. an agent working on SecOps gets an agent token scoped to SecOps line sets).

## MCP profile compatibility

Flyloft's MCP server is designed to support per-session profile scoping — so an forestage Supervisor can bring only a relevant subset of Flyloft tools into a Crew session rather than bleeding the full surface into every context. This is the Flyloft-side answer to the MCP profile scoping gap identified in forestage's design.
