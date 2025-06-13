use std::collections::HashMap;
use crate::master_data::skills::SkillDef;
use crate::sim::person::stats::StatType;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::Display;
use tracing::{error, warn};

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



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Domain {
    Execution,
    Coordination,
    Interpersonal,
    Contextual,
}

impl std::fmt::Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name =match self {
            Domain::Execution => "Execution",
            Domain::Coordination => "Coordination",
            Domain::Interpersonal => "Interpersonal",
            Domain::Contextual => "Contextual",
        };
        write!(f, "{}", name)
    }
}
impl FromStr for Domain {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().trim() {
            "execution" => Ok(Domain::Execution),
            "coordination" => Ok(Domain::Coordination),
            "interpersonal" => Ok(Domain::Interpersonal),
            "contextual" => Ok(Domain::Contextual),
            other => {error!("Seeing {}", other );Err(())},
        }
    }
}
#[derive(Clone)]
pub struct GlobalSkill {
    pub id: SkillId,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub tier: Tier,
    pub domain: Vec<Domain>,
    pub feedforward: Vec<SkillLink>,
    pub feedback: Vec<SkillLink>,
    pub related_stats: Vec<StatType>,
}
use std::fmt;
impl fmt::Debug for GlobalSkill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to Display for the main formatting
        write!(f, "{}", self)
    }
}

impl fmt::Display for GlobalSkill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{} - {}]\n{}\n",
            self.name,
            self.tier.to_string(),
            self.domain.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(","), //.fold(String::from(""),|d, domain |{  d += ", " + domain.to_string() }),
            self.description
        )
    }
}

impl From<&SkillDef> for GlobalSkill {
    fn from(value: &SkillDef) -> Self {
        Self {
            id: SkillId(0),
            slug: value.id.parse().unwrap(),
            name: value.name.parse().unwrap(),
            description: value.description.parse().unwrap(),
            tier: value.tier.parse().unwrap(),
            domain: value.domain.split(",").fold(Vec::<Domain>::new(),|mut acc,s|{ acc.push(s.parse().unwrap_or_else(|_| { error!("Failed to parse into domain: {}. Error:", s );Domain::Execution})); acc } ),// .map(|val|{ value}) .parse().unwrap_or_else(|_| { error!("{}", value.domain );Domain::Execution}),
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


#[derive(Clone)]
pub struct SkillLink {
    target: SkillId,
    factor: u8,
}

struct SkillThreshold {
    upper: u8,
    lower: u8,
}
pub struct AssignedSkill {
    skill :SkillId,
    value: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SkillSet {
    pub(crate) skills: HashMap<SkillId, u32>,
}

pub mod ecs_components {
    use super::*;
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TierFoundational;
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TierConceptual;
    #[derive(Copy, Clone, Debug, Default)]
    pub struct TierApplied;

    #[derive(Copy, Clone, Debug, Default)]
    pub struct DomainExecution;
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DomainCoordination;
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DomainInterpersonal;
    #[derive(Copy, Clone, Debug, Default)]
    pub struct DomainContext;
}