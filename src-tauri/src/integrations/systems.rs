use crate::integrations::snapshots::{PersonSnapshot, ProfilePictureSnapshot, StatsSnapshot};
use crate::integrations::ui::SnapshotState;
use crate::sim::person::components::{Person, PersonId, ProfilePicture};
use crate::sim::person::stats::Stats;
use crate::sim::resources::global::TickCounter;
use legion::system;
use std::sync::Arc;

#[system]
pub fn push_tick_counter_to_integration(
    #[resource] app_state: &Arc<SnapshotState>,
    #[resource] tick_counter: &Arc<TickCounter>,
) {
    app_state.tick.set(tick_counter.value());
}

#[system]
pub fn clear_person_list(#[resource] app_state: &Arc<SnapshotState>) {
    app_state.persons.clear();
}

#[system(for_each)]
pub fn push_persons_to_integration(
    #[resource] app_state: &Arc<SnapshotState>,
    person: &Person,
    stats: &Stats,
    profile_picture: &ProfilePicture,
) {
    let PersonId(id) = person.person_id;
    let profile_picture = ProfilePictureSnapshot {
        gender: profile_picture.gender.to_string(),
        category: profile_picture.category.as_file_category_number(),
        batch: profile_picture.batch,
        index: profile_picture.index,
    };
    let stats = StatsSnapshot::from(*stats);
    let person = PersonSnapshot {
        stats,
        profile_picture,
        person_id: id,
        name: person.name.clone(),
        gender: person.gender.to_string(),
    };
    app_state.persons.insert(id, person);
}


