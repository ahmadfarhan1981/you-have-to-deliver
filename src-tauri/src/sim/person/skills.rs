use crate::master_data::skills::SkillDef;
use crate::sim::person::stats::StatType;
use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct SkillId(pub u32);

struct SkillRequirement {
    direct_requirement: Vec<SkillLink>,
    indirect_requirement: Vec<SkillLink>,
}

struct SkillBoost {
    direct_boost: Vec<SkillLink>,
    indirect_boost: Vec<SkillLink>,
}

pub struct GlobalSkill {
    pub id: SkillId,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub feedforward: Vec<SkillLink>,
    pub feedback: Vec<SkillLink>,
    pub related_stats: Vec<StatType>,
}
impl From<&SkillDef> for GlobalSkill {
    fn from(value: &SkillDef) -> Self {
        Self {
            id: SkillId(0),
            slug: value.id.parse().unwrap(),
            name: value.name.parse().unwrap(),
            description: value.description.parse().unwrap(),
            feedforward: vec![],
            feedback: vec![],
            related_stats: value.related_stats.iter().fold(Vec::new(), |mut acc, s| {
                match s.parse::<StatType>() {
                    Ok(stat) => acc.push(stat),
                    Err(e) => {
                        warn!("Failed to parse stat '{}': {:?}", s, e);
                    }
                }
                acc
            }),
        }
    }
}

struct SkillLink {
    target: SkillId,
    factor: u8,
}

struct SkillThreshold {
    upper: u8,
    lower: u8,
}
struct AssignedSkill {}
