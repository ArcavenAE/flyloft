//! Cues — the retrieval event log.
//!
//! Every drop (retrieval event) writes a Cue. The cue sheet is the
//! flyperson's primary signal source: what's being asked, what's returning
//! nothing, what's retrieved-but-never-cited (cold), what's frequently cited
//! (hot and a candidate for spiking).

use crate::BattenId;
use crate::provenance::Contributor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CueId(pub String);

impl CueId {
    pub fn new() -> Self {
        Self(format!("cue_{}", uuid::Uuid::now_v7().simple()))
    }
}

impl Default for CueId {
    fn default() -> Self {
        Self::new()
    }
}

/// A single retrieval event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cue {
    pub id: CueId,
    pub query: String,
    pub rewritten: Option<String>,
    pub rewrite_strategy: Option<String>,
    pub requester: Contributor,

    /// Battens flown in with their rerank scores.
    pub flown: Vec<FlownBatten>,

    /// Battens the caller later reported as actually used.
    pub cited: Vec<BattenId>,

    pub at: chrono::DateTime<chrono::Utc>,

    /// Time to retrieve in milliseconds.
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlownBatten {
    pub batten_id: BattenId,
    pub dense_score: Option<f32>,
    pub sparse_score: Option<f32>,
    pub rerank_score: Option<f32>,
    pub final_rank: usize,
}
