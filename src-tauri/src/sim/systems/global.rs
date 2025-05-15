use legion::system;
use crate::sim::{person::stats::Stats, person::components::Person, resources::global::TickCounter};
use crate::sim::person::components::ProfilePicture;

#[system]
pub fn tick_counter(#[resource] mut counter: &mut TickCounter) {
    counter.tick += 1;
    // println!("tick {}", counter.tick);
}

#[system(for_each)]
pub fn print_person(person: &Person, stats: &Stats, profile_picture: &ProfilePicture) {
    // println!("Person: {:?}", person);
    // println!("Stats: {:?}", stats);
    // println!("Profile picture: {:?}", profile_picture);
}