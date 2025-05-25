use crate::integrations::snapshots::{
    PersonSnapshot, ProfilePictureSnapshot, SnapshotState, StatsSnapshot,
};
use crate::sim::game_speed::components::GameSpeedManager;
use crate::sim::person::components::{Person, PersonId, ProfilePicture};
use crate::sim::person::spawner::spawn_person;
use crate::sim::person::stats::Stats;
use crate::sim::resources::global::{Dirty, TickCounter};
use crate::sim::utils::snapshots::replace_if_changed;
use dashmap::Entry;
use legion::systems::CommandBuffer;
use legion::{system, Entity};
use parking_lot::RwLock;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tracing::info;

#[system]
pub fn push_game_speed_snapshots(
    #[resource] app_state: &Arc<SnapshotState>,
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] game_speed_manager: &Arc<RwLock<GameSpeedManager>>,
) {
    app_state.tick.set(&tick_counter);

    app_state.game_speed.value.load().tick.set(tick_counter);
    let game_speed: u8 = game_speed_manager.read().game_speed.into();
    let snapshot_speed = app_state
        .game_speed
        .value
        .load()
        .game_speed
        .load(Ordering::Relaxed);
    let speed_changed = snapshot_speed != game_speed;

    if speed_changed {
        app_state
            .game_speed
            .value
            .load()
            .game_speed
            .store(game_speed, Ordering::Relaxed);
    }
}

#[system(for_each)]
pub fn push_persons_to_integration(
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] app_state: &Arc<SnapshotState>,
    entity: &Entity,
    person: &Person,
    stats: &Stats,
    profile_picture: &ProfilePicture,
    _dirty: &Dirty,
    cmd: &mut CommandBuffer,
) {
    info!(person.name);
    let current_tick = tick_counter.value();
    let registry = &app_state.persons;

    match registry.entry(person.person_id.0) {
        Entry::Occupied(mut existing) => {
            let existing_person = existing.get_mut();
            let mut changed = false;

            //basic person fields
            if existing_person.name != person.name {
                changed = true;
                existing_person.name = person.name.clone();
            }
            if existing_person.gender != person.gender.to_string() {
                changed = true;
                existing_person.gender = person.gender.to_string();
            }

            changed |= replace_if_changed(&mut existing_person.stats, *stats);
            changed |= replace_if_changed(&mut existing_person.profile_picture, *profile_picture);

            if changed {
                existing_person.updated = current_tick;
            }
        }
        Entry::Vacant(vacant) => {
            let person = PersonSnapshot::from((
                person.clone(),
                profile_picture.clone(),
                stats.clone(),
                current_tick,
            ));
            vacant.insert(person);
        }
    };
    info!("{}", app_state.persons.len());
    cmd.remove_component::<Dirty>(*entity);
}
