use std::fmt;
use crate::sim::person::components::ProfilePicture;
use crate::sim::{
    person::components::Person, person::stats::Stats, resources::global::TickCounter,
};
use legion::system;
use std::sync::Arc;
use dashmap::DashSet;
use owo_colors::OwoColorize;

#[system]
pub fn increase_sim_tick(#[resource] tick_counter: &Arc<TickCounter>) {
    tick_counter.tick()
}

#[system(for_each)]
pub fn print_person(person: &Person, stats: &Stats, profile_picture: &ProfilePicture) {
    // println!("Person: {:?}", person);
    // println!("Stats: {:?}", stats);
    // println!("Profile picture: {:?}", profile_picture);
}


#[derive(Default)]
pub struct UsedProfilePictureRegistry{
    pub used_profile_pictures: DashSet::<ProfilePicture>
}

impl fmt::Debug for UsedProfilePictureRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UsedProfilePictureRegistry[{}]",  format!("{} entries", self.used_profile_pictures.len()).green())
        // write!(
        //     f,
        //     "{}[{}. next_id={}]",
        //     "PersonRegistry".bold().bright_cyan(),
        //     format!("{count} entries").green(),
        //     format!("{next}").yellow().bold()
        // )

    }
}