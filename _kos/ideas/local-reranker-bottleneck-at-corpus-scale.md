# idea: local reranker bottleneck at corpus scale

Captured: 2026-05-13 (session-049, extracted from charter.md
candidate-questions list per finding-047)

## The question

At what corpus size does the local reranker (cross-encoder)
become the bottleneck?

flyloft's retrieval pipeline (planned) puts a cross-encoder
rerank stage on top of LanceDB+Tantifity hybrid retrieval. The
cross-encoder is expensive per pair; latency scales with
top-k (~10-50 pairs). At small corpora, retrieval dominates;
at larger corpora, rerank either dominates or top-k must
shrink.

## What to probe

- Measure rerank wall-time at k=10/20/50 across corpus sizes
  10k / 100k / 1M battens.
- Find the inflection point where rerank dominates retrieval.
- Evaluate fallbacks: smaller cross-encoder, distilled model,
  sparse-only at scale, two-stage rerank (cheap then expensive).
- Hardware sensitivity: does GPU vs CPU change the inflection?

## Related

- docs/ARCHITECTURE.md (retrieval pipeline)
- Reranker model choice (currently TBD; cross-encoder bias)

## Status

Pre-frontier. Promote when the MVP retrieval pipeline ships
and corpus accumulates enough for the measurement to be
meaningful.
