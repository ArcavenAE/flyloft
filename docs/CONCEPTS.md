# Concepts

Flyloft's vocabulary is borrowed from theatrical fly systems — the machinery above the stage where scenery, backdrops, and set pieces hang on rigged lines, ready to be lowered ("flown in") exactly when the scene calls for them. This is the most mechanically honest metaphor in the theatrical vocabulary for what a retrieval system does: stored material, indexed on a rail, delivered on cue.

The vocabulary is not decoration. It shapes the verbs, the mental model, and the role.

## Terminology

| Term            | Meaning in Flyloft                                                                        |
|-----------------|-------------------------------------------------------------------------------------------|
| **Grid**        | The overall storage structure. The corpus in aggregate.                                   |
| **Line set**    | A versioned group of battens from one source (e.g. one document and its chunks).          |
| **Batten**      | The atomic unit of indexed material: a chunk plus its metadata and provenance.            |
| **Fly rail**    | The indexing and search surface (dense + sparse + reranker).                              |
| **Pin rail**    | The manual curation control surface.                                                      |
| **Cue sheet**   | The retrieval log — what was flown in, when, for whom, and whether it was used.           |
| **Flyperson**   | The human curator. (Historically "flyman"; we use the gender-neutral form.)               |
| **Drop**        | A single retrieval event. A set of battens flown in to answer a query.                    |

## Verbs

| Verb            | Operation                                                                                 |
|-----------------|-------------------------------------------------------------------------------------------|
| **Rig**         | Ingest source material: parse, chunk, embed, index.                                       |
| **Fly in**      | Retrieve: query the fly rail, return ranked battens.                                      |
| **Strike**      | Retire a line set or batten. Removed from active retrieval, preserved in archive.         |
| **Spike**       | Mark a batten as authoritative. Candidate for promotion into KOS bedrock.                 |
| **Dog**         | Lock a line set against modification. (A *dog* is a real fly-rail locking mechanism.)     |
| **Dispute**     | Flag a batten as wrong, stale, or contested. Surfaces for flyperson review.               |
| **Annotate**    | Attach a note, correction, or cross-reference to a batten without modifying its source.   |
| **Groom**       | The flyperson's interactive curation session.                                             |
| **Promote**     | Move a spiked batten out of Flyloft and into KOS as bedrock.                              |
| **Weed**        | Identify retirement candidates by staleness, disuse, or supersession.                     |

## Beyond the stacks: catalogs

The local grid is the team's **stacks** — what's been rigged, chunked, and committed. But a flyperson's job has never stopped at the shelves they own. Flyloft extends the same curation surface to external sources through **catalogs**: adapter-wrapped connections to GitHub, Confluence, peer Flyloft instances, research databases, and arbitrary REST APIs.

| Term                   | Meaning                                                                           |
|------------------------|-----------------------------------------------------------------------------------|
| **Stacks**             | The local grid. Held battens.                                                     |
| **Catalog**            | A registered external source with an adapter.                                     |
| **Held batten**        | Text lives in the grid. Always available.                                         |
| **Cataloged batten**   | A pointer with snippet and provenance. Full text fetched on demand.               |
| **Fetch**              | Resolve a cataloged batten's full content from its catalog.                       |
| **Loan**               | Special case: fetching from a peer Flyloft.                                       |
| **Adopt**              | Convert a cataloged batten into a held one (copy the bytes into the stacks).      |
| **Paging**             | Requesting content be fetched up from a catalog (library term; retained).         |

The curation verbs (rig, spike, strike, dog, annotate, dispute, promote) work identically on both. A flyperson spiking a JSTOR article doesn't move the article into the grid; the spike asserts authority, not ownership.

See `docs/CATALOGS.md` for the full catalog concept, adapter contract, and configuration.

## The Flyperson

In a traveling troupe, the flyman is the crew member who knows what's rigged above the stage, which line brings in which piece, and which counterweights are set for tonight's show. They are invisible to the audience. They are indispensable to the performance.

The flyperson in a human-AI team occupies the same structural position. They:

1. **Review the cue sheet.** What have agents been asking for? What queries went cold? What battens get flown in repeatedly without ever being cited in finished work?
2. **Rig new material.** Turn a meeting transcript, a vendor doc, a postmortem, a paper into indexed battens.
3. **Register and tend catalogs.** Decide which external sources the team reaches into, which default into federation, which require explicit loan. Retire catalogs whose quality slips.
4. **Spike and dog.** When a batten proves authoritative — held or cataloged — spike it. When a line set is stable and should not drift, dog it.
5. **Strike and weed.** When material is superseded, wrong, or simply never used, strike it. The collection stays lean.
6. **Promote into KOS.** The mature, spiked, corroborated battens graduate out of Flyloft and become bedrock nodes in the knowledge graph.

This is not a janitorial role. The cue sheet gives the flyperson *signal* — real retrieval telemetry, real usage patterns, real gaps. A flyperson working a live Flyloft is closer to a dramaturg or a research librarian than an archivist: they shape what the team knows by shaping both what the team can find locally *and* what sources the team can reach.

## Flyloft ↔ KOS

Flyloft and KOS are the two halves of a working knowledge system:

- **Flyloft is the casebook.** Raw material, provisional, inclusive, discoverable. Everything the troupe has gathered.
- **KOS is the promptbook.** Distilled, authoritative, spec-shaped. What the troupe has *decided*.

Material flows **Flyloft → KOS** through the promote verb, which is the Spec Cycle's `harvest → promote` step realized as a concrete operation. Material occasionally flows back **KOS → Flyloft** when a bedrock node is demoted to frontier during revision.

The two systems share provenance pointers. A KOS node knows which Flyloft battens it was harvested from. A Flyloft batten knows which KOS nodes cite it. This is the audit trail for the team's epistemology.

## What Flyloft is not

- **Not a chat memory.** Conversation logs may be rigged as sources, but Flyloft is not a transcript store.
- **Not a general document management system.** No sharing permissions, no co-editing, no versioned Word documents. Source material lives elsewhere; Flyloft indexes and curates it.
- **Not an agent runtime.** Flyloft is called *by* agents (via MCP) and *by* humans (via CLI). It does not orchestrate them. That's forestage's job.
- **Not a replacement for KOS.** Flyloft is the discoverable corpus; KOS is the decided spec graph. They complement; they do not compete.
