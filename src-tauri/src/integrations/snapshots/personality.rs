use serde::{Deserialize, Serialize};
use crate::sim::person::personality_matrix::{PersonalityAxis, PersonalityMatrix};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PersonalitySnapshot {
    pub assertiveness: i8,
    pub structure_preference: i8,
    pub openness: i8,
    pub sociability: i8,
    pub influence: i8,
    pub assertiveness_description: String,
    pub structure_preference_description: String,
    pub openness_description: String,
    pub sociability_description: String,
    pub influence_description: String,
}

impl From<&PersonalityMatrix> for PersonalitySnapshot {
    fn from(matrix: &PersonalityMatrix) -> Self {
        PersonalitySnapshot {
            assertiveness: matrix.assertiveness,
            structure_preference: matrix.structure_preference,
            openness: matrix.openness,
            sociability: matrix.sociability,
            influence: matrix.influence,
            assertiveness_description: matrix.describe_axis(PersonalityAxis::Assertiveness),
            structure_preference_description: matrix
                .describe_axis(PersonalityAxis::StructurePreference),
            openness_description: matrix.describe_axis(PersonalityAxis::Openness),
            sociability_description: matrix.describe_axis(PersonalityAxis::Sociability),
            influence_description: matrix.describe_axis(PersonalityAxis::Influence),
        }
    }
}

impl PartialEq<&PersonalityMatrix> for PersonalitySnapshot {
    fn eq(&self, other: &&PersonalityMatrix) -> bool {
        self.assertiveness == other.assertiveness
            && self.structure_preference == other.structure_preference
            && self.openness == other.openness
            && self.sociability == other.sociability
            && self.influence == other.influence
    }
}