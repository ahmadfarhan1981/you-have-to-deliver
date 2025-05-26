use rand::prelude::SliceRandom;
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumIter;
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Stats {
    // Cognition
    judgement: u16,
    judgement_raw: u32,
    creativity: u16,
    creativity_raw: u32,

    // Perception
    systems: u16,
    systems_raw: u32,
    precision: u16,
    precision_raw: u32,

    // Drive
    focus: u16,
    focus_raw: u32,
    discipline: u16,
    discipline_raw: u32,

    // Social
    empathy: u16,
    empathy_raw: u32,
    communication: u16,
    communication_raw: u32,

    // Defense
    resilience: u16,
    resilience_raw: u32,
    adaptability: u16,
    adaptability_raw: u32,
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

        let mut rng = rng();
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
                self.set_stat(*stat, current + increment);
                if self.total() >= target {
                    break;
                }
            }
        }
    }

    /// Sets the given stat to a specific value.
    ///
    /// This updates both the visible stat and its raw counterpart.
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
        let clamped = value.max(0); // optional, in case negative u16s are passed indirectly

        macro_rules! set {
            ($field:ident, $raw_field:ident) => {{
                self.$field = clamped;
                self.$raw_field = (clamped as u32) * 1000;
            }};
        }

        match stat {
            StatType::Judgement => set!(judgement, judgement_raw),
            StatType::Creativity => set!(creativity, creativity_raw),
            StatType::Systems => set!(systems, systems_raw),
            StatType::Precision => set!(precision, precision_raw),
            StatType::Focus => set!(focus, focus_raw),
            StatType::Discipline => set!(discipline, discipline_raw),
            StatType::Empathy => set!(empathy, empathy_raw),
            StatType::Communication => set!(communication, communication_raw),
            StatType::Resilience => set!(resilience, resilience_raw),
            StatType::Adaptability => set!(adaptability, adaptability_raw),
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
    /// Each `StatType` will have its raw value modified by `amount * 1000`.
    /// All updates are applied first, then `sync_from_raw()` is called once at the end.
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
            let delta = (amount * 1000.0).round() as i32;

            macro_rules! apply {
                ($field:ident, $raw_field:ident) => {{
                    let raw = self.$raw_field as i32 + delta;
                    self.$raw_field = raw.max(0) as u32;
                }};
            }

            match stat {
                StatType::Judgement => apply!(judgement, judgement_raw),
                StatType::Creativity => apply!(creativity, creativity_raw),
                StatType::Systems => apply!(systems, systems_raw),
                StatType::Precision => apply!(precision, precision_raw),
                StatType::Focus => apply!(focus, focus_raw),
                StatType::Discipline => apply!(discipline, discipline_raw),
                StatType::Empathy => apply!(empathy, empathy_raw),
                StatType::Communication => apply!(communication, communication_raw),
                StatType::Resilience => apply!(resilience, resilience_raw),
                StatType::Adaptability => apply!(adaptability, adaptability_raw),
            }
        }

        self.sync_from_raw();
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
        self.judgement
            + self.creativity
            + self.systems
            + self.precision
            + self.focus
            + self.discipline
            + self.empathy
            + self.communication
            + self.resilience
            + self.adaptability
    }
}
impl From<StatsConfig> for Stats {
    fn from(config: StatsConfig) -> Self {
        Self {
            judgement: config.judgement,
            judgement_raw: config.judgement as u32 * 1000,
            creativity: config.creativity,
            creativity_raw: config.creativity as u32 * 1000,
            systems: config.systems,
            systems_raw: config.systems as u32 * 1000,
            precision: config.precision,
            precision_raw: config.precision as u32 * 1000,
            focus: config.focus,
            focus_raw: config.focus as u32 * 1000,
            discipline: config.discipline,
            discipline_raw: config.discipline as u32 * 1000,
            empathy: config.empathy,
            empathy_raw: config.empathy as u32 * 1000,
            communication: config.communication,
            communication_raw: config.communication as u32 * 1000,
            resilience: config.resilience,
            resilience_raw: config.resilience as u32 * 1000,
            adaptability: config.adaptability,
            adaptability_raw: config.adaptability as u32 * 1000,
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
        match stat {
            StatType::Judgement => self.judgement,
            StatType::Creativity => self.creativity,
            StatType::Systems => self.systems,
            StatType::Precision => self.precision,
            StatType::Focus => self.focus,
            StatType::Discipline => self.discipline,
            StatType::Empathy => self.empathy,
            StatType::Communication => self.communication,
            StatType::Resilience => self.resilience,
            StatType::Adaptability => self.adaptability,
        }
    }
    pub fn average_cognition(&self) -> f32 {
        (self.judgement as f32 + self.creativity as f32) / 2.0
    }

    pub fn average_perception(&self) -> f32 {
        (self.systems as f32 + self.precision as f32) / 2.0
    }

    pub fn average_drive(&self) -> f32 {
        (self.focus as f32 + self.discipline as f32) / 2.0
    }

    pub fn average_social(&self) -> f32 {
        (self.empathy as f32 + self.communication as f32) / 2.0
    }

    pub fn average_defense(&self) -> f32 {
        (self.resilience as f32 + self.adaptability as f32) / 2.0
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

    pub fn sync_from_raw(&mut self) {
        self.judgement = (self.judgement_raw / 1000) as u16;
        self.creativity = (self.creativity_raw / 1000) as u16;

        self.systems = (self.systems_raw / 1000) as u16;
        self.precision = (self.precision_raw / 1000) as u16;

        self.focus = (self.focus_raw / 1000) as u16;
        self.discipline = (self.discipline_raw / 1000) as u16;

        self.empathy = (self.empathy_raw / 1000) as u16;
        self.communication = (self.communication_raw / 1000) as u16;

        self.resilience = (self.resilience_raw / 1000) as u16;
        self.adaptability = (self.adaptability_raw / 1000) as u16;
    }
}
