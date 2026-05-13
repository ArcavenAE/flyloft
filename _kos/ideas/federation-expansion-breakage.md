# idea: what breaks first when federation expands beyond 2-3 catalogs?

Captured: 2026-05-13 (session-049, extracted from charter.md
candidate-questions list per finding-047)

## The question

What breaks first when federation expands beyond 2-3 catalogs?

flyloft's catalog adapters expose external sources (GitHub,
Confluence, peer flyloft, REST, KOS-as-catalog) through a
common interface. At 2-3 catalogs the per-adapter latency,
auth scope, and ranking-coherence costs are manageable. At
10+ catalogs they may not be.

## What to probe

- Identify the dominant failure mode by simulation:
  - Latency fan-out (Nth catalog dominates p95)?
  - Score-distribution drift (each catalog ranks differently;
    fusion becomes meaningless)?
  - Auth/credential management (per-catalog tokens, rotation,
    failure handling)?
  - Schema/relevance drift (cataloged content's "what counts as
    a match" varies)?
- Set up a benchmark with 2/5/10 catalogs and measure each.
- Identify the breaking point per failure mode; the lowest is
  the binding constraint.

## Related

- docs/CATALOGS.md (catalog adapter design)
- Catalog trait + registry in flyloft-core

## Status

Pre-frontier. Promote after MVP + initial catalog adapters
ship, when there's a concrete second/third catalog to test
against.
