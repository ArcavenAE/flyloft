//! Battens — the atomic unit of indexed material.
//!
//! A batten is a chunk of source text plus its provenance, position, confidence,
//! annotations, and entity links. Battens hang from the grid; they are rigged,
//! spiked, dogged, or struck.

use crate::catalog::{CatalogId, CatalogPointer};
use crate::{Annotation, EntityRef, LineSetId, Position};
use crate::provenance::{Confidence, Provenance};
use serde::{Deserialize, Serialize};

/// Content-addressable identifier for a batten.
///
/// For held battens: blake3 over the canonical text plus line-set and position.
/// For cataloged battens: blake3 over the catalog id, external id, and line-set.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BattenId(pub String);

impl BattenId {
    pub fn from_held(line_set: &LineSetId, position: &Position, text: &str) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"held:");
        hasher.update(line_set.0.as_bytes());
        hasher.update(&position.ordinal.to_le_bytes());
        hasher.update(text.as_bytes());
        let hash = hasher.finalize();
        Self(format!("bat_{}", &hash.to_hex()[..24]))
    }

    pub fn from_cataloged(
        line_set: &LineSetId,
        catalog: &CatalogId,
        external_id: &str,
    ) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"cataloged:");
        hasher.update(line_set.0.as_bytes());
        hasher.update(catalog.0.as_bytes());
        hasher.update(external_id.as_bytes());
        let hash = hasher.finalize();
        Self(format!("bat_{}", &hash.to_hex()[..24]))
    }
}

/// The content of a batten: either held locally in the stacks, or a pointer
/// into a catalog resolved on demand.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BattenContent {
    /// Text is persisted in the batten YAML. Always available.
    Held { text: String },
    /// A pointer into a catalog. Full text fetched on demand; snippet cached
    /// for retrieval-time display.
    Cataloged(CatalogPointer),
}

impl BattenContent {
    /// Returns the best text available without a catalog fetch:
    /// the held text for held battens, the snippet (if any) for cataloged.
    pub fn preview(&self) -> Option<&str> {
        match self {
            BattenContent::Held { text } => Some(text),
            BattenContent::Cataloged(ptr) => ptr.snippet.as_deref(),
        }
    }

    pub fn is_held(&self) -> bool {
        matches!(self, BattenContent::Held { .. })
    }

    pub fn is_cataloged(&self) -> bool {
        matches!(self, BattenContent::Cataloged(_))
    }
}

/// A batten: the atomic indexed unit. Held or cataloged — curation verbs
/// treat both uniformly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batten {
    pub id: BattenId,
    pub line_set: LineSetId,

    /// The batten's content: either held locally or a catalog pointer.
    pub content: BattenContent,

    /// For small-to-big retrieval. A batten may point to a larger parent batten
    /// (e.g. a sentence pointing to its paragraph, a paragraph to its section).
    pub parent: Option<BattenId>,

    pub position: Position,
    pub provenance: Provenance,
    pub confidence: Confidence,

    /// Spiked battens are weighted higher in retrieval and are candidates for
    /// promotion into KOS.
    pub spiked: bool,

    /// Dogged battens are frozen against modification.
    pub dogged: bool,

    pub annotations: Vec<Annotation>,
    pub entities: Vec<EntityRef>,

    /// If this batten has been promoted into KOS, the target node id.
    pub promoted_to: Option<String>,

    pub rigged_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}
