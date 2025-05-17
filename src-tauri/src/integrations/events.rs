use crate::sim::game_speed::components::{GameSpeed, GameSpeedSettings};
use crossbeam::queue::SegQueue;
use legion::system;
use std::sync::{Arc, RwLock};

pub enum SimCommand {
    //Game speed settings related.
    SetGameSpeed(GameSpeed),
    IncreaseGameSpeed(),
    DecreaseGameSpeed(),
    PauseGame(),
    ResumeGame(),
}

#[system]
pub fn handle_commands(
    #[resource] queue: &Arc<SegQueue<SimCommand>>,
    #[resource] game_speed: &Arc<RwLock<GameSpeedSettings>>,
) {
    while let Some(command) = queue.pop() {
        match command {
            SimCommand::SetGameSpeed(speed) => {
                if let Ok(mut settings) = game_speed.write() {
                    settings.game_speed = speed;
                }
            }
            SimCommand::IncreaseGameSpeed() => {
                if let Ok(mut settings) = game_speed.write() {
                    settings.increase();
                }
            }
            SimCommand::DecreaseGameSpeed() => {
                if let Ok(mut settings) = game_speed.write() {
                    settings.decrease();
                }
            }
            SimCommand::PauseGame() => {
                if let Ok(mut settings) = game_speed.write() {
                    settings.game_speed = GameSpeed::Stopped;
                }
            }
            SimCommand::ResumeGame() => {
                if let Ok(mut settings) = game_speed.write() {
                    settings.game_speed = GameSpeed::Normal;
                }
            }
        }
    }
}
