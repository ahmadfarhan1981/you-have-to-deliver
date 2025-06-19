use std::collections::VecDeque;

const STRESS_HISTORY_DAYS: usize = 28;
const MAX_DAILY_STRESS: f32 = 100.0;




#[derive(Debug, Default)]
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
