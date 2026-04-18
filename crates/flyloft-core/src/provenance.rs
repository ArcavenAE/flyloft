//! Provenance — where a batten came from, and how much we trust it.
//!
//! The confidence model mirrors KOS: bedrock (authoritative), frontier
//! (provisional), graveyard (struck but preserved for audit).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    /// Corroborated, spiked, stable. Candidate for KOS promotion.
    Bedrock,
    /// Default for newly rigged material. Active in retrieval, provisional.
    Frontier,
    /// Struck. Preserved in the grid but not returned by retrieval.
    Graveyard,
}

impl Default for Confidence {
    fn default() -> Self {
        Confidence::Frontier
    }
}

/// Who contributed (rigged, annotated, disputed) something.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Contributor {
    Human { id: String, display: Option<String> },
    Agent { id: String, runtime: String },
    System { id: String },
}

/// The provenance record for a batten.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub rigged_at: chrono::DateTime<chrono::Utc>,
    pub rigged_by: Contributor,

    /// How this batten was extracted from the source.
    pub extraction: ExtractionMethod,

    /// Optional upstream identifier (e.g. a URL, a document id, a KOS node id
    /// if this was rigged from KOS).
    pub upstream_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "snake_case")]
pub enum ExtractionMethod {
    SemanticChunk { model: String, target_tokens: usize },
    FixedChunk { tokens: usize, overlap: usize },
    DocumentWhole,
    StructuredPath { path: String },
    Manual,
}
