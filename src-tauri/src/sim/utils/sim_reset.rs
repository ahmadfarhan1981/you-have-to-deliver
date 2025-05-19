use std::sync::Arc;
use legion::{Resources, World};
use tauri::utils::resources;
use tracing::{debug, info};
use crate::integrations::ui::SnapshotState;
use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crate::sim::person::registry::PersonRegistry;
use crate::sim::resources::global::{AssetBasePath, TickCounter};
use crate::sim::systems::global::UsedProfilePictureRegistry;

pub fn reset_simulation(world: &mut World, resources: &mut Resources) {
    info!("Resetting simulation");


    debug!("Clearing entities...");
    world.clear();
    debug!("Clearing resources...");
    /// Special cases:
    ///   - sim manager: preserve
    ///   - snapshot state
    ///   - command queues, is under snapshot state, clear the queu
    ///   - game speed, set to paused
    ///   - asset base path: set once on app load, this should never change, TODO move to coresettings
    ///

    debug!("Clearing command queue...");
    {
        let snapshot_state = resources.get::<Arc<SnapshotState>>().expect("Cant get SnapshotState");
        snapshot_state.command_queue.clear();
    }
    debug!("Resetting ticker counter...");
    {
        let tick_counter = resources.get::<TickCounter>().expect("Cant get tick counter");
        tick_counter.reset();
    }
    debug!("Resetting game speed...");
    {
        let mut game_speed_manager = resources.get_mut::<GameSpeedManager>().expect("Cant get GameSpeedManager");
        game_speed_manager.set(GameSpeed::Stopped );
    }
    debug!("Resetting registries...");
    resources.insert(UsedProfilePictureRegistry::default());
    resources.insert(Arc::new(PersonRegistry::new()));


}