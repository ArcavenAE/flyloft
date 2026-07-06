# Flyloft

> *A curated, graph-aware retrieval substrate for human-AI teams. Rig the materials. Fly them in on cue. Strike what's stale.*

Flyloft is a private-deployable reference system designed for small teams of humans and AI agents working together on sustained, evolving problems — security operations, platform engineering, research programs, anywhere a shared working memory needs to grow, be groomed, and be trusted.

It is a RAG substrate with a reranker, a graph overlay, and a curation surface — but more importantly, it is a **role**: a home for the emerging researcher-librarian, the *flyperson*, who tends the collection that the rest of the troupe draws from.

## Where it sits

Flyloft is part of the Arcaven theatrical stack. Each component is
independently useful; the stack composes them.

| Project        | Role                                                                           |
|----------------|--------------------------------------------------------------------------------|
| **forestage**  | The agent console — persona-themed CLI wrapping Claude Code (rename to *apron* planned) |
| **marvel**     | Agent control plane — Kubernetes-like orchestration (teams, roles, sessions, shifts) |
| **tmux-cmc**   | tmux control mode client library — the wiring beneath forestage                |
| **switchboard**| Remote tmux relay — serial console for distributed agents                      |
| **director**   | Inter-agent communication (planned) — supervisor/crew coordination protocol    |
| **curtain**    | Minimal agent sandbox — kernel-enforced containment                            |
| **spectacle**  | Standards-based spec templates + Claude Code commands (ISO 29148, IEEE 1016, 42010, arc42, MADR) |
| **sideshow**   | Content pack manager — distributes rules, skills, themes across repos and AI CLI tools |
| **Flyloft**    | **Retrieval substrate — the casebook**                                          |
| **kos**        | Knowledge graph and epistemological process — the promptbook                   |
| **beads**      | Task tracker — dolt-backed issues across the fleet                             |

**Flyloft is the raw, discoverable corpus. kos is the distilled, authoritative spec layer.** Material is *rigged* into Flyloft, *flown* to agents on demand, and — once proven and stable — *promoted* into kos.

Flyloft also federates. Alongside the local stacks (material the team has rigged), Flyloft queries **catalogs** — registered external sources exposed through adapters (GitHub, Confluence, peer Flylofts, research databases). Held and cataloged battens are curated with the same verbs. See [`docs/CATALOGS.md`](docs/CATALOGS.md).

## Deployment modes

Flyloft is designed to run two ways:

1. **Managed by marvel** — teams running marvel-managed agent workloads get a provisioned Flyloft as part of their workspace, reachable over MCP by the agent sessions marvel schedules.
2. **Standalone (BYOA)** — Bring Your Own Agents: run Flyloft as a private service against any agent runtime (Claude Code, forestage, or a custom stack). Expose via MCP and a REST fallback.

Same binary, same data model, different front doors.

## Installation

Flyloft ships as a signed, notarized macOS binary (Apple Silicon) and Linux
binaries (`amd64`, `arm64`) via GitHub Releases. **The first stable release is
pending** — every release today is a prerelease from `main`.

### Option 1: Homebrew (macOS arm64)

```bash
brew install arcavenae/tap/flyloft
```

Until the first stable release is cut, the tap tracks the latest alpha.

### Option 2: Install with mise

[mise](https://mise.jdx.dev/) is a polyglot version manager. It reads a
per-project `mise.toml`, pulls the exact signed binary from GitHub Releases,
and verifies GitHub Artifact Attestations natively — no Homebrew tap required.

**Stable** — no stable release exists yet; this block starts working once the
first `v*` tag lands:

```bash
mise use github:ArcavenAE/flyloft@latest
flyloft --version
```

**Alpha channel** (prereleases from `main`) — add `prerelease = true` to opt in
per-tool. Alpha binaries are not `-a`-suffixed for flyloft, so stable and alpha
share the `flyloft` shim (installing both concurrently is not supported until
we split the release-asset naming):

```toml
# mise.toml
[tools]
"github:ArcavenAE/flyloft" = { version = "latest", prerelease = true }
```

```bash
mise install
flyloft --version
```

**macOS troubleshooting** — if a quarantine-aware host propagates
`com.apple.quarantine` into the mise install and Gatekeeper prompts, clear it
once:

```bash
xattr -d com.apple.quarantine "$(mise which flyloft)"
```

### Option 3: Download Pre-built Binary

Download the latest release from [GitHub Releases](https://github.com/ArcavenAE/flyloft/releases). Binaries are available for macOS (arm64) and Linux (amd64, arm64).

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
