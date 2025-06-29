use crate::sim::calendar::components::calendar_event_id::CalendarEventId;
use crate::sim::registries::registry::Registry;
use legion::Entity;

pub type CalendarEventRegistry = Registry<CalendarEventId, Entity>;
