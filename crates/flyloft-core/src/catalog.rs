//! Catalogs — external sources exposed through adapters.
//!
//! A catalog is anything an adapter can wrap behind `search()` and `fetch()`:
//! GitHub, Confluence, another Flyloft, a REST API, a research database.
//! Once registered, catalogs are queryable through the same fly rail as
//! the local stacks, and returned battens can be spiked, struck, annotated,
//! and disputed identically.
//!
//! See `docs/CATALOGS.md` for the full concept doc.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Identifier for a registered catalog.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CatalogId(pub String);

/// Human-readable description of a catalog. Returned by `describe()`
/// and shown in `flyloft catalog list` / the grooming surface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogDescription {
    pub id: CatalogId,
    pub adapter: String, // e.g. "github", "confluence", "flyloft"
    pub display_name: String,
    pub notes: Option<String>,
    pub in_default_federation: bool,
    pub supports_fetch: bool, // false for search-only adapters
    pub rate_limit_hint: Option<String>,
}

/// Health state of a catalog adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CatalogHealth {
    Ok,
    Degraded { reason: String },
    Unavailable { reason: String },
}

/// A search result from a catalog. Lightweight pointer; full content is
/// resolved later via `fetch()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogRef {
    pub catalog: CatalogId,
    pub external_id: String,
    pub title: Option<String>,
    pub snippet: Option<String>,
    pub url: Option<String>,
    pub score: Option<f32>,
    /// Adapter-specific metadata (authors, date, issue number, etc.).
    pub metadata: serde_json::Value,
}

/// Full content resolved from a catalog. Returned by `fetch()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogContent {
    pub external_id: String,
    pub text: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub fetched_at: chrono::DateTime<chrono::Utc>,
    pub metadata: serde_json::Value,
}

/// The adapter trait. Each supported external source type (github, confluence,
/// flyloft, ...) implements this.
#[async_trait]
pub trait Catalog: Send + Sync {
    fn id(&self) -> &CatalogId;

    fn describe(&self) -> CatalogDescription;

    /// Search the catalog. Returns lightweight pointers.
    async fn search(&self, query: &str, limit: usize) -> anyhow::Result<Vec<CatalogRef>>;

    /// Fetch full content for an external id previously returned by `search`.
    /// Adapters that are search-only may return an Err here and set
    /// `supports_fetch: false` in their description.
    async fn fetch(&self, external_id: &str) -> anyhow::Result<CatalogContent>;

    /// Lightweight health check. Used by `flyloft doctor` and for graceful
    /// degradation in federated queries.
    async fn health(&self) -> CatalogHealth;
}

/// Cache policy for a cataloged batten.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum CachePolicy {
    /// Don't cache. Fetch every resolution.
    #[default]
    None,
    /// Cache with a time-to-live.
    Ttl { seconds: u64 },
    /// Cache indefinitely; invalidate manually.
    Pinned,
}

/// The pointer stored on a cataloged batten. Distinct from `CatalogRef` —
/// `CatalogRef` is a transient search result, `CatalogPointer` is the
/// persisted reference on a committed batten.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogPointer {
    pub catalog: CatalogId,
    pub external_id: String,
    pub url: Option<String>,
    /// A cached preview for display in retrieval results without a full fetch.
    pub snippet: Option<String>,
    pub cache_policy: CachePolicy,
    pub last_fetched: Option<chrono::DateTime<chrono::Utc>>,
    pub adapter_metadata: serde_json::Value,
}

/// Registry of catalogs available at runtime. Populated from config at startup.
pub struct CatalogRegistry {
    // Using Box<dyn Catalog> keyed by id. Real impl will wire this up.
    _private: (),
}

impl CatalogRegistry {
    pub fn empty() -> Self {
        Self { _private: () }
    }

    pub fn register(&mut self, _catalog: Box<dyn Catalog>) -> anyhow::Result<()> {
        todo!("phase 1.5: keyed insertion, duplicate check, config validation")
    }

    pub fn get(&self, _id: &CatalogId) -> Option<&dyn Catalog> {
        todo!("phase 1.5")
    }

    pub fn federation(&self) -> Vec<&dyn Catalog> {
        todo!("phase 1.5: catalogs with in_default_federation=true")
    }
}
