use crate::sim::globals::MAX_CURRENT_THOUGHTS;
use crate::sim::person::needs::NeedType;
use crate::sim::person::skills::SkillId;
use crate::sim::person::stats::StatType;
use crate::sim::sim_date::sim_date::SimDate;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Encode, Decode)]
pub struct EntityId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum ThoughtContext {
    None,
    Person(EntityId),
    Need(NeedType),
    Stat(StatType),
    Task(EntityId),
    Meeting(EntityId),
    Project(EntityId),
    Skill(SkillId),
    Event(String),
    Multi(Box<[ThoughtContext]>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Thought {
    pub sim_date: SimDate,
    pub context: ThoughtContext,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Thoughts {
    pub thoughts: VecDeque<Thought>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ArchivedThoughts {
    pub thoughts: Vec<Thought>,
}

impl Thoughts {
    pub fn new() -> Self {
        Self {
            thoughts: VecDeque::new(),
        }
    }

    pub fn add(&mut self, thought: Thought, archive: &mut ArchivedThoughts) {
        if self.thoughts.len() >= MAX_CURRENT_THOUGHTS {
            if let Some(old) = self.thoughts.pop_front() {
                archive.thoughts.push(old);
            }
        }
        self.thoughts.push_back(thought);
    }
}

impl ArchivedThoughts {
    pub fn new() -> Self {
        Self {
            thoughts: Vec::new(),
        }
    }

    pub fn add(&mut self, thought: Thought) {
        self.thoughts.push(thought);
    }
}
