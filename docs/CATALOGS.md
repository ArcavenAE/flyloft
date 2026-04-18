# Catalogs

Flyloft's native grid is the team's local stacks — material the team has rigged, chunked, indexed, and committed to git. But a working library is rarely self-contained. A flyperson routes requests to JSTOR when an article lives there, files an ILL when another library has what yours doesn't, and resolves a citation to whichever copy actually exists. Flyloft does the same through **catalogs**.

A catalog is a configured external source exposed through an adapter. Once registered, a catalog is queryable through the same fly rail as the local stacks. The flyperson can spike, strike, annotate, and dispute catalog results exactly as they would held material. The distinction between *held* and *cataloged* is visible in provenance but transparent to retrieval.

## Terminology

| Term                   | Meaning                                                                           |
|------------------------|-----------------------------------------------------------------------------------|
| **Stacks**             | The local grid. Battens whose text lives in the git repository.                   |
| **Catalog**            | An external source with a registered adapter (JSTOR, Confluence, another Flyloft, a REST API). |
| **Held batten**        | Text is in the grid. Always available.                                            |
| **Cataloged batten**   | A pointer with provenance and optional snippet. Text fetched on demand.           |
| **Fetch**              | Resolve a cataloged batten's full content from its catalog.                       |
| **Loan**               | A special case of fetch: borrowing a batten from another Flyloft instance (inter-flyloft loan).|
| **Paging**             | Requesting full content be fetched up from a catalog. Borrowed from library operations. |

## What a catalog can be

A catalog is anything an adapter can wrap behind `search()` and `fetch()`. Expected catalog types at v1:

- **GitHub** — issues, PRs, code, markdown across configured repos.
- **Generic REST / Confluence / Notion** — team-internal wikis and knowledge bases.
- **Another Flyloft** — inter-flyloft loan. Search the remote fly rail; fetch remote battens.
- **Filesystem** — a directory watched but not chunked-and-committed; useful for transient or very large corpora.
- **KOS** — read-only view of the spec graph as a catalog (promoting *out* still uses the dedicated promote verb).

Expected v2+:

- **JSTOR, Semantic Scholar, arXiv** — research literature.
- **Vector-only RAG endpoints** — teams with existing retrieval services fronted by a Flyloft catalog adapter.
- **Databases with text fields** — SQL/NoSQL stores where specific columns are textually useful.

## Held vs cataloged

A batten is one of two shapes:

- **Held.** Text lives in `grid/line-sets/<id>/battens/<id>.yaml`. Committed to git. Always retrievable. This is the primary mode for material the team has curated.
- **Cataloged.** The batten YAML contains a `catalog` pointer (catalog id + external id), a snippet for retrieval-time display, and optional cache metadata. The full text is fetched on demand.

Both share identity, provenance, confidence state, annotations, entities, and curation flags. A flyperson spiking a cataloged JSTOR article doesn't move the article into the grid — the spike asserts authority, not ownership. If the team wants a permanent local copy, they can **adopt** a cataloged batten (a verb that copies its current content into the stacks as a held batten with a provenance trail back to the catalog).

## Retrieval semantics

`flyloft_fly` queries the union of the stacks and any catalogs configured as `in_default_federation = true`. Each source returns candidates with its own scoring; results are merged with reciprocal-rank-fusion before reranking. The drop presented to the caller doesn't distinguish sources in its structure — held and cataloged battens arrive with the same shape, plus a `catalog` field populated only on the latter.

An agent that cites a cataloged batten triggers the same cue-sheet logging as a held one. A flyperson weeding the corpus sees cataloged battens in the cue sheet alongside held ones, and can spike or strike either.

## Caching

Cataloged battens can be cached locally for performance and offline availability:

- **No cache** (default for most catalogs). Fetch on every resolution.
- **TTL cache.** Cache with an expiry. Good for mostly-stable external sources.
- **Pinned.** Cache indefinitely, invalidate manually. Good for citations that must not rot.

Cache state is stored in `.flyloft/cache/` (gitignored) keyed by catalog id + external id. The cache is a performance optimization; source of truth remains the catalog.

## Adapter contract

An adapter implements the `Catalog` trait (`crates/flyloft-core/src/catalog.rs`):

```rust
#[async_trait]
pub trait Catalog: Send + Sync {
    fn id(&self) -> &CatalogId;
    fn describe(&self) -> CatalogDescription;

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<CatalogRef>>;
    async fn fetch(&self, external_id: &str) -> Result<CatalogContent>;
    async fn health(&self) -> CatalogHealth;
}
```

- **`search`** returns lightweight pointers (id, title, snippet, url, score) — not full content. This is the first-round retrieval call.
- **`fetch`** resolves a pointer into full content. Called when a batten needs to be materialized for an agent or flyperson.
- **`health`** reports adapter status for the doctor command and for failing gracefully in federated queries.

Adapters live in their own crates (`crates/flyloft-catalog-<name>/`) so dependencies don't infect the core. The core ships the trait and a handful of reference adapters; third parties can ship more.

## Configuration

Catalogs are declared in `flyloft.toml`:

```toml
[[catalogs]]
id = "arcaven-github"
adapter = "github"
in_default_federation = true
cache = { mode = "ttl", seconds = 3600 }

  [catalogs.config]
  org = "ArcavenAE"
  repos = ["flyloft", "forestage"]
  token_env = "GITHUB_TOKEN"

[[catalogs]]
id = "team-confluence"
adapter = "confluence"
in_default_federation = true
cache = { mode = "ttl", seconds = 86400 }

  [catalogs.config]
  base_url = "https://company.atlassian.net/wiki"
  space_keys = ["MSS", "PLAT"]
  token_env = "CONFLUENCE_TOKEN"

[[catalogs]]
id = "peer-flyloft-secops"
adapter = "flyloft"
in_default_federation = false   # opt-in; query explicitly
cache = { mode = "none" }

  [catalogs.config]
  endpoint = "https://flyloft.secops.internal/mcp"
  token_env = "PEER_FLYLOFT_TOKEN"
```

## CLI

- `flyloft catalog list` — registered catalogs with health status.
- `flyloft catalog health` — ping each adapter and report.
- `flyloft catalog add --id <id> --adapter <type> …` — register.
- `flyloft catalog remove <id>` — unregister.
- `flyloft rig --catalog <id> --query "<q>"` — rig a slice of a catalog as a held line set (for when you want to bring the material into the stacks permanently).
- `flyloft adopt <batten-id>` — convert a cataloged batten into a held one.
- `flyloft fly "<q>" --catalogs only` — query catalogs without the local stacks.
- `flyloft fly "<q>" --catalogs none` — query only the local stacks.

## The flyperson and federation

Federation changes the flyperson's role in a specific way: they're no longer only curating the corpus they *own*, they're curating the corpus they *can reach*. Weeding becomes bimodal — retire held battens that are stale, demote catalogs whose quality has slipped. Spiking becomes an assertion: "this external reference is authoritative for us" is a meaningful act of curation even without moving bytes.

This is the honest shape of the emerging role. A librarian has never been *only* the keeper of the local collection; they have always been the person who knows how to reach the material a patron needs, wherever it lives. Catalogs make Flyloft honest about that.

## Security notes

- Adapter tokens live in environment variables, never committed.
- Cataloged battens carry the catalog id in provenance so a downstream reader can assess trust.
- Catalog responses are treated as untrusted input: entity extraction is advisory, injection-defense applies to any text fetched from a catalog before it's surfaced to an agent, and adapters should scope credentials to the minimum needed.
- `in_default_federation = false` is the right setting for catalogs that shouldn't be queried implicitly — external databases with per-query costs, peer Flylofts that need explicit loan semantics, research sources with licensing constraints.

## What this is not

- **Not a proxy.** Flyloft doesn't forward arbitrary queries to arbitrary services. Adapters are specific, configured, and have bounded surfaces.
- **Not a universal RAG router.** Flyloft is the librarian's workstation, not a traffic layer. Adapters exist to bring known, trusted sources into the flyperson's curation surface, not to front-end every possible information source in the world.
- **Not a replacement for KOS.** KOS remains the authoritative distilled graph. Catalogs broaden Flyloft's *discovery* reach; KOS narrows *decision* to what the team has chosen to commit to.
