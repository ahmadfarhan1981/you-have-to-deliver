use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stat {
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
impl From<StatsConfig> for Stats{
    fn from(config: StatsConfig) -> Self{
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatGroup {
    Cognition,
    Perception,
    Drive,
    Social,
    Defense,
}

impl Stats {
    pub fn get_stat(&self, stat: Stat) -> u16 {
        match stat {
            Stat::Judgement => self.judgement,
            Stat::Creativity => self.creativity,
            Stat::Systems => self.systems,
            Stat::Precision => self.precision,
            Stat::Focus => self.focus,
            Stat::Discipline => self.discipline,
            Stat::Empathy => self.empathy,
            Stat::Communication => self.communication,
            Stat::Resilience => self.resilience,
            Stat::Adaptability => self.adaptability,
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
