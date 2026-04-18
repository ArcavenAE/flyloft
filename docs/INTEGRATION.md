# Integration

Flyloft is designed to run in two modes without code divergence:

## 1. Embedded in Marvel

When a team is hosted inside Marvel, Flyloft is provisioned as part of the team's platform slice. Integration points:

- **Forestage / Apron** call Flyloft via its REST surface for human-driven retrieval (a sidebar search, a cited-sources panel, a grooming dashboard).
- **Spectacle** renders Flyloft views: the cue sheet as a heatmap, the entity graph as a navigable network, grooming queues as Kanban-style boards.
- **forestage** Supervisors are given Flyloft's MCP endpoint scoped to the relevant line sets for their Crew session.
- **KOS** instance is co-located and wired up via `flyloft.toml` so promotion is a single command.
- **Authentication** is delegated to Marvel's auth; Flyloft honors Marvel roles (team-member, team-lead, platform-admin) with mappings to Flyloft roles (observer, flyperson, admin).

Marvel-embedded Flyloft does not expose its CLI to end users — the flyperson workflows happen through Marvel's surfaces, which wrap the CLI operations.

### Provisioning

A new Marvel team gets a new Flyloft instance automatically:

- Git repository created under the team's namespace.
- Initial `flyloft.toml` generated with Marvel-specific defaults.
- MCP endpoint registered with the team's forestage instance.
- Spectacle dashboards wired up.
- KOS connection configured.

## 2. Standalone (BYOA — Bring Your Own Agents)

Run Flyloft as a private service against any agent runtime. This mode is for:

- Teams using Claude Code directly without Marvel.
- Teams running forestage but not hosting on Marvel.
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

1. Who issues auth tokens (Marvel vs Flyloft itself).
2. Where the UI lives (Forestage/Apron/Spectacle vs terminal).
3. Provisioning path (automatic in Marvel, manual via CLI in BYOA).

A team can migrate BYOA → Marvel-embedded (and vice versa) by moving the grid directory. All state travels with it.

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

## forestage integration

forestage integration is specifically tuned for the Director/Supervisor/Crew pattern:

- **Directors** get read-only access to the full cue sheet — they shape sessions based on what the team has been asking about.
- **Supervisors** get scoped retrieval access for their Crew's domain.
- **Crew agents** get minimal, query-bound MCP tools.
- Crew-side `flyloft_contribute` calls surface as shift-change items for the Supervisor to review before accepting to the flyperson queue.

This preserves the hierarchical authority model forestage already assumes.
