use serde::{Deserialize, Serialize};
use crate::integrations::snapshots::needs::{EnergySnapshot, HungerSnapshot};
use crate::integrations::snapshots::personality::PersonalitySnapshot;
use crate::integrations::snapshots::skills::SkillSetSnapshot;
use crate::integrations::snapshots::profile_picture::ProfilePictureSnapshot;
use crate::integrations::snapshots::stats::StatsSnapshot;
use crate::sim::person::components::{Person, ProfilePicture};
use crate::sim::person::needs::{Energy, Hunger};
use crate::sim::person::personality_matrix::PersonalityMatrix;
use crate::sim::person::skills::SkillSet;
use crate::sim::person::spawner::TalentGrade;
use crate::sim::person::stats::Stats;
use crate::sim::registries::registry::GlobalSkillNameMap;
use crate::sim::sim_date::sim_date::SimDate;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PersonSnapshot {
    pub(crate) stats: StatsSnapshot,
    pub(crate) profile_picture: ProfilePictureSnapshot,
    pub(crate) person_id: u32,
    pub(crate) name: String,
    pub(crate) gender: String,
    pub(crate) personality: PersonalitySnapshot,
    pub(crate) assigned_skill: SkillSetSnapshot,
    pub talent_grade: TalentGrade,
    pub team: Option<u32>,
    /// The tick number this snapshot was last updated
    pub updated: u64,
    pub joined_tick: u64,
    pub joined_gamedate: SimDate,
    pub energy: EnergySnapshot,
    pub hunger: HungerSnapshot,
}

impl
    From<(
        &Person,
        &ProfilePicture,
        &Stats,
        &PersonalityMatrix,
        &SkillSet,
        u64
    )> for PersonSnapshot
{
    fn from(
        (person, picture, stats, personality, skillset, current_tick): (
            &Person,
            &ProfilePicture,
            &Stats,
            &PersonalityMatrix,
            &SkillSet,
            u64
        ),
    ) -> Self {
        Self {
            person_id: person.person_id.0,
            name: person.name.clone(),
            gender: person.gender.to_string(),
            stats: StatsSnapshot::from(stats),
            profile_picture: ProfilePictureSnapshot::from(picture),
            personality: PersonalitySnapshot::from(personality),
            assigned_skill: SkillSetSnapshot::from(skillset),
            updated: current_tick,
            joined_tick: person.joined,
            team: person.team.map(|id| id.0),
            talent_grade: person.talent_grade,
            joined_gamedate: SimDate::from(person.joined),
            energy: EnergySnapshot::default(),
            hunger: HungerSnapshot::default(),
        }
    }
}