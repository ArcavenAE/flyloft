# CLI — The Flyperson's Tools

The CLI is the primary surface for the flyperson. It wraps all curation operations, exposes the grooming workflow, and is the only path to privileged operations like strike, spike, and promote.

The command structure mirrors the theatrical verbs one-to-one.

## Lifecycle commands

### `flyloft init`

Initialize a new flyloft in the current directory. Creates the storage layout, writes a starter `flyloft.toml`, and optionally wires up a git remote.

```bash
flyloft init --name "mss-flyloft" --remote git@github.com:ArcavenAE/mss-flyloft.git
```

### `flyloft serve`

Start the MCP server (and optional REST endpoint). This is the daemon for embedded and BYOA deployments.

```bash
flyloft serve --mcp --port 7070
flyloft serve --mcp --rest --port 7070  # both
```

### `flyloft status`

One-screen summary of the grid: line set counts, batten counts, cues logged, pending contributions, open disputes.

## Rigging

### `flyloft rig <source>`

Ingest material. Sources can be files, directories, URLs, or structured exports. The command chunks, embeds, extracts entities, and writes battens.

```bash
flyloft rig ./docs/runbooks/
flyloft rig https://example.com/article
flyloft rig --transcript meeting-2026-04-10.txt --speakers "Avi, Michael, Noah"
flyloft rig --line-set-id custom_id ./source.md
```

Flags:
- `--dry-run` — report what would be rigged without writing.
- `--tag <tag>` — attach a tag to the line set.
- `--rigged-by <name>` — override the contributor (defaults to git user).
- `--chunk-strategy <semantic|fixed|document>` — override default chunking.

### `flyloft contribute review`

Review pending agent-contributed material.

```bash
flyloft contribute review          # list pending
flyloft contribute accept <id>     # rig it
flyloft contribute reject <id>     # discard with optional reason
flyloft contribute defer <id>      # hold for later
```

## Retrieval (for human use)

### `flyloft fly <query>`

Same as the MCP `flyloft_fly` tool, from the terminal. For the flyperson spot-checking what the collection knows.

```bash
flyloft fly "how does cilium handle egress policy"
flyloft fly "sfrclak c2 domain" --filter confidence=bedrock
flyloft fly "..." --catalogs only    # skip local stacks; query catalogs only
flyloft fly "..." --catalogs none    # skip catalogs; query stacks only
flyloft fly "..." --catalog arcaven-github   # query one specific catalog + stacks
```

## Curation — the privileged verbs

### `flyloft spike <batten-id>`

Mark a batten as authoritative. Spiked battens are weighted higher in retrieval and are candidates for promotion to KOS.

```bash
flyloft spike bat_01hxyz...
flyloft spike bat_01hxyz... --reason "corroborated against vendor doc"
```

### `flyloft strike <batten-id | line-set-id>`

Retire material. Moved to graveyard confidence. Removed from active retrieval. Preserved in the grid for audit.

```bash
flyloft strike bat_01hxyz...
flyloft strike ls_01hxyz... --reason "superseded by 2026 version"
```

### `flyloft dog <line-set-id>`

Lock a line set against modification. Useful for reference material that must not drift.

```bash
flyloft dog ls_01hxyz...
flyloft dog ls_01hxyz... --release  # unfreeze
```

### `flyloft annotate <batten-id>`

Attach a note. Opens `$EDITOR` if no inline text is given.

```bash
flyloft annotate bat_01hxyz... --note "see KOS node kos:cilium-egress-policy"
flyloft annotate bat_01hxyz...  # opens editor
```

### `flyloft dispute <batten-id>`

Register or resolve a dispute.

```bash
flyloft dispute bat_01hxyz... --reason "..."
flyloft dispute resolve <dispute-id> --action strike
flyloft dispute resolve <dispute-id> --action annotate --note "..."
flyloft dispute resolve <dispute-id> --action dismiss
```

## Promotion to KOS

### `flyloft promote <batten-id> --to kos`

Promote a spiked batten into KOS as a bedrock node. Requires KOS to be configured in `flyloft.toml`.

```bash
flyloft promote bat_01hxyz... --to kos --node-kind concept
```

The batten remains in Flyloft with a `promoted_to` pointer added.

## Grooming and weeding

### `flyloft groom`

The flyperson's interactive session. Presents, in priority order:

1. **Disputes** awaiting resolution.
2. **Pending contributions** from agents.
3. **Weeding candidates** — battens never cited, stale by age, superseded by newer rigs.
4. **Spike candidates** — battens cited frequently across many cues.
5. **Promotion candidates** — spiked battens ready for KOS.

Each item presents options: accept, reject, defer, inspect, batch-action.

### `flyloft weed`

Non-interactive report of retirement candidates.

```bash
flyloft weed --stale 90d              # rigged >90 days ago, never cited
flyloft weed --disused                # never cited across >N cues
flyloft weed --superseded             # near-duplicates where a newer version exists
```

### `flyloft cue`

Query the cue sheet.

```bash
flyloft cue recent                    # recent drops
flyloft cue gaps                      # queries that returned nothing
flyloft cue coldspots                 # frequently retrieved, rarely cited
flyloft cue hotspots                  # frequently cited (promotion candidates)
flyloft cue by-requester <id>         # what a specific agent has been asking for
```

The cue sheet is the flyperson's primary signal source. `cue coldspots` and `cue gaps` are the two commands that should run every grooming session.

## Entity operations

### `flyloft entity list`

```bash
flyloft entity list
flyloft entity list --kind technology
flyloft entity show "Cilium"         # all battens + relationships
flyloft entity merge "cilium" "Cilium"  # canonicalize duplicates
```

## Catalogs

### `flyloft catalog`

Manage registered external sources.

```bash
flyloft catalog list                      # all registered catalogs + health
flyloft catalog health                    # ping each adapter, report status
flyloft catalog add --id arcaven-github \
    --adapter github \
    --federate \
    --config org=ArcavenAE repos=flyloft,forestage token-env=GITHUB_TOKEN
flyloft catalog remove arcaven-github
flyloft catalog describe arcaven-github   # adapter config, cache policy, recent usage
```

### `flyloft rig --catalog`

Rig a slice of a catalog into the stacks permanently — for when you want the bytes held locally, not just pointed at.

```bash
flyloft rig --catalog arcaven-github --query "cilium egress" --tag networking
flyloft rig --catalog team-confluence --collection-id "MSS-RUNBOOKS"
```

### `flyloft adopt <batten-id>`

Convert a cataloged batten into a held batten. Fetches the current content from the catalog, copies it into the stacks, records the catalog origin in provenance.

```bash
flyloft adopt bat_01hxyz...
flyloft adopt bat_01hxyz... --reason "pinning for offline incident response"
```

Use adopt when a cataloged batten is load-bearing enough that availability risk is unacceptable — the catalog could be rate-limited, paywalled, or go offline during an incident.

## Config and introspection

### `flyloft config`

```bash
flyloft config show
flyloft config set embedding.backend local-bge-small
flyloft config set reranker.backend cohere
```

### `flyloft doctor`

Health check: index integrity, orphaned battens, corrupt line sets, config sanity.

```bash
flyloft doctor
flyloft doctor --fix    # auto-repair what can be auto-repaired
```

### `flyloft reindex`

Rebuild the derived index from the grid. Safe to run anytime; the grid is source of truth.

```bash
flyloft reindex
flyloft reindex --only dense
flyloft reindex --only sparse
```
