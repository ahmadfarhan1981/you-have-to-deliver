use std::fmt;
use crate::sim::person::components::ProfilePicture;
use crate::sim::{
    person::components::Person, person::stats::Stats, resources::global::TickCounter,
};
use legion::{system, Entity};
use std::sync::Arc;
use dashmap::DashSet;
use legion::systems::CommandBuffer;
use tauri::Emitter;
use tracing::{error, info};
use crate::integrations::snapshots::SnapshotState;
use crate::integrations::snapshots_emitter::snapshots_emitter::{SnapshotEmitRegistry, SnapshotFieldEmitter};
use crate::integrations::ui::AppContext;
use crate::sim::person::skills::SkillId;
use crate::sim::registries::registry::Registry;

#[system]
pub fn increase_sim_tick(#[resource] tick_counter: &Arc<TickCounter>) {
    tick_counter.tick()
}

#[system]
pub fn print_person( #[resource] app_context: &Arc<AppContext>,
#[resource]skill_registry: &Arc<Registry<SkillId, Entity>>) {
    info!("{}",skill_registry)
// pub fn print_person(cmd: &mut CommandBuffer, e:&Entity, person: &Person, stats: &Stats, profile_picture: &ProfilePicture, #[resource] app_context: &Arc<AppContext>,) {
//     info!("Printing person...");
    //info!("Person: {:?}", person);

    // println!("Stats: {:?}", stats);
    // println!("Profile picture: {:?}", profile_picture);
}


#[derive(Default)]
pub struct UsedProfilePictureRegistry{
    pub used_profile_pictures: DashSet<ProfilePicture>
}

impl fmt::Debug for UsedProfilePictureRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UsedProfilePictureRegistry[{}]",  format!("{} entries", self.used_profile_pictures.len()))
        // write!(
        //     f,
        //     "{}[{}. next_id={}]",
        //     "PersonRegistry".bold().bright_cyan(),
        //     format!("{count} entries").green(),
        //     format!("{next}").yellow().bold()
        // )

    }
}

pub fn test () {
    let mut  snapshot_registry = SnapshotEmitRegistry::new();
    let snapshot_state = SnapshotState::default();
    let g = snapshot_state.game_speed;

    let x  = SnapshotFieldEmitter{ field: Arc::new(g), config: Default::default() };
    snapshot_registry.register(x);


}

pub trait ReplaceIfChanged {
    fn replace_if_changed(&mut self, new: Self) -> bool;
}

// impl<T> ReplaceIfChanged for T
// where
//     T: PartialEq + Sized,
// {
//     fn replace_if_changed(&mut self, new: Self) -> bool {
//         if *self != new {
//             *self = new;
//             true
//         } else {
//             false
//         }
//     }
// }
//
// pub trait ReplaceIfChangedFromRef {
//     fn replace_if_changed_ref(&mut self, new: &Self) -> bool;
// }
//
// impl<T> ReplaceIfChangedFromRef for T
// where
//     T: PartialEq + Clone,
// {
//     fn replace_if_changed_ref(&mut self, new: &Self) -> bool {
//         if self != new {
//             *self = new.clone();
//             true
//         } else {
//             false
//         }
//     }
// }
