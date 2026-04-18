//! Grid — the top-level storage handle.
//!
//! The grid is the corpus in aggregate. This module will own the on-disk
//! layout, git integration, and the verb operations (rig, fly, strike,
//! spike, dog). Stub for now.

use std::path::PathBuf;

use crate::{Batten, BattenId, LineSet, LineSetId};

pub struct Grid {
    pub root: PathBuf,
}

impl Grid {
    pub fn open(root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let root = root.into();
        // TODO: validate layout, load config
        Ok(Self { root })
    }

    pub fn init(_root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        todo!("phase 0: create layout, write flyloft.toml, optionally git init")
    }

    // -- verbs --

    pub fn rig(&self, _source: crate::line_set::Source) -> anyhow::Result<LineSetId> {
        todo!("phase 0/1: parse, chunk, embed, index, persist")
    }

    pub fn fly(&self, _query: &str, _k: usize) -> anyhow::Result<Vec<Batten>> {
        todo!("phase 1: hybrid retrieval + rerank")
    }

    pub fn spike(&self, _batten: &BattenId) -> anyhow::Result<()> {
        todo!("phase 2")
    }

    pub fn strike(&self, _batten: &BattenId) -> anyhow::Result<()> {
        todo!("phase 2")
    }

    pub fn dog(&self, _line_set: &LineSetId) -> anyhow::Result<()> {
        todo!("phase 2")
    }

    pub fn promote_to_kos(&self, _batten: &BattenId) -> anyhow::Result<()> {
        todo!("phase 5")
    }
}

// Avoid the unused-import warning on the skeleton.
#[allow(dead_code)]
fn _uses_linesets(_: &LineSet) {}
