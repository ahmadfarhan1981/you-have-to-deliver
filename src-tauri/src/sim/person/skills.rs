use std::str::FromStr;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tier {
    Foundational,
    Conceptual,
    Applied,
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Tier::Foundational => "Foundational",
            Tier::Conceptual => "Conceptual",
            Tier::Applied => "Applied",
        };
        write!(f, "{}", name)
    }
}
impl std::str::FromStr for Tier {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "foundational" => Ok(Tier::Foundational),
            "conceptual" => Ok(Tier::Conceptual),
            "applied" => Ok(Tier::Applied),
            _ => Err(()),
        }
    }
}


pub struct GlobalSkill {
    pub id: SkillId,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub tier: Tier,
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
            tier: Tier::from_str(value.tier).unwrap(),
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
