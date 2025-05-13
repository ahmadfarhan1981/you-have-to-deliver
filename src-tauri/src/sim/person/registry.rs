use std::collections::HashMap;

use legion::world::Entity;
use super::utils::PersonId;

#[derive(Default)]
pub struct PersonRegistry {
    pub id_to_entity: HashMap<PersonId, Entity>,
    pub entity_to_id: HashMap<Entity, PersonId>,
    pub next_id: u32,
}

impl PersonRegistry {
    pub fn generate_id(&mut self) -> PersonId {
        let id = PersonId(self.next_id);
        self.next_id += 1;
        id
    }

    pub fn insert(&mut self, id: PersonId, entity: Entity) {
        self.id_to_entity.insert(id, entity);
        self.entity_to_id.insert(entity, id);
    }

    pub fn get_entity(&self, id: &PersonId) -> Option<Entity> {
        self.id_to_entity.get(id).copied()
    }

    pub fn get_person(&self, entity: &Entity) -> Option<PersonId> {
        self.entity_to_id.get(entity).copied()
    }

    pub fn remove(&mut self, id: &PersonId) {
        if let Some(entity) = self.id_to_entity.remove(id) {
            self.entity_to_id.remove(&entity);
        }
    }
}
