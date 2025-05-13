use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Stats {
    // Cognition
    pub judgement: f32,
    pub creativity: f32,
    // Perception
    pub systems: f32,
    pub precision: f32,
    // Drive
    pub focus: f32,
    pub discipline: f32,
    // Social
    pub empathy: f32,
    pub communication: f32,
    // Defense
    pub resilience: f32,
    pub adaptability: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatGroup {
    Cognition,
    Perception,
    Drive,
    Social,
    Defense,
}

impl Stats {
    pub fn average_cognition(&self) -> f32 {
        (self.judgement + self.creativity) / 2.0
    }

    pub fn average_perception(&self) -> f32 {
        (self.systems + self.precision) / 2.0
    }

    pub fn average_drive(&self) -> f32 {
        (self.focus + self.discipline) / 2.0
    }

    pub fn average_social(&self) -> f32 {
        (self.empathy + self.communication) / 2.0
    }

    pub fn average_defense(&self) -> f32 {
        (self.resilience + self.adaptability) / 2.0
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
