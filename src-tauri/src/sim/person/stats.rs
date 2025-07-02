use bincode::{Decode, Encode};
use rand::prelude::SliceRandom;
use rand::Rng;
use crate::utils::discreet_float::DiscreteFloat33;
// Removed unused 'rng' import, will use thread_rng
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, Serialize, Deserialize, Encode, Decode)]
pub enum StatType {
    Judgement,
    Creativity,
    Systems,
    Precision,
    Focus,
    Discipline,
    Empathy,
    Communication,
    Resilience,
    Adaptability,
}
impl FromStr for StatType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Judgement" => Ok(StatType::Judgement),
            "Creativity" => Ok(StatType::Creativity),
            "Systems" => Ok(StatType::Systems),
            "Precision" => Ok(StatType::Precision),
            "Focus" => Ok(StatType::Focus),
            "Discipline" => Ok(StatType::Discipline),
            "Empathy" => Ok(StatType::Empathy),
            "Communication" => Ok(StatType::Communication),
            "Resilience" => Ok(StatType::Resilience),
            "Adaptability" => Ok(StatType::Adaptability),
            _ => Err(()),
        }
    }
}
impl StatType {
    pub fn from_str_lossy(s: &str) -> Option<Self> {
        s.parse().ok()
    }

    pub fn get_group(self) -> StatGroup {
        match self {
            StatType::Judgement => StatGroup::Cognition,
            StatType::Creativity => StatGroup::Cognition,
            StatType::Systems => StatGroup::Perception,
            StatType::Precision => StatGroup::Perception,
            StatType::Focus => StatGroup::Drive,
            StatType::Discipline => StatGroup::Drive,
            StatType::Empathy => StatGroup::Social,
            StatType::Communication => StatGroup::Social,
            StatType::Resilience => StatGroup::Defense,
            StatType::Adaptability => StatGroup::Defense,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StatsConfig {
    // Cognition
    pub judgement: u16,
    pub creativity: u16,

    // Perception
    pub systems: u16,
    pub precision: u16,

    // Drive
    pub focus: u16,
    pub discipline: u16,

    // Social
    pub empathy: u16,
    pub communication: u16,

    // Defense
    pub resilience: u16,
    pub adaptability: u16,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Stats {
    // Cognition
    judgement: DiscreteFloat33,
    creativity: DiscreteFloat33,

    // Perception
    systems: DiscreteFloat33,
    precision: DiscreteFloat33,

    // Drive
    focus: DiscreteFloat33,
    discipline: DiscreteFloat33,

    // Social
    empathy: DiscreteFloat33,
    communication: DiscreteFloat33,

    // Defense
    resilience: DiscreteFloat33,
    adaptability: DiscreteFloat33,
}
impl Stats {
    /// Increases random stats in 1–2 point increments until total stat points
    /// reach or exceed the given target.
    ///
    /// Each cycle shuffles the stat order to ensure even distribution, but the
    /// random increments give variability. Overshoot is allowed.
    ///
    /// # Arguments
    /// * `target` – Desired minimum total stat points.
    ///
    /// # Example
    /// ```
    /// stats.normalize_to(60);
    /// ```
    pub fn normalize_to(&mut self, target: u16) {
        use StatType::*;

        let mut rng = rand::rng();
        let mut stat_pool = vec![
            Judgement,
            Creativity,
            Systems,
            Precision,
            Focus,
            Discipline,
            Empathy,
            Communication,
            Resilience,
            Adaptability,
        ];

        while self.total() < target {
            stat_pool.shuffle(&mut rng);
            for stat in &stat_pool {
                let increment = if rng.random_bool(0.5) { 1 } else { 2 };
                let current = self.get_stat(*stat);
                self.set_stat(*stat, current.saturating_add(increment));
                if self.total() >= target {
                    break;
                }
            }
        }
    }

    /// Sets the given stat to a specific value.
    ///
    /// This creates a new `DiscreteFloat33` for the stat.
    /// Panics if `value` is outside the 0-999 range supported by `DiscreteFloat33`.
    /// The value is clamped to a minimum of 0.
    ///
    /// # Arguments
    /// * `stat` – The stat to set.
    /// * `value` – The target value (0–100 scale).
    ///
    /// # Example
    /// ```
    /// stats.set_stat(StatType::Creativity, 75);
    /// ```
    pub fn set_stat(&mut self, stat: StatType, value: u16) {
        // Clamping to 0 is implicitly handled by u16.
        // DiscreteFloat33::new will handle negative floats, but u16 is always non-negative.
        // The main concern is value > 999, which DiscreteFloat33::new will reject.
        let new_df33 = DiscreteFloat33::new(value as f32)
            .expect("Failed to set stat: value likely out of DiscreteFloat33 range (0-999)");

        match stat {
            StatType::Judgement => self.judgement = new_df33,
            StatType::Creativity => self.creativity = new_df33,
            StatType::Systems => self.systems = new_df33,
            StatType::Precision => self.precision = new_df33,
            StatType::Focus => self.focus = new_df33,
            StatType::Discipline => self.discipline = new_df33,
            StatType::Empathy => self.empathy = new_df33,
            StatType::Communication => self.communication = new_df33,
            StatType::Resilience => self.resilience = new_df33,
            StatType::Adaptability => self.adaptability = new_df33,
        }
    }

    /// Adjusts a single stat by the given floating-point amount.
    ///
    /// Internally calls `adjust_many` with a single-element slice.
    pub fn adjust(&mut self, stat: StatType, amount: f32) {
        self.adjust_many(&[(stat, amount)]);
    }

    /// Adjusts multiple stats by the specified amounts.
    ///
    /// Each `StatType`'s `DiscreteFloat33` will be adjusted using its `add_f32` method.
    ///
    /// # Arguments
    /// * `adjustments` – A slice of `(StatType, f32)` tuples.
    ///
    /// # Example
    /// ```
    /// stats.adjust_many(&[
    ///     (StatType::Judgement, 1.0),
    ///     (StatType::Focus, -0.5),
    ///     (StatType::Creativity, 2.25),
    /// ]);
    /// ```
    pub fn adjust_many(&mut self, adjustments: &[(StatType, f32)]) {
        for &(stat, amount) in adjustments {
            match stat {
                StatType::Judgement => self.judgement.add_f32(amount),
                StatType::Creativity => self.creativity.add_f32(amount),
                StatType::Systems => self.systems.add_f32(amount),
                StatType::Precision => self.precision.add_f32(amount),
                StatType::Focus => self.focus.add_f32(amount),
                StatType::Discipline => self.discipline.add_f32(amount),
                StatType::Empathy => self.empathy.add_f32(amount),
                StatType::Communication => self.communication.add_f32(amount),
                StatType::Resilience => self.resilience.add_f32(amount),
                StatType::Adaptability => self.adaptability.add_f32(amount),
            }
        }
        // No sync_from_raw() needed, DiscreteFloat33 handles its state.
    }

    /// Returns the `StatType` with the lowest visible stat value.
    ///
    /// If multiple stats are tied for the lowest, the one that appears first in enum order is returned.
    pub fn lowest_stat(&self) -> StatType {
        use StatType::*;

        let all_stats = [
            Judgement,
            Creativity,
            Systems,
            Precision,
            Focus,
            Discipline,
            Empathy,
            Communication,
            Resilience,
            Adaptability,
        ];

        *all_stats
            .iter()
            .min_by_key(|stat| self.get_stat(**stat))
            .unwrap()
    }

    /// Returns all `StatType`s where the stat value satisfies the given comparison.
    ///
    /// # Arguments
    /// * `threshold` – The value to compare each stat against.
    /// * `cmp` – A comparison function that takes `(stat_value, threshold) -> bool`.
    ///
    /// # Example
    /// ```
    /// let high_stats = stats.stat_filter(70, |v, t| v > t);
    /// let low_or_equal = stats.stat_filter(50, |v, t| v <= t);
    /// ```
    pub fn stat_filter<F>(&self, threshold: u16, cmp: F) -> Vec<StatType>
    where
        F: Fn(u16, u16) -> bool,
    {
        use StatType::*;

        let all_stats = [
            Judgement,
            Creativity,
            Systems,
            Precision,
            Focus,
            Discipline,
            Empathy,
            Communication,
            Resilience,
            Adaptability,
        ];

        all_stats
            .iter()
            .cloned()
            .filter(|stat| cmp(self.get_stat(*stat), threshold))
            .collect()
    }

    pub fn total(&self) -> u16 {
        (self.judgement.value()
            + self.creativity.value()
            + self.systems.value()
            + self.precision.value()
            + self.focus.value()
            + self.discipline.value()
            + self.empathy.value()
            + self.communication.value()
            + self.resilience.value()
            + self.adaptability.value()) as u16
    }
}
impl From<StatsConfig> for Stats {
    fn from(config: StatsConfig) -> Self {
        // This will panic if any config value > 999 due to DiscreteFloat33's current range.
        Self {
            judgement: DiscreteFloat33::new(config.judgement as f32).expect("Config judgement out of range"),
            creativity: DiscreteFloat33::new(config.creativity as f32).expect("Config creativity out of range"),
            systems: DiscreteFloat33::new(config.systems as f32).expect("Config systems out of range"),
            precision: DiscreteFloat33::new(config.precision as f32).expect("Config precision out of range"),
            focus: DiscreteFloat33::new(config.focus as f32).expect("Config focus out of range"),
            discipline: DiscreteFloat33::new(config.discipline as f32).expect("Config discipline out of range"),
            empathy: DiscreteFloat33::new(config.empathy as f32).expect("Config empathy out of range"),
            communication: DiscreteFloat33::new(config.communication as f32).expect("Config communication out of range"),
            resilience: DiscreteFloat33::new(config.resilience as f32).expect("Config resilience out of range"),
            adaptability: DiscreteFloat33::new(config.adaptability as f32).expect("Config adaptability out of range"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum StatGroup {
    Cognition,
    Perception,
    Drive,
    Social,
    Defense,
}
impl StatGroup {
    pub fn members(&self) -> Vec<StatType> {
        use StatGroup::*;
        use StatType::*;

        match self {
            Cognition => vec![Judgement, Creativity],
            Perception => vec![Systems, Precision],
            Drive => vec![Focus, Discipline],
            Social => vec![Empathy, Communication],
            Defense => vec![Resilience, Adaptability],
        }
    }
}

impl Stats {
    pub fn get_stat(&self, stat: StatType) -> u16 {
        // DiscreteFloat33::value() returns u32, cast to u16. Safe if value <= 999.
        match stat {
            StatType::Judgement => self.judgement.value() as u16,
            StatType::Creativity => self.creativity.value() as u16,
            StatType::Systems => self.systems.value() as u16,
            StatType::Precision => self.precision.value() as u16,
            StatType::Focus => self.focus.value() as u16,
            StatType::Discipline => self.discipline.value() as u16,
            StatType::Empathy => self.empathy.value() as u16,
            StatType::Communication => self.communication.value() as u16,
            StatType::Resilience => self.resilience.value() as u16,
            StatType::Adaptability => self.adaptability.value() as u16,
        }
    }

    pub fn average_cognition(&self) -> f32 {
        (self.judgement.value() as f32 + self.creativity.value() as f32) / 2.0
    }

    pub fn average_perception(&self) -> f32 {
        (self.systems.value() as f32 + self.precision.value() as f32) / 2.0
    }

    pub fn average_drive(&self) -> f32 {
        (self.focus.value() as f32 + self.discipline.value() as f32) / 2.0
    }

    pub fn average_social(&self) -> f32 {
        (self.empathy.value() as f32 + self.communication.value() as f32) / 2.0
    }

    pub fn average_defense(&self) -> f32 {
        (self.resilience.value() as f32 + self.adaptability.value() as f32) / 2.0
    }

    pub fn group_average(&self, group: StatGroup) -> f32 {
        match group {
            StatGroup::Cognition => self.average_cognition(),
            StatGroup::Perception => self.average_perception(),
            StatGroup::Drive => self.average_drive(),
            StatGroup::Social => self.average_social(),
            StatGroup::Defense => self.average_defense(),
        }
    }

    pub fn highest_group(&self) -> StatGroup {
        use StatGroup::*;
        let groups = [Cognition, Perception, Drive, Social, Defense];
        *groups
            .iter()
            .max_by(|a, b| {
                self.group_average(**a)
                    .partial_cmp(&self.group_average(**b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
    }

    pub fn compare_group(&self, other: &Stats, group: StatGroup) -> std::cmp::Ordering {
        self.group_average(group)
            .partial_cmp(&other.group_average(group))
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> StatsConfig {
        StatsConfig {
            judgement: 1,
            creativity: 2,
            systems: 3,
            precision: 4,
            focus: 5,
            discipline: 6,
            empathy: 7,
            communication: 8,
            resilience: 9,
            adaptability: 10,
        }
    }

    fn uniform_config(val: u16) -> StatsConfig {
        StatsConfig {
            judgement: val,
            creativity: val,
            systems: val,
            precision: val,
            focus: val,
            discipline: val,
            empathy: val,
            communication: val,
            resilience: val,
            adaptability: val,
        }
    }

    #[test]
    fn total_counts_all_stats() {
        let stats: Stats = sample_config().into();
        assert_eq!(stats.total(), 55);
    }

    #[test]
    fn set_stat_changes_value_and_total() {
        let mut stats: Stats = sample_config().into();
        stats.set_stat(StatType::Judgement, 5);
        assert_eq!(stats.get_stat(StatType::Judgement), 5);
        // original total was 55, we increased judgement by 4
        assert_eq!(stats.total(), 59);
    }

    #[test]
    fn adjust_many_modifies_multiple_fields() {
        let mut stats: Stats = uniform_config(10).into();
        stats.adjust_many(&[
            (StatType::Judgement, 1.0),
            (StatType::Creativity, -2.0),
        ]);
        assert_eq!(stats.get_stat(StatType::Judgement), 11);
        assert_eq!(stats.get_stat(StatType::Creativity), 8);
    }

    #[test]
    fn normalize_to_reaches_target_total() {
        let mut stats: Stats = StatsConfig::default().into();
        stats.normalize_to(10);
        assert!(stats.total() >= 10);
    }

    #[test]
    fn lowest_stat_returns_minimum() {
        let stats: Stats = sample_config().into();
        assert_eq!(stats.lowest_stat(), StatType::Judgement);
    }

    #[test]
    fn stat_filter_respects_comparator() {
        let stats: Stats = sample_config().into();
        let result = stats.stat_filter(5, |v, t| v >= t);
        assert_eq!(
            result,
            vec![
                StatType::Focus,
                StatType::Discipline,
                StatType::Empathy,
                StatType::Communication,
                StatType::Resilience,
                StatType::Adaptability,
            ]
        );
    }

    #[test]
    fn highest_group_returns_expected_group() {
        let config = StatsConfig {
            judgement: 1,
            creativity: 2,
            systems: 10,
            precision: 10,
            focus: 6,
            discipline: 7,
            empathy: 2,
            communication: 2,
            resilience: 3,
            adaptability: 3,
        };
        let stats: Stats = config.into();
        assert_eq!(stats.highest_group(), StatGroup::Perception);
    }
}
