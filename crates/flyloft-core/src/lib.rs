//! flyloft-core
//!
//! Core types and operations for the Flyloft retrieval substrate.
//!
//! The vocabulary is theatrical: battens hang on a grid, flown in by the
//! retrieval pipeline, struck when stale, spiked when authoritative. See
//! `docs/CONCEPTS.md` in the repository root for the full terminology.

pub mod batten;
pub mod catalog;
pub mod cue;
pub mod grid;
pub mod line_set;
pub mod provenance;

pub use batten::{Batten, BattenContent, BattenId};
pub use catalog::{Catalog, CatalogId, CatalogPointer, CatalogRef, CatalogRegistry};
pub use cue::{Cue, CueId};
pub use grid::Grid;
pub use line_set::{LineSet, LineSetId};
pub use provenance::{Confidence, Contributor, Provenance};

use serde::{Deserialize, Serialize};

/// A position of a batten within its parent line set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub ordinal: usize,
    pub path: Vec<String>, // hierarchical path for structured sources
    pub char_range: Option<(usize, usize)>,
}

/// An annotation attached to a batten.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub by: Contributor,
    pub at: chrono::DateTime<chrono::Utc>,
    pub kind: AnnotationKind,
    pub note: String,
    pub reviewed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnnotationKind {
    Correction,
    Clarification,
    CrossReference,
    Caution,
    FlypersonNote,
}

/// A reference to an entity in the graph overlay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRef {
    pub name: String,
    pub kind: String, // e.g. "technology", "person", "concept"
    pub confidence: f32,
}

/// A dispute against a batten.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispute {
    pub id: String,
    pub batten_id: BattenId,
    pub by: Contributor,
    pub at: chrono::DateTime<chrono::Utc>,
    pub reason: String,
    pub evidence: Option<String>,
    pub resolution: Option<DisputeResolution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisputeResolution {
    Struck,
    Annotated { annotation_id: String },
    Dismissed { reason: String },
}
