# Example grid layout

This directory is a minimal working example of what Flyloft's on-disk grid looks like. It's pedagogical — if you're reading this to understand how Flyloft persists state, start here.

## Structure

```
examples/grid/
├── line-sets/
│   ├── ls_01hxxxegress/                      # held line set (Cilium docs)
│   │   ├── manifest.yaml
│   │   ├── source.md                         # preserved original source
│   │   └── battens/
│   │       ├── bat_aaaa...yaml              # frontier batten
│   │       ├── bat_bbbb...yaml              # spiked, annotated (bedrock)
│   │       └── bat_cccc...yaml              # struck (graveyard)
│   └── ls_01hxxxgithub_cilium/               # cataloged line set (GitHub)
│       ├── manifest.yaml                     # Source::Catalog
│       └── battens/
│           └── bat_dddd...yaml              # cataloged batten (pointer, no full text)
└── cues/
    └── 2026-04.jsonl                         # sample retrieval log entries
```

## What the example shows

Two line sets, illustrating both content modes:

1. **Held line set** (`ls_01hxxxegress`) — a short Cilium egress policy doc rigged as three chunks. A flyperson has worked it: spiked one batten as authoritative, struck another that was subtly wrong, attached a cross-reference annotation. Source preserved alongside the battens.

2. **Cataloged line set** (`ls_01hxxxgithub_cilium`) — a slice of GitHub issues/PRs matching "cilium egress" in the Arcaven GitHub catalog. The single example batten points to issue #47 rather than holding its full text. A snippet is cached for retrieval-time display; full content is fetched from GitHub on demand with a 1-hour TTL. An annotation notes the intent to `flyloft adopt` once the discussion resolves.

Together they demonstrate that held and cataloged battens share identity, provenance, confidence state, annotations, and entity links — only the `content` field differs.

## Reading a batten

Each batten YAML contains:

- `id` — content-addressable identifier
- `line_set` — which line set it belongs to
- `text` — the chunk itself
- `position` — where in the source it came from
- `provenance` — who rigged it, when, how
- `confidence` — bedrock / frontier / graveyard
- `spiked` / `dogged` — curation flags
- `annotations` — flyperson or agent notes
- `entities` — graph overlay links
- `promoted_to` — KOS node id if promoted

The files are deliberately human-readable and diffable. A curation session produces git diffs a reviewer can read.
