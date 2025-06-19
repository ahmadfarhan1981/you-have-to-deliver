use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct FeatureTag {
    label: String,
    path: Vec<String>,
    description: String,
    visibility: bool,
}