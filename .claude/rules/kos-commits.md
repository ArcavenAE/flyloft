# kos Commit Conventions

## kos-specific actions (from KOS process cycle)

In addition to conventional commit types (feat, fix, docs, etc.),
use these kos action types when working with the knowledge graph:

- `harvest`: update nodes after a probe cycle completes
- `promote`: move a node to a higher confidence tier
- `graveyard`: move a node to graveyard (ruled out)
- `probe`: begin or continue an exploration
- `finding`: write a finding from a probe
- `schema`: update the node schema
- `charter`: update the charter document

### Format
`[action]: [node-ids affected] — [one line description]`

### Examples
```
harvest: question-catalog-adapter — probe complete, see finding-007
promote: elem-batten-content-model — validated across held + cataloged
graveyard: grv-single-index-only — ruled out, see graveyard entry
probe(flyloft): hybrid retrieval — dense vs sparse vs hybrid quality
finding(flyloft): reranker-latency — cross-encoder latency acceptable at k=20
```

### No AI Attribution
Do not add "Generated with Claude Code", "Co-Authored-By: Claude", or any
AI attribution to commits. The human is the author.
