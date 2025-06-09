use crate::sim::person::needs::Needs;
use crate::sim::person::personality_matrix::PersonalityMatrix;
use crate::sim::person::stats::Stats;

// --- Consideration Trait ---
// Defines the interface for all scoring logic.
// `Send + Sync` is needed if you pass these around between threads (common in ECS).
pub trait Consideration: Send + Sync {
    // Calculates a normalized score (0.0 to 1.0) for a specific aspect.
    // Higher score means more desirable / urgent for this consideration.
    fn score(&self, needs: &Needs,stats: Stats, personality: &PersonalityMatrix) -> f32;
}

// --- Concrete Consideration Implementations (Global Logic) ---

pub struct EnergyConsideration;
impl Consideration for EnergyConsideration {
    fn score(&self, needs: &Needs, stats: Stats, personality: &PersonalityMatrix) -> f32 {
            let score = 1.0 - (needs.energy.value() as f32 / 100.0); // 0.0 if full, 1.0 if empty
            score.powf(2.0) // Non-linear: becomes more urgent as energy drops (squared curve)
    }
}