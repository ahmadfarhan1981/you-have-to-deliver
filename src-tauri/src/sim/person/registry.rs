use std::fmt;
use super::components::PersonId;
use dashmap::DashMap;
use legion::world::Entity;
use std::sync::atomic::{AtomicU32, Ordering};
use owo_colors::OwoColorize;

#[derive(Default)]
pub struct PersonRegistry {
    id_to_entity: DashMap<PersonId, Entity>,
    entity_to_id: DashMap<Entity, PersonId>,
    next_id: AtomicU32,
}
impl fmt::Debug for PersonRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count = self.id_to_entity.len();
        let next = self.next_id.load(std::sync::atomic::Ordering::Relaxed);

        write!(
            f,
            "{}[{}. next_id={}]",
            "PersonRegistry".bold().bright_cyan(),
            format!("{count} entries").green(),
            format!("{next}").yellow().bold()
        )
    }
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
