# flyloft as an on-demand knowledge loadout ("I know kung fu")

- **Status:** idea (pre-hypothesis), operator-raised 2026-09-20.
- **Subject:** flyloft. The product framing: an agent or project loads the
  knowledge it needs for the task at hand, on demand, at the depth it needs.
  kos (the promptbook to flyloft's casebook), external RAGs and knowledge
  graphs, and the F25 stage rig are objects and prior art this framing ties
  together.
- **Tracking:** none yet. Relates [[question-stage-rig-lod-axis]] (the
  level-of-detail axis), [[federation-expansion-breakage]] (connecting external
  sources), and the orc idea token-work-profiling-to-binary-tools (loading
  knowledge costs tokens).

## The framing

flyloft is the loadout system from the Matrix: "I know kung fu." A project or
agent does not carry every domain baked in; it SWAPS IN the knowledge the
current task needs, when it needs it, and dials the detail to fit. Four moves,
each already a piece of flyloft's design, named in one metaphor a newcomer
gets immediately:

1. **Swap in ("I know kung fu").** Load the right knowledge for this project or
   task on demand, not compiled into the agent. The agent gains a domain for
   the scene it is playing and drops it after, instead of every session
   carrying everything.
2. **Improve areas of knowledge.** Curate and deepen a domain where the work
   concentrates (git-backed curation). The loadout gets sharper exactly where
   you invest, and the investment persists and is reviewable.
3. **Connect to external RAGs and graphs.** flyloft is not only the local
   casebook; it bridges to external retrieval sources and knowledge graphs so a
   loadout can draw from beyond the repo. Federation is a first-class move, not
   an afterthought.
4. **Dial resolution / level of detail.** Per the F25 stage rig, turn the depth
   up or down for the scene: backdrops (shape preserved, detail absent) for
   orientation, up to props (verbatim, full fidelity) for active work. The same
   corpus serves a quick orient and a deep dive because the caller sets the
   distance.

## Why this is flyloft's existing design in plain words

- Swap-in and dial-detail are the F25 discipline flyloft already owns: sources
  verbatim, summaries authored, task-scoped projection. The loadout metaphor is
  the accessible face of the level-of-detail axis
  ([[question-stage-rig-lod-axis]]).
- Improve-areas is the git-backed curation surface (the flyperson grooming the
  loft).
- Connect-external is federation ([[federation-expansion-breakage]] already
  weighs where that breaks).
- The retrieval underneath (hybrid dense + sparse + rerank, MCP-first) is the
  engine that makes a swap-in fast enough to feel instant.

## Open questions

- What is the LOADOUT UNIT: a knowledge pack, a domain, a project profile, a
  named "kung fu" the caller asks for by name? The swap-in verb needs a noun.
- Swap-in has a token cost: loading a domain spends context budget, so a
  loadout has to be priced and dialed against the project's budget. This is the
  direct tie to the token-work-profiling idea, and to why level-of-detail is not
  cosmetic but a budget lever.
- External federation vs verbatim authority: an external RAG or graph may be
  lossy or unattributed, and flyloft's whole stance is verbatim, cited,
  ruled-out-kept knowledge. How does a swapped-in external source carry its
  provenance and detail-class, or is it fenced as lower-trust scenery?
- The control surface for dialing detail: cue-driven (task, cwd, query, per
  F25) or an explicit resolution knob the caller sets, or both.
- Who assembles a loadout: the agent asks for a domain and flyloft resolves it,
  a supervisor provisions it for a team, or a project declares a standing
  loadout it always wants.

## Why an idea and not yet design

It is the product vision for flyloft, which today is a skeleton (types only,
verbs todo!()). It crystallizes into design when the first verb ships and the
loadout unit and the swap-in call have to take a concrete shape, most likely
when a real project needs one domain at full detail and three others at
backdrop depth in the same session.
