use crate::sim::person::components::ProfilePicture;
use crate::sim::person::skills::{GlobalSkill, SkillId, SkillSet};
use crate::sim::registries::registry::Registry;
use crate::sim::{
    person::components::Person, person::stats::Stats, resources::global::TickCounter,
};
use dashmap::DashSet;
use legion::world::SubWorld;
use legion::{system, Entity, Query};
use rayon::prelude::*;
use std::fmt;
use std::sync::Arc;
use bincode::{Decode, Encode};
use crate::integrations::snapshots::snapshots::SnapshotState;

#[system]
pub fn increase_sim_tick(#[resource] tick_counter: &Arc<TickCounter>) {
    tick_counter.tick()
}

#[system]
pub fn print_person(
    // global_skill: Option<&GlobalSkill>,
    // domain_interpersonal: &DomainInterpersonal,
    // stats: &Stats,
    // person: &Person,
    // skill_set: &SkillSet,
    query: &mut Query<(&Stats, &Person, &SkillSet)>,
    #[resource] app_state: &Arc<SnapshotState>,
    query_skill: &mut Query<&GlobalSkill>,
    world: &mut SubWorld,
) {

}

#[derive(Default)]
pub struct UsedProfilePictureRegistry {
    pub used_profile_pictures: DashSet<ProfilePicture>,
}

impl fmt::Debug for UsedProfilePictureRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UsedProfilePictureRegistry[{}]",
            format!("{} entries", self.used_profile_pictures.len())
        )
    }
}
impl Encode for UsedProfilePictureRegistry {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        // Convert DashSet to Vec for encoding
        let vec: Vec<ProfilePicture> = self.used_profile_pictures
            .iter()
            .map(|item| item.clone())
            .collect();

        vec.encode(encoder)
    }
}

impl Decode<()> for UsedProfilePictureRegistry {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        // Decode as Vec first
        let vec: Vec<ProfilePicture> = Vec::decode(decoder)?;

        // Convert Vec back to DashSet
        let used_profile_pictures = DashSet::new();
        for item in vec {
            used_profile_pictures.insert(item);
        }

        Ok(UsedProfilePictureRegistry {
            used_profile_pictures,
        })
    }
}
