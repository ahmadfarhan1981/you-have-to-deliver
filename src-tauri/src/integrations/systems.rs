use crate::integrations::snapshots::{AssignedSkillSnapshot, CompanySnapshot, PersonSnapshot, ProfilePictureSnapshot, SnapshotState, StatsSnapshot};
use crate::sim::game_speed::components::GameSpeedManager;
use crate::sim::person::components::{Person, PersonId, ProfilePicture};
use crate::sim::person::personality_matrix::PersonalityMatrix;
use crate::sim::person::skills::{SkillId, SkillSet};
use crate::sim::person::spawner::spawn_person;
use crate::sim::person::stats::Stats;
use crate::sim::registries::registry::GlobalSkillNameMap;
use crate::sim::resources::global::{Dirty, TickCounter};
use crate::sim::utils::snapshots::{replace_if_changed};
use dashmap::Entry;
use legion::systems::CommandBuffer;
use legion::{system, Entity};
use parking_lot::RwLock;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::mpsc::channel;
use arc_swap::ArcSwap;
use tracing::info;
use tracing_subscriber::registry;
use crate::integrations::snapshots_emitter::snapshots_emitter::{SnapshotEmitRegistry, SnapshotFieldEmitter};
use crate::sim::company::company::{Company, PlayerControlled};
use super::snapshots::SkillSetSnapshot;

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
pub fn push_company_to_integration(
    // #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] app_state: &Arc<SnapshotState>,
    entity: &Entity,
    company: &Company, // Live company data from ECS
    _player_controlled_tag: &PlayerControlled, // used for query filtering
    _dirty: &Dirty,
    cmd: &mut CommandBuffer,
) {
    // Use this method as the reference implementaion for all push_x_to_integration methods

    // 1. Load the current Arc<CompanySnapshot>. This is an Arc pointing to the snapshot data.
    let current_arc_snapshot = app_state.company.value.load_full();

    // 2. Clone the data *inside* the Arc to get a mutable CompanySnapshot.
    //    This `mutable_snapshot_data` will be compared and potentially updated by `replace_if_changed`.
    let mut mutable_snapshot_data = (**current_arc_snapshot).clone();

    // 3. Call `replace_if_changed`.
    //    - `&mut mutable_snapshot_data`: the snapshot to potentially update.
    //    - `company.clone()`: creates an owned `Company` instance from the `&Company` reference.
    //      This is necessary because `replace_if_changed` takes its `new_source_value` by value.
    //      `CompanySnapshot` has `From<Company>`.
    let changed = replace_if_changed::<CompanySnapshot, Company>(&mut mutable_snapshot_data, company.clone());

    if changed {
        // If `changed` is true, `mutable_snapshot_data` now holds the new, updated snapshot.
        // Store this new snapshot into the ArcSwap.
        // `ArcSwap::store` expects an `Arc<T>`, T in our case is always an Arc of the data. So it's another `Arc::new()`
        app_state.company.value.store(Arc::new(Arc::new(mutable_snapshot_data)));
        debug!("Company '{}' snapshot updated.", company.name);
    } else {
        debug!("Company '{}' snapshot unchanged.", company.name);
    }
    cmd.remove_component::<Dirty>(*entity);
}


#[system(for_each)]
pub fn push_persons_to_integration(
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] app_state: &Arc<SnapshotState>,
    #[resource] global_skill_name_map: &Arc<GlobalSkillNameMap>,
    entity: &Entity,
    person: &Person,
    stats: &Stats,
    profile_picture: &ProfilePicture,
    personality: &PersonalityMatrix,
    skill_set: &SkillSet,
    _dirty: &Dirty,
    cmd: &mut CommandBuffer,
) {
    let current_tick = tick_counter.value();
    let registry = &app_state.persons;
    
    let skillset_snapshot = SkillSetSnapshot::from_sim(skill_set, global_skill_name_map);

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
            changed |= replace_if_changed(&mut existing_person.assigned_skill, skillset_snapshot);
            if changed {
                existing_person.updated = current_tick;
            }
        }
        Entry::Vacant(vacant) => {
            let person = PersonSnapshot::from((
                person.clone(),
                profile_picture.clone(),
                stats.clone(),
                personality.clone(),
                (skill_set, &**global_skill_name_map),
                current_tick,
            ));
            vacant.insert(person);
        }
    };
    cmd.remove_component::<Dirty>(*entity);
}
