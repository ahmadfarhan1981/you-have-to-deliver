use legion::system;
use super::registry::PersonRegistry;

#[system]
pub fn generate_employees(#[resource] mut registry: &mut PersonRegistry) {
    
    // println!("tick {}", counter.tick);
}