use serde::{Deserialize, Serialize};
use crate::master_data::skills::GLOBAL_SKILLS;
use crate::sim::person::skills::{GlobalSkill, SkillSet};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssignedSkillSnapshot {
    pub skill_id: String,
    pub value: u32,
    pub skill_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillSetSnapshot {
    pub assigned_skills: Vec<AssignedSkillSnapshot>,
}

impl From<&SkillSetSnapshot> for SkillSetSnapshot {
    fn from(value: &SkillSetSnapshot) -> Self {
        Self{
            assigned_skills: value.assigned_skills.iter().map(|snapshot| snapshot.clone()).collect(),
        }
    }
}

impl PartialEq<&SkillSetSnapshot> for SkillSetSnapshot {
    fn eq(&self, other: &&SkillSetSnapshot) -> bool {
        self.assigned_skills == other.assigned_skills
    }
}


impl From<&SkillSet> for SkillSetSnapshot {
    fn from(value: &SkillSet) -> Self {
        let skill_list = value.skills.iter().collect::<Vec<_>>();

        let snapshot_list = skill_list
            .iter()
            .map(|(s, v)| {
                let id = (**s).clone();
                let val = **v;
                let name = GLOBAL_SKILLS.get().unwrap().get(s).unwrap().name.clone();

                AssignedSkillSnapshot {
                    skill_id: id.0.to_string(),
                    value: val,
                    skill_name: name,
                }
            })
            .collect::<Vec<_>>();

        SkillSetSnapshot {
            assigned_skills: snapshot_list,
        }
    }
}


pub struct GlobalSkillSnapshot {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tier: String,
    pub domain: String,
}

impl From<&GlobalSkill> for GlobalSkillSnapshot {
    fn from(value: &GlobalSkill) -> Self {
        Self {
            id: value.id.0.clone(),
            name: value.name.clone(),
            description: value.description.clone(),
            tier: value.tier.to_string(),
            domain: value
                .domain
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(","),
        }
    }
}