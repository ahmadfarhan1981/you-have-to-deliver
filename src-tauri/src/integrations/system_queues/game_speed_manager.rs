use crate::integrations::queues::{ QueueManager, SimCommand};
use crate::integrations::system_queues::shared::timed_dispatch;
use crate::integrations::ui::SnapshotState;
use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crate::sim::resources::global::SimManager;
use crossbeam::queue::SegQueue;
use legion::system;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tauri::utils::assets::phf::Set;
use tauri::State;
use tracing::{debug, warn};
use crate::integrations::system_queues::game_speed_manager::GameSpeedManagerCommand::{DecreaseGameSpeed, IncreaseGameSpeed, SetGameSpeed};

pub enum GameSpeedManagerCommand {
    //Game speed settings related.
    SetGameSpeed(GameSpeed),
    IncreaseGameSpeed,
    DecreaseGameSpeed,
    PauseGame,
    ResumeGame,
}
#[tauri::command]
pub fn set_game_speed(state: State<'_, Arc<SnapshotState>>, game_speed: GameSpeed){
    state.command_queue.push(SimCommand::GameSpeed(SetGameSpeed(game_speed)));

}

#[tauri::command]
pub fn increase_speed(state: State<'_, Arc<SnapshotState>>){
    state.command_queue.push(SimCommand::GameSpeed(IncreaseGameSpeed));
}
#[tauri::command]
pub fn decrease_speed(state: State<'_, Arc<SnapshotState>>){
    state.command_queue.push(SimCommand::GameSpeed(DecreaseGameSpeed));
}



#[system]
pub fn handle_game_speed_manager_queue(
    #[resource] queue_manager: &QueueManager,
    #[resource] game_speed_manager: &Arc<RwLock<GameSpeedManager>>,
) {
    debug!("Handling game speed manager queue");
    let queue = &queue_manager.game_speed_manager;
    let dispatch_time_limit = Duration::from_millis(5);

    timed_dispatch(queue, dispatch_time_limit, |cmd| match cmd {
        GameSpeedManagerCommand::IncreaseGameSpeed => {
            if let Ok(mut settings) = game_speed_manager.write() {
                settings.increase();
            }
        }
        GameSpeedManagerCommand::SetGameSpeed(speed) => {
            if let Ok(mut settings) = game_speed_manager.write() {
                settings.set(speed);
            }
        }
        GameSpeedManagerCommand::DecreaseGameSpeed => {
            if let Ok(mut settings) = game_speed_manager.write() {
                settings.decrease();
            }
        }
        GameSpeedManagerCommand::PauseGame => {
            if let Ok(mut settings) = game_speed_manager.write() {
                settings.set(GameSpeed::Stopped);
            }
        }
        GameSpeedManagerCommand::ResumeGame => {
            if let Ok(mut settings) = game_speed_manager.write() {
                settings.set(GameSpeed::Normal);
            }
        }
    })
}
