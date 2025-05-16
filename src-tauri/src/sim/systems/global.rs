use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Instant;
use legion::system;
use crate::integrations::ui::AppState;
use crate::sim::{person::stats::Stats, person::components::Person, resources::global::TickCounter};
use crate::sim::person::components::ProfilePicture;

#[system]
pub fn push_tick_counter_to_integration(#[resource] app_state: &Arc<AppState>, #[resource] tick_counter: &Arc<TickCounter>) {
    app_state.tick.swap(tick_counter.tick.load(Ordering::Relaxed), Ordering::Relaxed);
}

#[system]
pub fn increase_sim_tick( #[resource] tick_counter: &Arc<TickCounter>) {
    tick_counter.tick.fetch_add(1, Ordering::Relaxed);
}

#[system(for_each)]
pub fn print_person(person: &Person, stats: &Stats, profile_picture: &ProfilePicture) {
    // println!("Person: {:?}", person);
    // println!("Stats: {:?}", stats);
    // println!("Profile picture: {:?}", profile_picture);
}