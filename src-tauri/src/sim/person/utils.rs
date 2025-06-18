use std::collections::HashMap;
use legion::{Entity, IntoQuery};
use std::sync::Arc;
use legion::world::SubWorld;
use tracing::warn;
use crate::sim::person::components::{Person, PersonId};
use crate::sim::person::skills::SkillId;
use crate::sim::registries::registry::Registry;



#[derive(Debug)]
pub struct PersonLookupResult<'a> {
    pub entity: Entity,
    pub person: &'a mut Person,
}

pub fn get_person_from_id<'a>(
    person_id: u32,
    person_registry: &Arc<Registry<PersonId, Entity>>,
    subworld: &'a mut SubWorld,
) -> Option<PersonLookupResult<'a>> {
    let Some(person_entity) = person_registry.get_entity_from_id(&PersonId(person_id)) else {
        warn!("Can't find person entity with ID:{:?}", person_id);
        return None;
    };

    let Ok(person) = <&mut Person>::query().get_mut(subworld, person_entity) else {
        warn!(
            "Can't find person component with ID:{:?} - entity exists but component missing!",
            person_id
        );
        return None;
    };

    Some(PersonLookupResult {
        entity: person_entity,
        person, // No clone - return the mutable reference
    })
}