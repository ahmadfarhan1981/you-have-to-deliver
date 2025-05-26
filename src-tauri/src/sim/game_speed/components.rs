use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GameSpeed {
    Stopped = 1,
    Slow = 2,
    #[default]
    Normal = 3,
    Fast = 4,
    Max = 5,
}
impl From<u8> for GameSpeed {
    fn from(value: u8) -> Self {
        match value {
            2 => GameSpeed::Slow,
            3 => GameSpeed::Normal,
            4 => GameSpeed::Fast,
            5 => GameSpeed::Max,
            _ => GameSpeed::Stopped,
        }
    }
}

impl From<GameSpeed> for u8 {
    fn from(speed: GameSpeed) -> u8 {
        speed as u8
    }
}

impl GameSpeed {
    pub fn interval(&self) -> Option<Duration> {
        match self {
            GameSpeed::Stopped => None,
            GameSpeed::Slow => Some(Duration::from_millis(1000)),
            GameSpeed::Normal => Some(Duration::from_millis(500)),
            GameSpeed::Fast => Some(Duration::from_millis(200)),
            GameSpeed::Max => Some(Duration::from_millis(30)),
        }
    }

    pub fn increase(self) -> Self {
        use GameSpeed::*;
        match self {
            Stopped => Slow,
            Slow => Normal,
            Normal => Fast,
            Fast => Max,
            Max => Max,
        }
    }

    pub fn decrease(self) -> Self {
        use GameSpeed::*;
        match self {
            Max => Fast,
            Fast => Normal,
            Normal => Slow,
            Slow => Stopped,
            Stopped => Stopped,
        }
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, GameSpeed::Stopped)
    }
}

#[derive(Debug)]
pub struct GameSpeedManager {
    pub game_speed: GameSpeed,
}

impl GameSpeedManager {
    pub fn set(&mut self, game_speed: GameSpeed) {
        self.game_speed = game_speed;
    }

    pub fn current_interval(&self) -> Option<Duration> {
        self.game_speed.interval()
    }

    pub fn is_paused(&self) -> bool {
        self.game_speed.is_paused()
    }

    pub fn increase(&mut self) {
        self.game_speed = self.game_speed.increase();
    }

    pub fn decrease(&mut self) {
        self.game_speed = self.game_speed.decrease();
    }
}
