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

<!-- No frontier questions promoted yet — orient first.

     Candidate questions (from the design documents):
     - Is the held + cataloged unification viable at query time, or does
       it force a second-class experience on one or the other?
     - Does reciprocal-rank-fusion across stacks + catalogs produce a
       useful merged set, or does per-source score normalization matter
       before rerank?
     - At what corpus size does the local reranker (cross-encoder) become
       the bottleneck?
     - Does the flyperson's grooming surface actually pay off — is the
       cue sheet telemetry strong enough signal to drive curation?
     - How does Flyloft ↔ KOS promotion interact with kos's confidence
       tier model in practice?
     - What breaks first when federation expands beyond 2-3 catalogs?

     Promote these into `_kos/nodes/frontier/` once framed as probes.
-->

---

## Graveyard

*Tried, ruled out, permanently recorded.*

<!-- Nothing ruled out yet. -->
