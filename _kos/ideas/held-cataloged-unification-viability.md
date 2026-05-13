# idea: held + cataloged unification at query time

Captured: 2026-05-13 (session-049, extracted from charter.md
candidate-questions list per finding-047)

## The question

Is the held + cataloged unification viable at query time, or does
it force a second-class experience on one or the other?

flyloft's data model treats held battens (text in grid) and
cataloged battens (pointers to external sources) as same-shape
units behind the curation surface. But at query time, retrieval
behavior differs: held material is locally indexed (LanceDB +
Tantivy) with low-latency rerank, while cataloged material may
require live fetch + on-the-fly indexing or stale-cache reads.

## What to probe

- Does the latency gap between held and cataloged retrieval
  become user-visible at common query rates?
- Does score normalization before reciprocal-rank-fusion bridge
  the gap, or does the unified ranking still favor one tier?
- What's the right UX signal — surface the source kind in
  results, or hide it behind the unified ranking?

## Related

- docs/ARCHITECTURE.md (data model)
- docs/CATALOGS.md (catalog adapter design)
- flyloft/charter.md F1 (confidence-tier vocabulary alignment
  with kos)

## Status

Pre-frontier. Promote to `_kos/nodes/frontier/` once a probe
brief frames the question with hypothesis, timebox, and success
signal.
