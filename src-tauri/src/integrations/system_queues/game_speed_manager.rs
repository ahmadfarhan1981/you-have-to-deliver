use crate::integrations::queues::{QueueManager, SimCommand, UICommandQueues};
use crate::integrations::system_queues::shared::timed_dispatch;
use std::ptr::write;
use std::sync::Arc;

use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crate::sim::resources::global::SimManager;
use crossbeam::queue::SegQueue;
use legion::system;

use crate::integrations::system_queues::game_speed_manager::GameSpeedManagerCommand::{
    DecreaseGameSpeed, IncreaseGameSpeed, SetGameSpeed,
};
use parking_lot::RwLock;
use std::time::{Duration, Instant};
use tauri::utils::assets::phf::Set;
use tauri::State;
use tracing::{debug, info, trace, warn};

#[derive(Default)]
pub enum GameSpeedManagerCommand {
    //Game speed settings related.
    SetGameSpeed(GameSpeed),
    #[default]
    IncreaseGameSpeed,
    DecreaseGameSpeed,
    PauseGame,
    ResumeGame,
}
#[tauri::command]
pub fn set_game_speed(queues: State<'_, Arc<UICommandQueues>>, speed_number: u8) {
    let game_speed = GameSpeed::from(speed_number);
    queues
        .runtime
        .push(SimCommand::GameSpeed(SetGameSpeed(game_speed)));
}

#[tauri::command]
pub fn increase_speed(queues: State<'_, Arc<UICommandQueues>>) {
    queues
        .runtime
        .push(SimCommand::GameSpeed(IncreaseGameSpeed));
}
#[tauri::command]
pub fn decrease_speed(queues: State<'_, Arc<UICommandQueues>>) {
    queues
        .runtime
        .push(SimCommand::GameSpeed(DecreaseGameSpeed));
}

#[system]
pub fn handle_game_speed_manager_queue(
    #[resource] queue_manager: &QueueManager,
    #[resource] game_speed_manager: &Arc<RwLock<GameSpeedManager>>,
) {
    trace!("Handling game speed manager queue");
    let queue = &queue_manager.game_speed_manager;
    let dispatch_time_limit = Duration::from_millis(5);

    timed_dispatch(queue, dispatch_time_limit, |cmd| match cmd {
        GameSpeedManagerCommand::IncreaseGameSpeed => {
            game_speed_manager.write().increase();
        }
        GameSpeedManagerCommand::SetGameSpeed(speed) => {
            game_speed_manager.write().set(speed);
        }
        GameSpeedManagerCommand::DecreaseGameSpeed => {
            game_speed_manager.write().decrease();
        }
        GameSpeedManagerCommand::PauseGame => {
            game_speed_manager.write().set(GameSpeed::Stopped);
        }
        GameSpeedManagerCommand::ResumeGame => {
            game_speed_manager.write().set(GameSpeed::Normal);
        }
    })
}
