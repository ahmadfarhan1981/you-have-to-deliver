use crate::integrations::snapshots::tick::TickSnapshot;
use serde::Serialize;
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Default, Serialize)]
pub struct GameSpeedSnapshot {
    pub tick: TickSnapshot,
    pub game_speed: AtomicU8,
}

impl Clone for GameSpeedSnapshot {
    fn clone(&self) -> Self {
        Self {
            tick: self.tick.clone(),
            game_speed: AtomicU8::new(self.game_speed.load(Ordering::Relaxed)),
        }
    }
}