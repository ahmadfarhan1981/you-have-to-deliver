use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize,Deserialize, Default)]
pub enum GameSpeed {
    Stopped,
    Slow,
    #[default]
    Normal,
    Fast,
    Max,
}

impl GameSpeed {
    pub fn interval(&self) -> Option<Duration> {
        match self {
            GameSpeed::Stopped => None,
            GameSpeed::Slow => Some(Duration::from_millis(1000)),
            GameSpeed::Normal => Some(Duration::from_millis(500)),
            GameSpeed::Fast => Some(Duration::from_millis(200)),
            GameSpeed::Max => Some(Duration::from_millis(50)),
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
