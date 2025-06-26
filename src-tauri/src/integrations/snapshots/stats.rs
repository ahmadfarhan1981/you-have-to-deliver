use crate::sim::person::stats::{StatType, Stats};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StatsSnapshot {
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

impl From<&Stats> for StatsSnapshot {
    fn from(s: &Stats) -> Self {
        Self {
            judgement: s.get_stat(StatType::Judgement),
            creativity: s.get_stat(StatType::Creativity),
            systems: s.get_stat(StatType::Systems),
            precision: s.get_stat(StatType::Precision),
            focus: s.get_stat(StatType::Focus),
            discipline: s.get_stat(StatType::Discipline),
            empathy: s.get_stat(StatType::Empathy),
            communication: s.get_stat(StatType::Communication),
            resilience: s.get_stat(StatType::Resilience),
            adaptability: s.get_stat(StatType::Adaptability),
        }
    }
}

impl PartialEq<&Stats> for StatsSnapshot {
    fn eq(&self, other: &&Stats) -> bool {
        self.judgement == other.get_stat(StatType::Judgement)
            && self.creativity == other.get_stat(StatType::Creativity)
            && self.systems == other.get_stat(StatType::Systems)
            && self.precision == other.get_stat(StatType::Precision)
            && self.focus == other.get_stat(StatType::Focus)
            && self.discipline == other.get_stat(StatType::Discipline)
            && self.empathy == other.get_stat(StatType::Empathy)
            && self.communication == other.get_stat(StatType::Communication)
            && self.resilience == other.get_stat(StatType::Resilience)
            && self.adaptability == other.get_stat(StatType::Adaptability)
    }
}