use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicU64;

#[derive(Debug, Default)]
pub struct TickCounter {
    pub tick: AtomicU64,
}

#[derive(Debug, Default)]
pub struct AssetBasePath(pub PathBuf);

