# idea: reciprocal-rank-fusion across stacks and catalogs

Captured: 2026-05-13 (session-049, extracted from charter.md
candidate-questions list per finding-047)

## The question

Does reciprocal-rank-fusion (RRF) across stacks + catalogs
produce a useful merged set, or does per-source score
normalization matter before rerank?

RRF combines rankings from multiple retrievers (LanceDB dense,
Tantivy sparse, per-catalog retrievers) by inverse rank. It's
score-agnostic by design. But the cross-encoder reranker
expects coherent score distributions; if RRF produces a
top-k where dense and sparse contribute at different
"quality grades," the reranker may not have enough signal
to choose.

## What to probe

- Compare three pipelines: (a) raw RRF → rerank, (b) per-
  source score normalization → RRF → rerank, (c) per-source
  rerank → merge.
- Measure: rerank-friendly score distribution; final ranking
  agreement with human-judged ground truth.
- Corpus-size dependency: does the answer change at 10k vs
  100k vs 1M battens?

## Related

- docs/ARCHITECTURE.md (retrieval pipeline)
- LanceDB + Tantivy hybrid retrieval choice

## Status

Pre-frontier. Promote when corpus + ground-truth dataset
exists to run the comparison.
