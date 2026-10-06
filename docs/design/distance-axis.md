# Design: the distance axis and backdrop and scenery battens

**Status:** option C ruled by the operator, 2026-10-06; the type change below is
still proposed. Nothing here is built.
**Ticket:** `aae-orc-p4t1.1`, flyloft#63. Unblocks `aae-orc-p4t1.5` (the paint verb MVP).
Answers `aae-orc-p4t1.7` (sub-question D) by citing rulings already made.
**Sources:** charter F2; `_kos/nodes/frontier/question-stage-rig-lod-axis.yaml`;
orc `_kos/ideas/stage-rig-information-architecture.md` (C1 to C7).

## Why

The paint verb (p4t1.5) has to write a backdrop and say what distance it
was painted for, and today a batten cannot record either. flyloft main
(9c1f7fe) has no distance field, no backdrop kind and no paint verb, and
the kos node schema has no distance axis. This design adds the smallest
type change that lets paint land, and settles the one question that
decides its shape: is distance stored at all.

## Sub-question D first

**Question:** is distance a property of the batten, set when it is
authored, or a parameter of retrieval?

| Option | What it means | Status |
|---|---|---|
| A. Authored property only | stored on the batten; retrieval cannot ask for a distance | contradicts charter F2 ("Retrieval has a distance parameter") |
| B. Retrieval parameter only | no stored field; one batten surfaces at whatever depth a query asks | contradicts charter F2 ("Viewing-distance is authored") and C4 (backdrops are painted, never derived from props); it would let a prop be rendered as a backdrop, which C1 calls a type error |
| C. Both, composed | the stored distance is what the batten is; the retrieval parameter selects which stored distances come back | **ruled** 2026-10-06 |

**Ruling: C.** The operator ruled it on 2026-10-06: "flyloft distance (C)".
It was already ruled in substance, not open. The flyloft node
`question-stage-rig-lod-axis` records the data-model axis as ratified into
flyloft on 2026-07-02 ("batten schema gains the distance field at Phase
0"), and charter F2 states both halves. What remained open was how the two
compose, which the rules below answer. So p4t1.1 does add a stored field.

**The composition rule:** retrieval never changes a batten's distance. A
request for distance `d` returns battens whose stored distance is `d`, and
nothing else. When none match, it says so and names the counts at the other
distances. It never substitutes a prop for a missing backdrop, because
"backdrop and prop coexist, never substitute" (F2).

## The type change

All in `flyloft-core`. No CLI verb is implemented today (`fly`, `rig` and
the rest are `todo!`), so nothing persisted needs migrating.

1. **`Distance` enum**, sibling to `Confidence` in `provenance.rs`:
   `Backdrop`, `Scenery`, `Prop`, serialized snake_case, `#[non_exhaustive]`.
   Default `Prop`, because everything `rig` ingests is source material held
   verbatim. Costumes, the fourth stage-rig distance, are agent identity,
   not battens, so they are not a variant here.
2. **`Batten.distance: Distance`**, with `#[serde(default)]` so a batten
   YAML written before this field reads as a prop.
3. **`Batten.projects: Vec<BattenId>`**, the props a backdrop or scenery
   batten projects (C3: up close, a backdrop carries visible pointers to
   the prop room). `#[serde(default)]`, empty for props.
4. **One invariant, checked when a batten is built or loaded:** a prop has
   no `projects`; a backdrop or scenery batten has at least one, and its
   content is `Held` (it is authored text, not a catalog pointer). A
   violation is a load error. This is a structural check, so it may gate.
5. **Distance is fixed at creation.** `Batten`'s fields are public today,
   so `distance` and `projects` are the first two made private, with
   getters and a validated constructor (`.claude/rules/rust.md`: private
   fields where invariants must hold). No method changes distance.
   Repainting a backdrop makes a new batten; a prop never becomes a
   backdrop. This is how "mixing one distance for another is a type error"
   (C1) is held in code rather than by discipline.
6. **`Cue.distance: Option<Distance>`**, the distance a retrieval asked
   for, so the cue sheet shows whether an agent oriented or worked.

"First-class batten kinds" (p4t1.1's title) is met by the stored distance
plus the invariant: a backdrop has its own provenance (the painter as
`Contributor`), its own confidence and its own strike lifecycle, as every
batten does. No separate struct is needed, and a separate struct would make
held and cataloged handling diverge again.

## Out of scope here

- The paint, refresh and cue verbs (p4t1.5, p4t1.6) and the abstraction
  trace format (p4t1.2).
- The `--distance` flag on `fly`, which lands with Phase 1 retrieval.
  Charter F2 sets its default to backdrop; with the composition rule, an
  empty result at that default reports the counts rather than falling back.
- A distance axis in the kos node schema. kos's projection work is p4t1.9.

## Open for the author

- **O1. Which line set a backdrop hangs in.** A painted batten has no
  source document. Candidate: one line set per grid for painted material,
  so `rig` can never produce a backdrop. Decide in p4t1.5.
- **O2. Is a backdrop that projects a struck prop still valid?** Candidate:
  it loads, and drift detection (p4t1.4) reports it.

## What a builder builds

Items 1 to 6 above, red then green: unit tests for the serde default (an
old YAML reads as a prop), the invariant (each of the three refusals), and
the round trip of a backdrop with `projects`. No CLI change.
