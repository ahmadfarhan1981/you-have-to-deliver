use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::hash::Hash;
use crate::sim::locations::locations::LocationId;
use crate::sim::person::stats::StatType;
// Assuming this macro is used for StatType


// --- NEW: StatSkewDetail Struct ---
// Represents a specific stat and its relative weight in a skew.
// Note: f32 does not implement Eq, so we can only derive PartialEq.
#[derive(Debug, Clone, PartialEq)]
pub struct StatSkewDetail {
    pub stat_type: StatType,
    pub weight: f32, // Relative weight for skewing this stat
}

// --- UPDATED: TalentProfile Enum ---
// Now uses `StatSkewDetail` for boosted and suppressed stats.
#[derive(Debug, Clone, PartialEq)]
pub enum TalentProfile {
    /// Represents a talent pool that skews certain stats by a given relative weight.
    /// The first Vec is for boosted stats, the second Vec is for suppressed stats.
    StatsSkew {
        boosted: Vec<StatSkewDetail>,
        suppressed: Vec<StatSkewDetail>,
    },
    // Future variants can still be added here.
}

// --- NEW: LocationTalentProfile Struct ---
// This struct maps Location IDs to their corresponding Talent Profiles.
#[derive(Debug, Clone)]
pub struct LocationTalentProfile {
    profiles: HashMap<LocationId, TalentProfile>, // LocationId comes from your other definitions
}

impl LocationTalentProfile {
    /// Creates a new, empty `LocationTalentProfile`.
    pub fn new() -> Self {
        LocationTalentProfile {
            profiles: HashMap::new(),
        }
    }

    /// Adds a talent profile for a specific location.
    pub fn add_profile(&mut self, location_id: LocationId, profile: TalentProfile) {
        self.profiles.insert(location_id, profile);
    }

    /// Retrieves a reference to the talent profile for a given location.
    pub fn get_profile(&self, location_id: &LocationId) -> Option<&TalentProfile> {
        self.profiles.get(location_id)
    }

    /// Initializes the `LocationTalentProfile` with default data for your locations.
    /// All weights are set to 1.0 initially, allowing for easy relative tweaking later.
    pub fn initialize_default_profiles(&mut self) {
        self.add_profile(
            "precision_spire".into(),
            TalentProfile::StatsSkew {
                boosted: vec![
                    StatSkewDetail { stat_type: StatType::Precision, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Focus, weight: 1.0 },
                ],
                suppressed: vec![
                    StatSkewDetail { stat_type: StatType::Creativity, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Empathy, weight: 1.0 },
                ],
            },
        );
        self.add_profile(
            "discipline_stack".into(),
            TalentProfile::StatsSkew {
                boosted: vec![
                    StatSkewDetail { stat_type: StatType::Discipline, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Precision, weight: 1.0 },
                ],
                suppressed: vec![
                    StatSkewDetail { stat_type: StatType::Creativity, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Adaptability, weight: 1.0 },
                ],
            },
        );
        self.add_profile(
            "architecture_foundry".into(),
            TalentProfile::StatsSkew {
                boosted: vec![
                    StatSkewDetail { stat_type: StatType::Systems, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Judgement, weight: 1.0 },
                ],
                suppressed: vec![
                    StatSkewDetail { stat_type: StatType::Communication, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Empathy, weight: 1.0 },
                ],
            },
        );
        self.add_profile(
            "endurance_basin".into(),
            TalentProfile::StatsSkew {
                boosted: vec![
                    StatSkewDetail { stat_type: StatType::Resilience, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Focus, weight: 1.0 },
                ],
                suppressed: vec![
                    StatSkewDetail { stat_type: StatType::Creativity, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Adaptability, weight: 1.0 },
                ],
            },
        );
        self.add_profile(
            "collaboration_core".into(),
            TalentProfile::StatsSkew {
                boosted: vec![
                    StatSkewDetail { stat_type: StatType::Empathy, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Communication, weight: 1.0 },
                ],
                suppressed: vec![
                    StatSkewDetail { stat_type: StatType::Discipline, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Focus, weight: 1.0 },
                ],
            },
        );
        self.add_profile(
            "idea_hub".into(),
            TalentProfile::StatsSkew {
                boosted: vec![
                    StatSkewDetail { stat_type: StatType::Creativity, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Adaptability, weight: 1.0 },
                ],
                suppressed: vec![
                    StatSkewDetail { stat_type: StatType::Precision, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Judgement, weight: 1.0 },
                ],
            },
        );
        self.add_profile(
            "versatile_plaza".into(),
            TalentProfile::StatsSkew {
                boosted: vec![], // No specific boosts
                suppressed: vec![], // No specific suppressions
            },
        );
        self.add_profile(
            "tech_valley".into(),
            TalentProfile::StatsSkew {
                boosted: vec![
                    StatSkewDetail { stat_type: StatType::Judgement, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Creativity, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Systems, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Precision, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Focus, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Discipline, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Empathy, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Communication, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Resilience, weight: 1.0 },
                    StatSkewDetail { stat_type: StatType::Adaptability, weight: 1.0 },
                ], // All stats listed with a weight of 1.0
                suppressed: vec![],
            },
        );
    }
}

// --- Global Static Declaration ---

/// Globally available LocationTalentProfile, containing the talent skew for each location.
/// Initialized once when first accessed.
pub static LOCATION_TALENT_PROFILES: Lazy<LocationTalentProfile> = Lazy::new(|| {
    let mut profiles = LocationTalentProfile::new();
    profiles.initialize_default_profiles();
    profiles
});