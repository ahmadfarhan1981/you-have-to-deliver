use crate::integrations::snapshots::company::CompanySnapshot;
use crate::integrations::snapshots::debug_display::DebugDisplayEntrySnapshot;
use crate::integrations::snapshots::person::PersonSnapshot;
use crate::integrations::snapshots::skills::SkillSetSnapshot;
use crate::integrations::snapshots::snapshots::SnapshotState;
use crate::integrations::snapshots::stress::StressSnapshot;
use crate::integrations::snapshots::stress_history::StressHistorySnapshot;
use crate::integrations::snapshots::working_hours::WorkingHoursSnapshot;
use crate::integrations::snapshots::team::TeamSnapshot;
use crate::integrations::snapshots_emitter::snapshots_emitter::{
    SnapshotEmitRegistry, SnapshotEvent, SnapshotFieldEmitter,
};
use crate::sim::action::action::ActionIntent;
use crate::sim::company::company::{Company, PlayerControlled};
use crate::sim::game_speed::components::GameSpeedManager;
use crate::sim::person::components::{Person, PersonId, ProfilePicture};
use crate::sim::person::morale::StressLevel;
use crate::sim::person::needs::{Energy, Hunger};
use crate::sim::person::personality_matrix::PersonalityMatrix;
use crate::sim::person::skills::{SkillId, SkillSet};
use crate::sim::person::spawner::spawn_person;
use crate::sim::person::stats::Stats;
use crate::sim::resources::global::{Dirty, TickCounter};
use crate::sim::team::components::Team;
use crate::sim::utils::debugging::DebugDisplayComponent;
use crate::sim::utils::snapshots::replace_if_changed;
use arc_swap::ArcSwap;
use dashmap::{DashMap, Entry};
use legion::systems::CommandBuffer;
use legion::{system, Entity};
use parking_lot::RwLock;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::Arc;
use tracing::field::debug;
use tracing::{debug, info, warn};
use tracing_subscriber::registry;
use crate::sim::calendar::availability::{MonthlyAvailability, YearMonth, AvailabilityBitSet};

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
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] app_state: &Arc<SnapshotState>,
    #[resource] emit_registry: &Arc<SnapshotEmitRegistry>,
    entity: &Entity,
    company: &Company,                         // Live company data from ECS
    _player_controlled_tag: &PlayerControlled, // used for query filtering
    _dirty: &Dirty,
    cmd: &mut CommandBuffer,
) {
    info!("Company '{:?}' snapshot updated.", company);
    // Use this method as the reference implementation for all push_x_to_integration methods

    // 1. Load the current Arc<CompanySnapshot>. This is an Arc pointing to the snapshot data.
    let current_arc_snapshot = app_state.company.value.load_full();

    // 2. Clone the data *inside* the Arc to get a mutable CompanySnapshot.
    //    This `mutable_snapshot_data` will be compared and potentially updated by `replace_if_changed`.
    let mut mutable_snapshot_data = (**current_arc_snapshot).clone();
    info!(
        "{:?} -- Snaphot: {:?} data: {:?}",
        SnapshotEvent::Company.as_str(),
        mutable_snapshot_data,
        company
    );
    // 3. Call `replace_if_changed`.
    //    - `&mut mutable_snapshot_data`: the snapshot to potentially update.
    //    - `company.clone()`: creates an owned `Company` instance from the `&Company` reference.
    //      This is necessary because `replace_if_changed` takes its `new_source_value` by value.
    //      `CompanySnapshot` has `From<Company>`.
    let changed =
        replace_if_changed::<CompanySnapshot, Company>(&mut mutable_snapshot_data, &company);

    if changed {
        // If `changed` is true, `mutable_snapshot_data` now holds the new, updated snapshot.
        // Store this new snapshot into the ArcSwap.
        // `ArcSwap::store` expects an `Arc<T>`, T in our case is always an Arc of the data. So it's another `Arc::new()`
        app_state
            .company
            .value
            .store(Arc::new(Arc::new(mutable_snapshot_data)));
        emit_registry.mark_data_updated(SnapshotEvent::Company, tick_counter.value());
        debug!("Company '{}' snapshot updated.", company.name);
    } else {
        debug!("Company '{}' snapshot unchanged.", company.name);
    }
    cmd.remove_component::<Dirty>(*entity);
}

#[system(for_each)]
pub fn tick_needs(energy: &mut Energy, hunger: &mut Hunger, stats: &Stats) {
    energy.tick(stats);
    hunger.tick(stats);
}
#[system(for_each)]
pub fn push_needs_to_integration(
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] app_state: &Arc<SnapshotState>,
    person: &Person,
    energy: &Energy,
    hunger: &Hunger,
) {
    let registry = &app_state.persons;
    match registry.entry(person.person_id) {
        Entry::Occupied(mut existing) => {
            let existing_person = existing.get_mut();

            replace_if_changed(&mut existing_person.energy, energy);
            replace_if_changed(&mut existing_person.hunger, hunger);
        }
        Entry::Vacant(vacant) => {
            warn!(
                "Unexpected. No entry in person registry for {}",
                person.name
            );
        }
    };
}

#[system(for_each)]
pub fn push_intents_and_goals_to_integration(
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] app_state: &Arc<SnapshotState>,
    entity: &Entity,
    person: &Person,
    action: Option<&ActionIntent>,
) {
    let registry = &app_state.persons;
    match registry.entry(person.person_id) {
        Entry::Occupied(mut existing) => match action {
            None => {}
            Some(_) => {}
        },
        Entry::Vacant(vacant) => {
            warn!(
                "Unexpected. No entry in person registry for {}",
                person.name
            );
        }
    };
}

#[system(for_each)]
pub fn push_persons_to_integration(
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] app_state: &Arc<SnapshotState>,
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

    let skillset_snapshot = SkillSetSnapshot::from(skill_set);
    match registry.entry(person.person_id) {
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
            if existing_person.team != person.team.map(|id| id.0) {
                changed = true;
                existing_person.team = person.team.map(|id| id.0)
            }

            changed |= replace_if_changed(&mut existing_person.stats, stats);
            changed |= replace_if_changed(&mut existing_person.profile_picture, profile_picture);
            changed |= replace_if_changed(&mut existing_person.assigned_skill, &skillset_snapshot);
            if changed {
                existing_person.updated = current_tick;
            }
        }
        Entry::Vacant(vacant) => {
            let person = PersonSnapshot::from((
                person,
                profile_picture,
                stats,
                personality,
                skill_set,
                current_tick,
            ));
            vacant.insert(person);
        }
    };
    cmd.remove_component::<Dirty>(*entity);
}

#[system(for_each)]
pub fn push_teams_to_integration(
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] app_state: &Arc<SnapshotState>,
    #[resource] emit_registry: &Arc<SnapshotEmitRegistry>,
    entity: &Entity,
    team: &Team,
    _dirty: &Dirty,
    cmd: &mut CommandBuffer,
) {
    let current_tick = tick_counter.value();
    let team_snapshots = &app_state.teams;

    info!("Team '{}' snapshot updated.", team.name);
    match team_snapshots.entry(team.team_id) {
        Entry::Occupied(mut existing) => {
            let mut existing_team = existing.get_mut();
            let mut changed = false;

            changed |= replace_if_changed::<TeamSnapshot, Team>(&mut existing_team, team);
            if changed {
                info!("Existing team '{}' snapshot changed. Updating..", team.name);
                // existing_team.updated = current_tick;
                emit_registry.mark_data_updated(SnapshotEvent::Teams, current_tick);
            }
        }
        Entry::Vacant(vacant) => {
            info!("New teamL {}. Updating..", team.name);
            let team = TeamSnapshot::from(team);
            vacant.insert(team);
            emit_registry.mark_data_updated(SnapshotEvent::Teams, current_tick);
        }
    };
    cmd.remove_component::<Dirty>(*entity);
}

#[system(for_each)]
pub fn push_debug_displays_to_integration(
    #[resource] app_state: &Arc<SnapshotState>,
    person: &Person,
    debug_display_component: &DebugDisplayComponent,
) {
    // let current_tick = tick_counter.value();
    let debug_display_registry = &app_state.debug_display;

    let mut entries: Vec<DebugDisplayEntrySnapshot> = debug_display_component
        .entries
        .iter()
        .map(|(key, val)| DebugDisplayEntrySnapshot::new(key.clone(), val.clone()))
        .collect();
    entries.sort_by(|a, b| a.label.cmp(&b.label));
    // info!("{:?}", entries);

    match debug_display_registry.entry(person.person_id) {
        Entry::Occupied(mut existing) => {
            let existing_debug_displays = existing.get_mut();
            *existing_debug_displays = entries;
        }
        Entry::Vacant(vacant) => {
            vacant.insert(entries);
        }
    };

    // print_all_debug_displays(&app_state.debug_display);
}



pub fn print_all_debug_displays(registry: &DashMap<PersonId, Vec<DebugDisplayEntrySnapshot>>) {
    if registry.is_empty() {
        info!("Debug display registry is empty.");
        return;
    }

    info!("--- All Debug Displays ---");
    for entry in registry.iter() {
        let person_id = entry.key();
        let display_entries = entry.value();

        info!("Person ID: {}", person_id.0);
        if display_entries.is_empty() {
            info!("  No debug entries for this person.");
        } else {
            for debug_entry in display_entries.iter() {
                info!("  - {}: {}", debug_entry.label, debug_entry.value);
            }
        }
    }
    info!("--- End of Debug Displays ---");
}
#[system(for_each)]
pub fn push_stress_level_to_integration(
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] app_state: &Arc<SnapshotState>,
    person: &Person,
    stress_level: &StressLevel,
) {
    // TODO dirty check
    let current_tick = tick_counter.value();
    let person_id = person.person_id;

    let stress_level_snapshots = &app_state.stress_level;

    match stress_level_snapshots.entry(person_id) {
        Entry::Occupied(mut existing) => {
            let mut existing_snapshot = existing.get_mut();
            if *existing_snapshot != stress_level {
                *existing_snapshot = StressSnapshot {
                    person_id: person_id.0,
                    ..StressSnapshot::from(stress_level)
                };
            }
        }
        Entry::Vacant(vacant) => {
            let s = StressSnapshot {
                person_id: person_id.0,
                ..StressSnapshot::from(stress_level)
            };
            vacant.insert(s);
        }
    };
}

#[system(for_each)]
pub fn push_stress_history_to_integration(
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] app_state: &Arc<SnapshotState>,
    #[resource] emit_registry: &Arc<SnapshotEmitRegistry>,
    person: &Person,
    stress_level: &StressLevel,
) {
    if tick_counter.current_date().quarter_tick != 1 {
        return;
    }

    // TODO dirty check
    let current_tick = tick_counter.value();
    let person_id = person.person_id;

    let stress_history_snapshots = &app_state.stress_history;

    match stress_history_snapshots.entry(person_id) {
        Entry::Occupied(mut existing) => {
            let mut existing_snapshot = existing.get_mut();
            if *existing_snapshot != stress_level {
                *existing_snapshot = StressHistorySnapshot {
                    person_id: person_id.0,
                    ..StressHistorySnapshot::from(stress_level)
                };
            }
        }
        Entry::Vacant(vacant) => {
            let s = StressHistorySnapshot {
                person_id: person_id.0,
                ..StressHistorySnapshot::from(stress_level)
            };
            vacant.insert(s);
        }
    };
    emit_registry.mark_data_updated(SnapshotEvent::StressHistory, current_tick);
}

#[system(for_each)]
pub fn push_working_hours_to_integration(
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] app_state: &Arc<SnapshotState>,
    #[resource] emit_registry: &Arc<SnapshotEmitRegistry>,
    person: &Person,
    availability: &MonthlyAvailability,
) {
    let current_month = YearMonth::from(&tick_counter.current_date());
    if let Some(snapshot) = WorkingHoursSnapshot::from_month(person.person_id.0, availability, current_month) {
        let current_tick = tick_counter.value();
        let map = &app_state.working_hours;
        let changed = match map.entry(person.person_id) {
            Entry::Occupied(mut occ) => {
                if *occ.get() != snapshot {
                    *occ.get_mut() = snapshot;
                    true
                } else { false }
            }
            Entry::Vacant(v) => { v.insert(snapshot); true }
        };

        if changed {
            emit_registry.mark_data_updated(SnapshotEvent::MonthlyAvailability, current_tick);
        }
    }
}
