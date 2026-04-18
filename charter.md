# Project Charter

What we know, what's open, what's been ruled out.

Follows the kos process: Orient → Question → Probe → Harvest → Promote.
Authoritative graph: `_kos/nodes/`.
Cross-repo questions belong in the orchestrator's charter.

Design intent lives in [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md),
[`docs/CONCEPTS.md`](docs/CONCEPTS.md), and [`docs/ROADMAP.md`](docs/ROADMAP.md).
Those documents are sketches, not bedrock. Until probed and harvested,
design claims live at frontier confidence.

Last updated: 2026-04-18

---

## Bedrock

*Established. Evidence-based or decided with rationale.*

<!-- Nothing established yet — start probing. -->

---

## Frontier

*Actively open — under exploration, not yet resolved.*

### F1: Confidence tier vocabulary — kos-mirrored or flyloft-native?

The current data model borrows kos's three tiers (`bedrock` / `frontier`
/ `graveyard`) for battens. Stated benefit: `promote` is a lossless
transfer because the vocabularies match on both sides. Quiet cost: the
alignment conflates two different things.

- **kos confidence** is *epistemic*: "is this true? have we tested it?
  what evidence supports it?"
- **flyloft 'confidence'** is *curation state*: "is this spiked? struck?
  dogged? still in active retrieval?"

These aren't the same thing. A batten can be factually correct
(epistemically bedrock-worthy if promoted) but still `frontier` in
flyloft simply because the flyperson hasn't spiked it yet. Conversely,
a spiked batten might be wrong in a way that would prevent kos
promotion. The shared vocabulary hides the mismatch.

**Alternative shape:** flyloft uses its own vocabulary reflecting
retrieval lifecycle — e.g. `active` / `promoted` / `struck`, or
orthogonal boolean flags for `spiked` / `dogged` / `struck` as the
curation state, separate from any confidence claim. The `promote` verb
then does an explicit mapping at the boundary — not a lossless
transfer.

**Question:** does tier alignment with kos produce enough bridge value
to justify the semantic compromise? Or should `promote` formalize as a
boundary operation that bridges two distinct models?

Related: `docs/ARCHITECTURE.md` Provenance and confidence section,
`docs/CONCEPTS.md` Flyloft↔KOS section. Also bears on `docs/MCP.md`
where `confidence: ["bedrock", "frontier"]` is exposed as an MCP filter
— that leaks the kos-aligned vocabulary into the agent API.

### Other candidate questions (from the design documents)

Not yet promoted to node form:

- Is the held + cataloged unification viable at query time, or does it
  force a second-class experience on one or the other?
- Does reciprocal-rank-fusion across stacks + catalogs produce a useful
  merged set, or does per-source score normalization matter before
  rerank?
- At what corpus size does the local reranker (cross-encoder) become
  the bottleneck?
- Does the flyperson's grooming surface actually pay off — is the cue
  sheet telemetry strong enough signal to drive curation?
- What breaks first when federation expands beyond 2-3 catalogs?

Promote these into `_kos/nodes/frontier/` once framed as probes.

---

## Graveyard

*Tried, ruled out, permanently recorded.*

<!-- Nothing ruled out yet. -->
