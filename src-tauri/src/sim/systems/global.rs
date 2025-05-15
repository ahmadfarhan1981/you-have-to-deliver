use std::sync::Arc;
use std::sync::atomic::Ordering;
use legion::system;
use crate::integrations::ui::AppState;
use crate::sim::{person::stats::Stats, person::components::Person, resources::global::TickCounter};
use crate::sim::person::components::ProfilePicture;

#[system]
pub fn tick_counter(#[resource] app_state: &Arc<AppState>) {
    app_state.tick.fetch_add(1, Ordering::Relaxed);
    // println!("tick {}", counter.tick);
}

#[system(for_each)]
pub fn print_person(person: &Person, stats: &Stats, profile_picture: &ProfilePicture) {
    // println!("Person: {:?}", person);
    // println!("Stats: {:?}", stats);
    // println!("Profile picture: {:?}", profile_picture);
}