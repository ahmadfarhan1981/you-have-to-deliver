use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::sim::project::FeatureTag::FeatureTag;

#[derive(Debug, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct Requirement {
    pub id: RequirementId,
    pub description: String,
    pub feature_tags: Vec<FeatureTag>,
    // Future: tags, hidden_expectations
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode,Decode , Serialize, Deserialize)]
pub struct RequirementId(pub u32);