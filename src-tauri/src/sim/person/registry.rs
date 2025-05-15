use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use dashmap::DashMap;
use super::components::PersonId;
use legion::world::Entity;

#[derive(Default)]
pub struct PersonRegistry {
    id_to_entity: DashMap<PersonId, Entity>,
    entity_to_id: DashMap<Entity, PersonId>,
    next_id: AtomicU32,
}

impl PersonRegistry {
    pub fn new() -> Self {
        Self {
            id_to_entity: DashMap::new(),
            entity_to_id: DashMap::new(),
            next_id: AtomicU32::new(0),
        }
    }
    pub fn generate_id(&self) -> PersonId {
        PersonId(self.next_id.fetch_add(1, Ordering::Relaxed))
    }

    pub fn insert(&self, id: PersonId, entity: Entity) {
        self.id_to_entity.insert(id, entity);
        self.entity_to_id.insert(entity, id);
    }

    pub fn get_entity(&self, id: &PersonId) -> Option<Entity> {
        self.id_to_entity.get(id).map(|entry| *entry.value())
    }

    pub fn get_person(&self, entity: &Entity) -> Option<PersonId> {
        self.entity_to_id.get(entity).map(|entry| *entry.value())
    }

    pub fn remove(&self, id: &PersonId) {
        if let Some((_id, entity)) = self.id_to_entity.remove(id) {
            self.entity_to_id.remove(&entity);
        }
    }
}
