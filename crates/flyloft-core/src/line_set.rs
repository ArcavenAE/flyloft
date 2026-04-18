//! Line sets — versioned groups of battens from a single source.
//!
//! A line set corresponds to one source (a document, a URL, a transcript,
//! an API export). It owns its battens; striking a line set retires all
//! its battens.

use crate::BattenId;
use crate::catalog::CatalogId;
use crate::provenance::Contributor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineSetId(pub String);

impl LineSetId {
    pub fn new() -> Self {
        Self(format!("ls_{}", uuid::Uuid::now_v7().simple()))
    }
}

impl Default for LineSetId {
    fn default() -> Self {
        Self::new()
    }
}

/// A line set: a versioned group of battens from one source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSet {
    pub id: LineSetId,
    pub source: Source,
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub rigged_at: chrono::DateTime<chrono::Utc>,
    pub rigged_by: Contributor,
    pub struck: Option<chrono::DateTime<chrono::Utc>>,
    pub dogged: bool,
    pub batten_ids: Vec<BattenId>,
}

/// The origin of a line set.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Source {
    File { path: String, sha256: String },
    Url { url: String, fetched_at: chrono::DateTime<chrono::Utc> },
    Transcript { session: String, speakers: Vec<String> },
    Api { endpoint: String, params: serde_json::Value },
    Text { origin: String },
    /// A slice of an external catalog materialized as a line set.
    /// Battens in this line set may be held (rigged into the stacks) or
    /// cataloged (pointers resolved on demand), depending on how the slice
    /// was rigged.
    Catalog {
        catalog: CatalogId,
        query: Option<String>,
        collection_id: Option<String>,
    },
}
