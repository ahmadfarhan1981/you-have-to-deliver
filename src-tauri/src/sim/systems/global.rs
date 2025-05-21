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
use crate::integrations::ui::AppContext;

#[system]
pub fn increase_sim_tick(#[resource] tick_counter: &Arc<TickCounter>) {
    tick_counter.tick()
}

#[system]
pub fn print_person( #[resource] app_context: &Arc<AppContext>,) {
// pub fn print_person(cmd: &mut CommandBuffer, e:&Entity, person: &Person, stats: &Stats, profile_picture: &ProfilePicture, #[resource] app_context: &Arc<AppContext>,) {
    info!("Printing person...");
    let result = app_context
        .app_handle
        .emit("test_emit", "hello frontend");

    if let Err(err) = result {
        error!("Failed to emit game_speed_update: {:?}", err);
    }
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