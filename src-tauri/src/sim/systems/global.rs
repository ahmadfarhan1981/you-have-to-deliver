use legion::system;
use crate::sim::resources::global::TickCounter;

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
