use crate::sim::sim_date::sim_date::SimDate;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default, Serialize, Deserialize, Encode, Decode)]
pub struct TickCounter {
    /// Absolute tick. This is the running tick counter that never resets
    tick: AtomicU64,
}

impl TickCounter {
    pub fn tick(&self) {
        self.tick.fetch_add(1, Ordering::Relaxed);
    }

    /// Gets the current tick.
    pub fn value(&self) -> u64 {
        self.tick.load(Ordering::Relaxed)
    }

    pub fn reset(&self) {
        self.tick.store(0, Ordering::Relaxed);
    }

    pub fn current_date(&self) -> SimDate {
        SimDate::from(self.tick.load(Ordering::Relaxed))
    }
    
    pub fn update_from(&self, other: &TickCounter){
        self.tick.store(other.tick.load(Ordering::Relaxed), Ordering::Relaxed);
    }
}

impl std::fmt::Debug for TickCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TickCounter")
            .field("tick", &self.value())
            .field("date", &self.current_date())
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct AssetBasePath(pub PathBuf);

#[derive(Copy, Clone, Debug, Default)]
pub struct Dirty;
