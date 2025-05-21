use crate::integrations::queues::{QueueManager, SimCommand, UICommandQueues};
use crate::integrations::system_queues::shared::timed_dispatch;

use crate::integrations::snapshots::SnapshotState;
use crate::integrations::system_queues::game_speed_manager::GameSpeedManagerCommand::{DecreaseGameSpeed, IncreaseGameSpeed, SetGameSpeed};

use crate::sim::resources::global::SimManager;
use crossbeam::queue::SegQueue;
use legion::system;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tauri::utils::assets::phf::Set;
use tauri::State;
use tracing::{debug, info, trace, warn};
use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};

pub enum GameSpeedManagerCommand {
    //Game speed settings related.
    SetGameSpeed(GameSpeed),
    IncreaseGameSpeed,
    DecreaseGameSpeed,
    PauseGame,
    ResumeGame,
}
#[tauri::command]
pub fn set_game_speed(queues: State<'_, Arc<UICommandQueues>>, game_speed: GameSpeed){
    queues.runtime.push(SimCommand::GameSpeed(SetGameSpeed(game_speed)));

}

#[tauri::command]
pub fn increase_speed(queues: State<'_, Arc<UICommandQueues>>){
    queues.runtime.push(SimCommand::GameSpeed(IncreaseGameSpeed));
}
#[tauri::command]
pub fn decrease_speed(queues: State<'_, Arc<UICommandQueues>>){
    queues.runtime.push(SimCommand::GameSpeed(DecreaseGameSpeed));
}



#[system]
pub fn handle_game_speed_manager_queue(
    #[resource] queue_manager: &QueueManager,
    #[resource] game_speed_manager: &mut Arc<GameSpeedManager>,
) {
    let type_name = std::any::type_name::<&Arc<RwLock<GameSpeedManager>>>();
    println!("System expects: {}", type_name);

    trace!("Handling game speed manager queue");
    let queue = &queue_manager.game_speed_manager;
    let dispatch_time_limit = Duration::from_millis(5);

    timed_dispatch(queue, dispatch_time_limit, |cmd| match cmd {
        GameSpeedManagerCommand::IncreaseGameSpeed => {
            info!("Increase game speed");
            game_speed_manager.increase();
        }
        GameSpeedManagerCommand::SetGameSpeed(speed) => {
            game_speed_manager.set(speed);
        }
        GameSpeedManagerCommand::DecreaseGameSpeed => {
            game_speed_manager.decrease();
        }
        GameSpeedManagerCommand::PauseGame => {
            game_speed_manager.set(GameSpeed::Stopped);
        }
        GameSpeedManagerCommand::ResumeGame => {
            game_speed_manager.set(GameSpeed::Normal);

        }
    })
}
