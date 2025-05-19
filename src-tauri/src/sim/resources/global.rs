use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[derive(Default)]
pub struct TickCounter {
    tick: AtomicU64,
}
impl TickCounter {
    /// Increments the tick counter by 1. Thread-safe
    pub fn tick(&self) {
        self.tick.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns the current tick value. Relexed ordering.
    pub fn value(&self) -> u64 {
        self.tick.load(Ordering::Relaxed)
    }

    pub fn reset(&self) {
        self.tick.store(0, Ordering::Relaxed);
    }
}

impl std::fmt::Debug for TickCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TickCounter")
            .field("value", &self.value())
            .finish()
    }
}
#[derive(Debug, Default)]
pub struct AssetBasePath(pub PathBuf);

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SimManager {
    running: AtomicBool,
}
impl SimManager {
    pub fn resume_sim(&self) {
        self.running.store(true, Ordering::SeqCst);
    }
    pub fn pause_sim(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn new_sim(&self) {
        self.pause_sim();
    }

    pub fn load_sim(&self) {}
    pub fn default() -> Self {
        Self {
            running: AtomicBool::new(false),
        }
    }
}
