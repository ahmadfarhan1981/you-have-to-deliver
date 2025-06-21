use std::collections::VecDeque;
use std::sync::Arc;
use bincode::{Decode, Encode};
use legion::system;
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use crate::sim::resources::global::TickCounter;

const STRESS_HISTORY_DAYS: usize = 28;
const MAX_DAILY_STRESS: f32 = 150.0;

const BASELINE_TOLERANCE: f32 = 65.0;
const SURGE_TOLERANCE: f32 = 90.0;
const RECOVERY_RATE:f32 = 40.0;

#[derive(Debug, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct StressLevel {
    // Tick-level load
    pub current: f32,

    // Stress accumulated during the current day
    pub daily_accumulator: f32,

    // === Dual Histories ===

    // Raw stress input received per day
    pub raw_input_history: VecDeque<f32>,
    pub raw_input_total: f32,

    // Felt stress recorded at the end of the day (after decay)
    pub felt_stress_history: VecDeque<f32>,
    pub felt_stress_total: f32,

    // Optional config values
    pub baseline_tolerance: f32,
    pub surge_tolerance: f32,

    // Daily flat recovery
    pub recovery_rate: f32,
}
impl Default for StressLevel {
    fn default() -> Self {
        Self{
            current: 0.0,
            daily_accumulator: 0.0,
            raw_input_history: VecDeque::new(),
            raw_input_total: 0.0,
            felt_stress_history: VecDeque::new(),
            felt_stress_total: 0.0,
            baseline_tolerance: BASELINE_TOLERANCE,// TODO take stats into account when generating
            surge_tolerance: SURGE_TOLERANCE,// TODO take stats into account when generating
            recovery_rate: RECOVERY_RATE,// TODO take stats into account when generating
        }
    }
}


impl StressLevel {
    /// Add stress for current tick
    pub fn apply(&mut self, amount: f32) {
        self.current += amount;
        self.daily_accumulator += amount;
    }

    /// Decay stress flatly based on recovery rate
    pub fn decay_tick(&mut self) {
        self.current = (self.current - self.recovery_rate).max(0.0);
    }

    /// End-of-day logic — capture both histories
    pub fn finalize_day(&mut self) {
        let clamped_input = self.daily_accumulator.clamp(0.0, MAX_DAILY_STRESS);
        
        self.decay_tick();//recover before taking the felt value
        let felt_today = self.current.clamp(0.0, MAX_DAILY_STRESS);

        // --- Raw input history ---
        self.raw_input_history.push_back(clamped_input);
        self.raw_input_total += clamped_input;
        if self.raw_input_history.len() > STRESS_HISTORY_DAYS {
            if let Some(removed) = self.raw_input_history.pop_front() {
                self.raw_input_total -= removed;
            }
        }

        // --- Felt stress history ---
        self.felt_stress_history.push_back(felt_today);
        self.felt_stress_total += felt_today;
        if self.felt_stress_history.len() > STRESS_HISTORY_DAYS {
            if let Some(removed) = self.felt_stress_history.pop_front() {
                self.felt_stress_total -= removed;
            }
        }

        // Reset per-day values
        self.daily_accumulator = 0.0;
    }

    // === Accessors ===

    pub fn average_raw_input(&self) -> f32 {
        if self.raw_input_history.is_empty() {
            0.0
        } else {
            self.raw_input_total / self.raw_input_history.len() as f32
        }
    }

    pub fn average_felt_stress(&self) -> f32 {
        if self.felt_stress_history.is_empty() {
            0.0
        } else {
            self.felt_stress_total / self.felt_stress_history.len() as f32
        }
    }

    pub fn max_recent_felt(&self, days: usize) -> f32 {
        self.felt_stress_history
            .iter()
            .rev()
            .take(days)
            .copied()
            .fold(0.0, f32::max)
    }

    pub fn max_recent_input(&self, days: usize) -> f32 {
        self.raw_input_history
            .iter()
            .rev()
            .take(days)
            .copied()
            .fold(0.0, f32::max)
    }
}

#[system(for_each)]
pub fn update_stress(
    stress_level: &mut StressLevel
){
    stress_level.apply( rng().random_range(0.001..0.8) )
}

#[system(for_each)]
pub fn daily_stress_reset(
    #[resource] tick_counter: &Arc<TickCounter>,
    stress_level: &mut StressLevel
){
    if tick_counter.current_date().quarter_tick == 1 {
        stress_level.finalize_day();
    }
    
}