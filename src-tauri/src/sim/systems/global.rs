use crate::sim::person::components::ProfilePicture;
use crate::sim::{
    person::components::Person, person::stats::Stats, resources::global::TickCounter,
};
use legion::system;
use std::sync::Arc;

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
