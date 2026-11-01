# finding-flyloft-28ki: distance is stored on the batten and selected by retrieval, and the stored half is built

Date: 2026-10-26
Status: final
Subject: flyloft (the distance axis on battens, `flyloft-core`)
Refs: flyloft#63, #64, #65, #66, #72; `docs/design/distance-axis.md`; charter F2

## Why it matters

The stage-rig node said the distance axis was ratified and its mechanics undesigned. Since then the design was written, the operator ruled the one open question, and the first six design items were built and tested. The node was stale, and the paint verb cannot start until the type change it needs is on record.

## The question and the options

Sub-question D: is distance a property of the batten, set when it is authored, or a parameter of retrieval? The design (#64) set three options against the charter:

| option | what it means | outcome |
|---|---|---|
| A. authored property only | stored on the batten; retrieval cannot ask for a distance | contradicts charter F2 ("Retrieval has a distance parameter") |
| B. retrieval parameter only | no stored field; one batten surfaces at whatever depth a query asks | refused: contradicts F2 ("Viewing-distance is authored") and C4 (backdrops are painted, never derived from props), and would let a prop be rendered as a backdrop, which C1 calls a type error |
| C. both, composed | the stored distance is what the batten is; the retrieval parameter selects which stored distances come back | ruled |

## The ruling

The operator ruled option C on 2026-10-06 ("flyloft distance (C)", recorded in #65 and shown in the design's status line in #66). The composition rule: retrieval never changes a batten's distance. A request for distance `d` returns battens whose stored distance is `d`, and nothing else. When none match it says so and names the counts at the other distances. It never substitutes a prop for a missing backdrop, because backdrop and prop coexist and never substitute (F2).

## What was built (#72, 2026-10-26)

Design items 1 to 6, in `flyloft-core`:

- `Distance` (backdrop, scenery, prop) on `Batten`, defaulting to prop, so a batten written before the field reads as a prop.
- `projects` on backdrop and scenery battens, the props they point at.
- Three refused shapes, at construction and at load: a prop that has `projects`; a backdrop or scenery batten with none; and one whose content is not held text.
- `distance` and `projects` as private fields with getters, so distance cannot change after creation. Repainting a backdrop makes a new batten.
- An optional distance on `Cue`, the distance a retrieval asked for.

## What is still open

- The paint, refresh and cue verbs (p4t1.5 and p4t1.6) and the `--distance` flag on `fly`: not built.
- O1: which line set a painted backdrop hangs in. A painted batten has no source document. Decided in p4t1.5.
- O2: whether a backdrop that projects a struck prop is still valid. The candidate is that it loads and drift detection reports it.
- A distance axis in the kos node schema (p4t1.9).
