use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct TickCounter {
    tick: AtomicU64,
}
impl TickCounter {
    pub fn tick(&self) {
        /// Increments the tick counter by 1. Thread-safe
        self.tick.fetch_add(1, Ordering::Relaxed);
    }
    pub fn value(&self) -> u64 {
        /// Returns the current tick value. Relexed ordering.
        self.tick.load(Ordering::Relaxed)
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

