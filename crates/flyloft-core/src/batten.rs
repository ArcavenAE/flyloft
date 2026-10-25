//! Battens — the atomic unit of indexed material.
//!
//! A batten is a chunk of source text plus its provenance, position, confidence,
//! annotations, and entity links. Battens hang from the grid; they are rigged,
//! spiked, dogged, or struck.

use serde::{Deserialize, Serialize};

use crate::catalog::{CatalogId, CatalogPointer};
use crate::provenance::{Confidence, Distance, Provenance};
use crate::{Annotation, EntityRef, LineSetId, Position};

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

    pub fn from_cataloged(line_set: &LineSetId, catalog: &CatalogId, external_id: &str) -> Self {
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

/// Why a batten could not be built or loaded.
///
/// The distance invariant (design item 4): a prop has no `projects`; a
/// backdrop or scenery batten has at least one, and its content is held.
/// Whether a projected id exists or is struck is not checked here.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum BattenError {
    #[error("a prop batten has no projects, but it lists some")]
    PropProjects,
    #[error("a {0:?} batten must project at least one prop, but its projects are empty")]
    NoProjects(Distance),
    #[error("a {0:?} batten is authored text, so its content must be held, not cataloged")]
    NotHeld(Distance),
}

/// The fields a new batten is built from. Pass it to [`Batten::new`], which
/// checks the distance invariant.
#[derive(Debug, Clone)]
pub struct NewBatten {
    pub id: BattenId,
    pub line_set: LineSetId,
    pub content: BattenContent,
    pub parent: Option<BattenId>,
    pub position: Position,
    pub provenance: Provenance,
    pub confidence: Confidence,
    pub spiked: bool,
    pub dogged: bool,
    pub annotations: Vec<Annotation>,
    pub entities: Vec<EntityRef>,
    pub promoted_to: Option<String>,
    pub rigged_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub distance: Distance,
    pub projects: Vec<BattenId>,
}

/// A batten: the atomic indexed unit. Held or cataloged — curation verbs
/// treat both uniformly.
///
/// `distance`, `projects` and `content` are private: the distance invariant
/// is checked when a batten is built ([`Batten::new`]) or loaded
/// (deserialization), and nothing afterward can break it. Repainting makes a
/// new batten.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawBatten")]
pub struct Batten {
    pub id: BattenId,
    pub line_set: LineSetId,

    /// The batten's content: either held locally or a catalog pointer.
    content: BattenContent,

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

    /// How far from the material this batten sits. Fixed at creation.
    distance: Distance,

    /// The props a backdrop or scenery batten projects. Empty for props.
    projects: Vec<BattenId>,
}

/// The on-disk shape. Deserialization lands here first, then `try_from`
/// checks the invariant, so the YAML has one shape and no way around it.
#[derive(Deserialize)]
struct RawBatten {
    id: BattenId,
    line_set: LineSetId,
    content: BattenContent,
    parent: Option<BattenId>,
    position: Position,
    provenance: Provenance,
    confidence: Confidence,
    spiked: bool,
    dogged: bool,
    annotations: Vec<Annotation>,
    entities: Vec<EntityRef>,
    promoted_to: Option<String>,
    rigged_at: chrono::DateTime<chrono::Utc>,
    last_modified: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    distance: Distance,
    #[serde(default)]
    projects: Vec<BattenId>,
}

impl TryFrom<RawBatten> for Batten {
    type Error = BattenError;

    fn try_from(r: RawBatten) -> Result<Self, BattenError> {
        Batten::new(NewBatten {
            id: r.id,
            line_set: r.line_set,
            content: r.content,
            parent: r.parent,
            position: r.position,
            provenance: r.provenance,
            confidence: r.confidence,
            spiked: r.spiked,
            dogged: r.dogged,
            annotations: r.annotations,
            entities: r.entities,
            promoted_to: r.promoted_to,
            rigged_at: r.rigged_at,
            last_modified: r.last_modified,
            distance: r.distance,
            projects: r.projects,
        })
    }
}

impl Batten {
    /// Build a batten, refusing one that breaks the distance invariant.
    pub fn new(n: NewBatten) -> Result<Self, BattenError> {
        if n.distance.projects_props() {
            if n.projects.is_empty() {
                return Err(BattenError::NoProjects(n.distance));
            }
            if !n.content.is_held() {
                return Err(BattenError::NotHeld(n.distance));
            }
        } else if !n.projects.is_empty() {
            return Err(BattenError::PropProjects);
        }
        Ok(Self {
            id: n.id,
            line_set: n.line_set,
            content: n.content,
            parent: n.parent,
            position: n.position,
            provenance: n.provenance,
            confidence: n.confidence,
            spiked: n.spiked,
            dogged: n.dogged,
            annotations: n.annotations,
            entities: n.entities,
            promoted_to: n.promoted_to,
            rigged_at: n.rigged_at,
            last_modified: n.last_modified,
            distance: n.distance,
            projects: n.projects,
        })
    }

    pub fn distance(&self) -> Distance {
        self.distance
    }

    pub fn projects(&self) -> &[BattenId] {
        &self.projects
    }

    pub fn content(&self) -> &BattenContent {
        &self.content
    }
}
