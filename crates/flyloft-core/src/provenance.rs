//! Provenance — where a batten came from, and how much we trust it.
//!
//! The confidence model mirrors KOS: bedrock (authoritative), frontier
//! (provisional), graveyard (struck but preserved for audit).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    /// Corroborated, spiked, stable. Candidate for KOS promotion.
    Bedrock,
    /// Default for newly rigged material. Active in retrieval, provisional.
    #[default]
    Frontier,
    /// Struck. Preserved in the grid but not returned by retrieval.
    Graveyard,
}

/// How far from the material a batten sits, stage-rig style.
///
/// Fixed when the batten is built: nothing changes a batten's distance, so a
/// prop never becomes a backdrop (design: `docs/design/distance-axis.md`).
/// Costumes, the fourth stage-rig distance, are agent identity, not battens.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Distance {
    /// Painted overview of a body of props. Authored text, never derived.
    Backdrop,
    /// Mid-distance material that points at the props it summarizes.
    Scenery,
    /// Source material held verbatim, or a pointer into a catalog. What
    /// `rig` ingests.
    #[default]
    Prop,
}

impl Distance {
    /// True for the distances that project props (backdrop and scenery).
    pub fn projects_props(self) -> bool {
        matches!(self, Distance::Backdrop | Distance::Scenery)
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
