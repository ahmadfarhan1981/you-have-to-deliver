use legion::system;
use crate::sim::{person::stats::Stats, person::components::Person, resources::global::TickCounter};

#[system]
pub fn tick_counter(#[resource] mut counter: &mut TickCounter) {
    counter.tick += 1;
    // println!("tick {}", counter.tick);
}


// #[system]
// pub fn tick_counter(#[resource] mut counter: &mut TickCounter) {
//     counter.tick += 1;
//     // println!("tick {}", counter.tick);
// }
#[system(for_each)]
pub fn print_person(person: &Person, stats: &Stats) {
    // println!("Person: {:?}", person);
    // println!("Stats: {:?}", stats);
}