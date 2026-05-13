# idea: does the flyperson's grooming surface pay off?

Captured: 2026-05-13 (session-049, extracted from charter.md
candidate-questions list per finding-047)

## The question

Does the flyperson's grooming surface actually pay off — is the
cue sheet telemetry strong enough signal to drive curation?

The flyperson role assumes that retrieval events (cues) provide
useful curation signals: which battens get returned, which get
clicked through to, which get cited downstream, which never
surface despite their presence. The cue sheet captures these
events. The hypothesis: with enough cue volume, the flyperson
can groom (rig new material, spike contested claims, strike
stale battens) based on data rather than intuition.

## What to probe

- Define minimum cue volume for grooming-by-data to outperform
  grooming-by-intuition. (Murat's threshold for sidestep's audit
  trail was 5-10 invocations/day for 8-12 weeks; analog needed
  here.)
- Identify which cue signals are load-bearing: surface-rate,
  click-through, citation, downstream-promotion.
- Test on a real flyperson workflow: does access to cue
  telemetry change curation decisions vs not having it?

## Related

- docs/CONCEPTS.md (flyperson role)
- docs/ARCHITECTURE.md (cue sheet)
- sidestep finding-001 (party-mode evidence-gating analog —
  Murat's threshold for v0.2 sugar)

## Status

Pre-frontier. Requires a running flyloft + a flyperson +
enough cues. Promote when MVP ships and usage accumulates.
