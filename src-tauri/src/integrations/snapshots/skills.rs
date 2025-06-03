use serde::{Deserialize, Serialize};
use crate::sim::person::skills::{GlobalSkill, SkillSet};
use crate::sim::registries::registry::GlobalSkillNameMap;

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

impl SkillSetSnapshot {
    pub fn from_sim(skillset: &SkillSet, name_lookup: &GlobalSkillNameMap) -> Self {
        let skill_list = skillset.skills.iter().collect::<Vec<_>>();

        let snapshot_list = skill_list
            .iter()
            .map(|(s, v)| {
                let id = **s;
                let val = **v;
                let binding = name_lookup.0.get(&id).unwrap();
                let name = binding.value();

                AssignedSkillSnapshot {
                    skill_id: id.0.to_string(),
                    value: val,
                    skill_name: name.clone(),
                }
            })
            .collect::<Vec<_>>();

        SkillSetSnapshot {
            assigned_skills: snapshot_list,
        }


    }
}

pub struct GlobalSkillSnapshot {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub tier: String,
    pub domain: String,
}

impl From<&GlobalSkill> for GlobalSkillSnapshot {
    fn from(value: &GlobalSkill) -> Self {
        Self {
            id: value.id.0,
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