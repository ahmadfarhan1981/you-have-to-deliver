use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct TickCounter {
    pub tick: u64,
}

#[derive(Debug, Default)]
pub struct AssetBasePath(pub PathBuf);