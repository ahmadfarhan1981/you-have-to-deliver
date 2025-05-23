use crate::integrations::snapshots::{
    GameSpeedSnapshot, PersonSnapshot, ProfilePictureSnapshot, SnapshotState, StatsSnapshot,
    TickSnapshot,
};
use crate::sim::game_speed::components::GameSpeedManager;
use crate::sim::person::components::{Person, PersonId, ProfilePicture};
use crate::sim::person::stats::Stats;
use crate::sim::resources::global::TickCounter;
use legion::system;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use parking_lot::RwLock;

#[system]
pub fn push_tick_counter_to_integration(
    #[resource] app_state: &Arc<SnapshotState>,
    #[resource] tick_counter: &Arc<TickCounter>,
) {
    app_state.tick.set(&tick_counter)
}

#[system]
pub fn push_game_speed_to_integration(
    #[resource] app_state: &Arc<SnapshotState>,
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] game_speed_manager: &Arc<RwLock<GameSpeedManager>>,
) {
    // TODO
    app_state.tick.set(&tick_counter);

    app_state.game_speed.value.load().tick.set(tick_counter);
    let atomicu_speed : u8 = game_speed_manager.read().game_speed.into();
    if app_state.game_speed.value.load().game_speed.load(Ordering::Relaxed) != atomicu_speed { app_state.game_speed.value.load().game_speed.store(atomicu_speed, Ordering::Relaxed); }

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
