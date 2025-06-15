use std::{ sync::{atomic::AtomicBool, Arc}};

use dashmap::DashMap;
use legion::{Entity, Resources};
use parking_lot::RwLock;

use crate::{integrations::{queues::{QueueManager, UICommandQueues}, snapshots::{company::CompanySnapshot, snapshots::SnapshotState}, snapshots_emitter::snapshots_emitter::{ExportFrequency, SnapshotCollectionEmitter, SnapshotEmitRegistry, SnapshotEmitterConfig, SnapshotFieldEmitter}}, sim::{game_speed::components::GameSpeedManager, new_game::new_game::{CompanyPreset, StartingEmployeesConfig}, person::{components::PersonId, skills::SkillId}, registries::registry::{GlobalSkillNameMap, Registry}, resources::global::TickCounter, systems::global::UsedProfilePictureRegistry, team::components::TeamId}};

pub fn initialize_non_shared_resources(resources: &mut Resources){
    let used_portrait = UsedProfilePictureRegistry::default();
    resources.insert(used_portrait);
    resources.insert(Arc::<GlobalSkillNameMap>::new(GlobalSkillNameMap::default()));
   //registries
                resources.insert(Arc::new(Registry::<PersonId, Entity>::with_name(
                    "Person registry",
                )));

                resources.insert(Arc::new(Registry::<SkillId, Entity>::with_name(
                    "Skill registry",
                )));

                resources.insert(Arc::new(Registry::<TeamId, Entity>::with_name(
                    "Team registry",
                )));

resources.insert(CompanyPreset::default());
                resources.insert(StartingEmployeesConfig::default());

            
            
                let data_last_update_map: DashMap<&'static str, u64> = DashMap::new();
                let data_last_update = Arc::new(data_last_update_map);
                resources.insert(data_last_update);
            
            }



pub fn initialize_emit_registries(main_snapshot_state:&Arc<SnapshotState>)->SnapshotEmitRegistry{
    
    
    //Snapshot registry
    let mut snapshot_registry = SnapshotEmitRegistry::new();
    let game_speed_snapshots_emitter = SnapshotFieldEmitter {
        field: main_snapshot_state.game_speed.clone(), // Clones the Arc<SnapshotField>, sharing the instance
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: "game_speed_snapshot",
            last_sent_tick: Default::default(),
        },
    };

    let person_snapshots_emitter = SnapshotCollectionEmitter {
        map: Arc::clone(&main_snapshot_state.persons),
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: "persons_snapshot",
            last_sent_tick: Default::default(),
        },
    };

    let debug_display_snapshots_emitter = SnapshotCollectionEmitter {
        map: Arc::clone(&main_snapshot_state.debug_display),
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: "debug_display_snapshot",
            last_sent_tick: Default::default(),
        },
    };
    let team_snapshots_emitter = SnapshotCollectionEmitter {
        map: Arc::clone(&main_snapshot_state.teams),
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: "teams_snapshot",
            last_sent_tick: Default::default(),
        },
    };
    let company_snapshots_emitter: SnapshotFieldEmitter<CompanySnapshot> = SnapshotFieldEmitter {
        field: main_snapshot_state.company.clone(), 
        config: SnapshotEmitterConfig {
            frequency: ExportFrequency::EveryTick,
            event_name: "company_snapshot",
            last_sent_tick: Default::default(),
        },
    };
    snapshot_registry.register(company_snapshots_emitter);
    snapshot_registry.register(game_speed_snapshots_emitter);
    snapshot_registry.register(person_snapshots_emitter);
    snapshot_registry.register(team_snapshots_emitter);
    snapshot_registry.register(debug_display_snapshots_emitter);

    snapshot_registry
}