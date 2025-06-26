use std::sync::Arc;

use dashmap::DashMap;
use legion::{Entity, Resources};

use crate::{
    integrations::{
        snapshots::{company::CompanySnapshot, snapshots::SnapshotState},
        snapshots_emitter::snapshots_emitter::{
            ExportFrequency, SnapshotCollectionEmitter, SnapshotEmitRegistry,
            SnapshotEmitterConfig, SnapshotFieldEmitter,
        },
    },
    sim::{
        new_game::new_game::{CompanyPreset, StartingEmployeesConfig},
        person::{components::PersonId, skills::SkillId},
        registries::registry::{Registry}
        ,
        systems::global::UsedProfilePictureRegistry,
        team::components::TeamId,
    },
};
use crate::db::init::SaveSlot;
use crate::integrations::snapshots_emitter::snapshots_emitter::SnapshotEvent;

pub fn initialize_non_shared_resources(resources: &mut Resources) {
    resources.insert(UsedProfilePictureRegistry::default());
    
    //registries
    resources.insert(Arc::new(Registry::<PersonId, Entity>::with_name(
        "Person registry",
    )));
    
    resources.insert(Arc::new(Registry::<TeamId, Entity>::with_name(
        "Team registry",
    )));
    
    // resources.insert(Arc::new(DashMap::<&'static str, u64>::new()));//last update map
    resources.insert(SaveSlot::default());
}

pub fn initialize_emit_registries(
    main_snapshot_state: &Arc<SnapshotState>,
) -> SnapshotEmitRegistry {
    //Snapshot registry
    let mut snapshot_registry = SnapshotEmitRegistry::new();
    let game_speed_snapshots_emitter = SnapshotFieldEmitter {
        field: Arc::clone(&main_snapshot_state.game_speed), 
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: SnapshotEvent::GameSpeed,
            last_sent_tick: Default::default(),
        },
    };

    let person_snapshots_emitter = SnapshotCollectionEmitter {
        map: Arc::clone(&main_snapshot_state.persons),
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: SnapshotEvent::Persons,
            last_sent_tick: Default::default(),
        },
    };

    let debug_display_snapshots_emitter = SnapshotCollectionEmitter {
        map: Arc::clone(&main_snapshot_state.debug_display),
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: SnapshotEvent::DebugDisplay,
            last_sent_tick: Default::default(),
        },
    };
    let team_snapshots_emitter = SnapshotCollectionEmitter {
        map: Arc::clone(&main_snapshot_state.teams),
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: SnapshotEvent::Teams,
            last_sent_tick: Default::default(),
        },
    };
    let company_snapshots_emitter: SnapshotFieldEmitter<CompanySnapshot> = SnapshotFieldEmitter {
        field: Arc::clone(&main_snapshot_state.company),
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: SnapshotEvent::Company,
            last_sent_tick: Default::default(),
        },
    };

    let stress_snapshots_emitter = SnapshotCollectionEmitter {
        map: Arc::clone(&main_snapshot_state.stress_level),
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: SnapshotEvent::Stress,
            last_sent_tick: Default::default(),
        },
    };

    let stress_history_snapshots_emitter = SnapshotCollectionEmitter {
        map: Arc::clone(&main_snapshot_state.stress_history),
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryNTicks(4),
            event_name: SnapshotEvent::StressHistory,
            last_sent_tick: Default::default(),
        },
    };
    snapshot_registry.register(company_snapshots_emitter);
    snapshot_registry.register(game_speed_snapshots_emitter);
    snapshot_registry.register(person_snapshots_emitter);
    snapshot_registry.register(team_snapshots_emitter);
    snapshot_registry.register(debug_display_snapshots_emitter);
    snapshot_registry.register(stress_snapshots_emitter);
    snapshot_registry.register(stress_history_snapshots_emitter);


    snapshot_registry
}
