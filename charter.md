# Project Charter

What we know, what's open, what's been ruled out.

Follows the kos process: Orient → Question → Probe → Harvest → Promote.
Authoritative graph: `_kos/nodes/`.
Cross-repo questions belong in the orchestrator's charter.

Design intent lives in [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md),
[`docs/CONCEPTS.md`](docs/CONCEPTS.md), and [`docs/ROADMAP.md`](docs/ROADMAP.md).
Those documents are sketches, not bedrock. Until probed and harvested,
design claims live at frontier confidence.

Last updated: 2026-04-19

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

### F2: Stage rig — multi-distance artifact model [cornerstone feature]

Committed at orc level in session-034 (see
`_kos/ideas/stage-rig-information-architecture.md` and
`_kos/nodes/frontier/question-stage-rig-information-architecture.yaml`).
One of three to five cornerstone features for flyloft. Promotes
flyloft from "curated RAG substrate with reranker and graph overlay"
to "stage rig for the production" — the flyperson curates depth and
cues, not just inclusion and provenance.

**Feature surface:**

- **Distance axis** on every batten — `backdrop | scenery | prop`.
  Sibling to `confidence`, not derived from it. Viewing-distance is
  authored; the flyperson paints-this-as-backdrop or shelves-this-as-
  prop at curation time.
- **Backdrop and scenery artifacts** are first-class batten kinds,
  not derived views. They have their own provenance, their own
  curation lifecycle, their own retention. Backdrops reference the
  props they project (honest-craft at close range: a backdrop is
  obviously a painted flat when you walk up to it, and carries
  visible pointers to the prop room).
- **New verbs** extend the flyperson's curation surface:
  `paint` (author a backdrop — deliberate work, never
  auto-summarization of props), `refresh` (revisit a backdrop after
  its props have changed and repaint if the shape has shifted),
  `cue` (assemble a scene — the right backdrops + the right props
  — for a named task or query), `strike` (retire a backdrop from
  active visibility, keep in the loft). Complements existing
  `rig | fly | spike | strike | dog | dispute | annotate | promote`.
- **Cue sheet** expands to log cue→scene assembly, not just
  individual batten retrievals. A cue is a named scene; the sheet
  records which backdrops dropped and which props loaded. This is
  the abstraction-trace instrumentation that makes industrialized
  forgetting detectable.
- **Retrieval has a distance parameter.** Default: backdrop (agent
  orients). Task-triggered with the right capability: props (agent
  works). Mid-range: scenery. Distance is compositional with
  confidence-tier and catalog-scope filters.
- **Drill-down is a walk, not a decompression.** Following a
  backdrop's reference pulls the actual prop from the catalog at
  full fidelity; it does not unpack the backdrop into larger
  summaries. Backdrop and prop coexist as different kinds of
  artifact, never substitute for each other.

**Load-bearing rule: props are never mutated to produce backdrops.**
This is the answer to recursive dilution ([@cogcanvas2026] names
"recursive dilution" as a measured failure mode where each
summarization pass compounds information loss). Backdrops are
*painted* — a separate authoring act with shape-preservation in
mind, referencing the props it projects. When props change, the
flyperson revisits the backdrop. They never auto-regenerate.

**Research grounding:** CogCanvas [@cogcanvas2026] establishes the
verbatim-grounded-artifacts argument empirically (97.5% recall vs
19% summarization); HippoRAG [@hipporag2024] provides the Personalized
PageRank activation primitive for on-demand snatch-and-pull retrieval;
DPGS [@dpgs2021] and GEC [@gec2024] characterize what shape can be
preserved at distance. Shneiderman's mantra [@shneiderman1996] is the
interaction grammar. See idea file for the full landscape and
citations.

**This bears directly on F1** (confidence tier vocabulary). If
flyloft adopts a distance axis independent of confidence, the
confidence-tier vocabulary becomes easier to resolve: epistemic
state is one axis, curation state is another, viewing distance is
a third. Mapping kos ↔ flyloft at promotion time becomes a
multi-axis transform, not a shared-vocabulary hand-wave.

**Open sub-questions** (detailed in the frontier node):
shape-features carried by each batten kind; backdrop-prop drift
detection; cue semantics and expressiveness; distance as authoring
property vs retrieval parameter; abstraction-trace instrumentation;
task-conditional projection at scale; relationship to the nine
layers (finding-020); at what scale kos requires flyloft as
infrastructure vs can simulate the rig inlined.

**This is the major feature direction for flyloft.** MVP work
(grid + hybrid retrieval + flyperson CLI, currently in
`docs/ROADMAP.md`) proceeds unchanged; the stage-rig feature
layers on top once the substrate is stable.

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
