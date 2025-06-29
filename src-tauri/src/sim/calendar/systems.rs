use std::sync::Arc;
use legion::{system, IntoQuery, Entity, Query};
use legion::world::SubWorld;
use crate::sim::calendar::components::{CalendarEvent, CalendarEventRegistry};
use crate::sim::registries::registry::Registry;
use crate::sim::team::components::{Team, TeamId};

#[system]
#[read_component(CalendarEvent)]
#[read_component(Entity)]
pub fn sync_registry_from_calendar_event(
    world: &mut SubWorld,
    #[resource] calendar_event_registry: &mut CalendarEventRegistry,
) {
    
    calendar_event_registry.clear();
    let mut query = <(Entity, &CalendarEvent)>::query();
    for (entity, calendar_event) in query.iter(world) {
        calendar_event_registry.insert(calendar_event.id, *entity);
    }
}

