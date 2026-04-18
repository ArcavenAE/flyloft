# Flyloft

> *A curated, graph-aware retrieval substrate for human-AI teams. Rig the materials. Fly them in on cue. Strike what's stale.*

Flyloft is a private-deployable reference system designed for small teams of humans and AI agents working together on sustained, evolving problems — security operations, platform engineering, research programs, anywhere a shared working memory needs to grow, be groomed, and be trusted.

It is a RAG substrate with a reranker, a graph overlay, and a curation surface — but more importantly, it is a **role**: a home for the emerging researcher-librarian, the *flyperson*, who tends the collection that the rest of the troupe draws from.

## Where it sits

Flyloft is part of the Arcaven theatrical stack:

| Project      | Role                                                         |
|--------------|--------------------------------------------------------------|
| Forestage    | User-facing surface (current generation)                     |
| Apron        | User-facing surface (next generation)                        |
| Spectacle    | Visualization and display layer                              |
| Sideshow     | Auxiliary tools and experiments                              |
| Marvel       | Umbrella / org-level platform                                |
| **Flyloft**  | **Retrieval substrate — where references hang and fly in**   |
| KOS          | Authoritative spec graph (the promptbook)                    |
| forestage      | Director/Supervisor/Crew multi-agent orchestration           |

**Flyloft is the raw, discoverable corpus. KOS is the distilled, authoritative spec layer.** Material is *rigged* into Flyloft, *flown* to agents on demand, and — once proven and stable — *promoted* into KOS.

Flyloft also federates. Alongside the local stacks (material the team has rigged), Flyloft queries **catalogs** — registered external sources exposed through adapters (GitHub, Confluence, peer Flylofts, research databases). Held and cataloged battens are curated with the same verbs. See [`docs/CATALOGS.md`](docs/CATALOGS.md).

## Deployment modes

Flyloft is designed to run two ways:

1. **Embedded in Marvel** — teams hosted inside Marvel get a Flyloft instance as part of the platform, integrated with Forestage/Apron and the rest of the surface.
2. **Standalone (BYOA)** — Bring Your Own Agents: run Flyloft as a private service against any agent runtime (Claude Code, forestage, a custom stack). Expose via MCP and a REST fallback.

Same binary, same data model, different front doors.

## Quickstart (placeholder)

```bash
# Initialize a new flyloft at the current directory
flyloft init

# Rig some source material
flyloft rig ./docs/runbooks/
flyloft rig https://example.com/article

# Start the MCP server so agents can fly material in
flyloft serve --mcp

# Open the flyperson's grooming surface
flyloft groom
```

## The Flyperson

A Flyloft without a flyperson is a pile. The flyperson is the team member — sometimes a dedicated role, more often a rotating responsibility — who:

- reviews what agents are asking for and what they're finding
- rigs new sources
- spikes battens that prove authoritative
- strikes what's stale
- promotes mature material into KOS
- watches the cue sheet for retrieval gaps

This role is the dramaturg of the human-AI team. Flyloft is the workshop they work in.

## Status

Design-stage. Schema, architecture, and interface specs are in `docs/`. A minimal Rust workspace skeleton lives in `crates/`. First implementation milestone is in `docs/ROADMAP.md`.

## Further reading

- [`docs/CONCEPTS.md`](docs/CONCEPTS.md) — theatrical terminology and the flyperson role
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — technical architecture
- [`docs/CATALOGS.md`](docs/CATALOGS.md) — external sources, federation, adapter contract
- [`docs/MCP.md`](docs/MCP.md) — agent-facing tool spec
- [`docs/CLI.md`](docs/CLI.md) — flyperson command surface
- [`docs/INTEGRATION.md`](docs/INTEGRATION.md) — Marvel-embedded vs BYOA
- [`docs/ROADMAP.md`](docs/ROADMAP.md) — phased build plan
