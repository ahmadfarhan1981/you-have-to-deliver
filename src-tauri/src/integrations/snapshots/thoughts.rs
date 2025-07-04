use crate::sim::person::thoughts::Thought;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtsSnapshot {
    pub person_id: u32,
    pub thoughts: Vec<Thought>,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct DirtyThought;
