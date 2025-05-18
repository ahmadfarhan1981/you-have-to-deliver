use crate::sim::game_speed::components::{GameSpeed, GameSpeedManager};
use crossbeam::queue::SegQueue;
use legion::system;
use std::sync::{Arc, RwLock};
use crate::sim::resources::global::SimManager;

pub enum SimCommand {
    //Game speed settings related.
    SetGameSpeed(GameSpeed),
    IncreaseGameSpeed(),
    DecreaseGameSpeed(),
    PauseGame(),
    ResumeGame(),

    //Sim manager commands
    StartSim(),
    StopSim(),
}

#[system]
pub fn handle_commands(
    #[resource] queue: &Arc<SegQueue<SimCommand>>,
    #[resource] game_speed_manager: &Arc<RwLock<GameSpeedManager>>,
    #[resource] sim_state: &Arc<SimManager>
) {
    while let Some(command) = queue.pop() {
        match command {
            SimCommand::SetGameSpeed(speed) => {
                if let Ok(mut settings) = game_speed_manager.write() {
                    settings.game_speed = speed;
                }
            }
            SimCommand::IncreaseGameSpeed() => {
                if let Ok(mut settings) = game_speed_manager.write() {
                    settings.increase();
                }
            }
            SimCommand::DecreaseGameSpeed() => {
                if let Ok(mut settings) = game_speed_manager.write() {
                    settings.decrease();
                }
            }
            SimCommand::PauseGame() => {
                if let Ok(mut settings) = game_speed_manager.write() {
                    settings.game_speed = GameSpeed::Stopped;
                }
            }
            SimCommand::ResumeGame() => {
                if let Ok(mut settings) = game_speed_manager.write() {
                    settings.game_speed = GameSpeed::Normal;
                }
            }
            SimCommand::StartSim() => {sim_state.resume_sim()}
            SimCommand::StopSim() => {sim_state.pause_sim()}
        }
    }
}
