# Integration

Flyloft is designed to run in two modes without code divergence:

## 1. Managed by marvel

When marvel manages a team's agent workloads, flyloft is provisioned as
part of the team's workspace. Integration points:

- **forestage** agent sessions (scheduled by marvel) call flyloft via
  MCP for retrieval. Human operators can also hit flyloft's REST surface
  for sidebar search, cited-sources panels, and grooming dashboards.
- **kos** is co-located and wired up via `flyloft.toml` so `promote` is
  a single command.
- **director** (when built) routes supervisor/crew retrieval traffic
  through flyloft's MCP endpoint scoped per session.
- **Authentication** is delegated to marvel's auth model. Flyloft maps
  marvel identities to its own roles (observer, flyperson, admin).

Flyloft's CLI remains the flyperson's tool. Human-facing UI surfaces
(sidebar search, grooming dashboards) are separate from flyloft proper;
they consume its REST/MCP endpoints. A visualization layer over the cue
sheet, entity graph, or grooming queues is a downstream concern and not
a flyloft deliverable.

### Provisioning

A new marvel-managed team gets a new flyloft instance automatically:

- Git repository created under the team's namespace.
- Initial `flyloft.toml` generated with marvel-aware defaults.
- MCP endpoint registered so marvel-scheduled agent sessions can reach it.
- kos connection configured.

## 2. Standalone (BYOA — Bring Your Own Agents)

Run Flyloft as a private service against any agent runtime. This mode is for:

- Teams using Claude Code directly without marvel.
- Teams running forestage outside a marvel-managed workspace.
- Teams with custom agent stacks (LangChain, AutoGen, home-grown).
- Individual researchers building their own flyloft for personal work.

### Deployment

Single binary. Runs anywhere the grid directory is accessible.

```bash
# On a laptop
flyloft serve --mcp --port 7070

# On a VM / container
docker run -v /data/flyloft:/grid -p 7070:7070 arcavenae/flyloft serve --mcp

# Systemd unit
systemctl start flyloft
```

### Agent runtime connections

- **Claude Code / forestage**: register the MCP endpoint in the agent's MCP config. Tokens can be scoped per session.
- **Custom runtimes**: use the REST API (same operations as MCP, HTTP-shaped).
- **Other MCP clients**: any MCP-compliant client works.

### Authentication

Standalone mode uses Flyloft's built-in token system. Tokens are:
- Issued by the flyperson via `flyloft token issue --role agent --scope "secops/*"`
- Revocable via `flyloft token revoke <id>`
- Role-scoped (agent, flyperson, observer, admin)
- Line-set-scoped (optional glob over line set IDs)

## Shared implementation

Both modes run the same binary and use the same data model. Differences are purely:

1. Who issues auth tokens (marvel vs flyloft itself).
2. Where human-facing UI lives (a marvel-wired dashboard vs the CLI).
3. Provisioning path (automatic under marvel, manual via CLI in BYOA).

A team can migrate BYOA → marvel-managed (and vice versa) by moving the grid directory. All state travels with it.

## KOS integration

Independent of deployment mode, KOS integration is configured in `flyloft.toml`:

```toml
[kos]
enabled = true
repo = "git@github.com:arcaven/threedoors-kos.git"
branch = "main"
promotion_default_confidence = "bedrock"
```

With KOS enabled:
- `flyloft promote` writes new KOS nodes and updates cross-references.
- Spiked battens track their promotion status.
- KOS demotions (bedrock → frontier) can optionally re-surface content back into Flyloft.

## director integration (planned)

When director ships, its supervisor/crew coordination pattern can layer
on flyloft's surface:

- **Directors** get read-only access to the full cue sheet — they shape
  sessions based on what the team has been asking about.
- **Supervisors** get scoped retrieval access for their crew's domain.
- **Crew agents** get minimal, query-bound MCP tools.
- Crew-side `flyloft_contribute` calls surface as shift-change items for
  the supervisor to review before reaching the flyperson queue.

This preserves the hierarchical authority model director establishes
between its roles. Director is unbuilt today; this section describes the
intended integration, not the current state.
