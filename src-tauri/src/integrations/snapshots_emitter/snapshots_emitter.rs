use crate::integrations::snapshots::SnapshotField;
use crate::integrations::ui::AppContext;
use crate::sim::resources::global::TickCounter;
use dashmap::DashMap;
use legion::system;
use serde::Serialize;
use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tracing::{info, instrument};

trait SnapshotEmitter {
    fn maybe_emit(&self, tick: u64, app: &AppHandle)->bool;
}
#[derive(Debug, Default)]
pub enum ExportFrequency {
    #[default]
    EveryTick,
    EveryNTicks(u64),
    ManualOnly,
}


#[derive(Debug, Default)]
pub struct SnapshotEmitterConfig {
    pub frequency: ExportFrequency,
    pub event_name: &'static str,
    pub last_sent_tick: AtomicU64,
}

pub struct SnapshotEmitRegistry {
    pub emitters: Vec<Box<dyn SnapshotEmitter + Send + Sync>>,
}

impl SnapshotEmitRegistry {
    pub fn new() -> Self {
        Self {
            emitters: Vec::new(),
        }
    }

    pub fn register<E: SnapshotEmitter + Send + Sync + 'static>(&mut self, emitter: E) {
        self.emitters.push(Box::new(emitter));
    }

    #[instrument(skip_all, level="debug")]
    pub fn maybe_emit_all(&self, tick: u64, app: &AppHandle) {
        for emitter in &self.emitters {
            let _did_emit = emitter.maybe_emit(tick, app);
        }
    }
}


#[system]
pub fn run_snapshot_emitters(
    #[resource] registry: &SnapshotEmitRegistry,
    #[resource] app_context: &Arc<AppContext>,
    #[resource] tick_counter: &Arc<TickCounter>,
) {
    let current_tick = tick_counter.value(); // However you expose tick as u64
    registry.maybe_emit_all(current_tick, &app_context.app_handle);
}






#[derive(Debug, Default)]
pub struct SnapshotFieldEmitter<T> {
    pub field: Arc<SnapshotField<T>>,
    pub config: SnapshotEmitterConfig,
}

impl<T: Serialize> SnapshotEmitter for SnapshotFieldEmitter<T> {
    fn maybe_emit(&self, tick: u64, app: &AppHandle) -> bool {
        let should_emit = match self.config.frequency {
            ExportFrequency::EveryTick => true,
            ExportFrequency::EveryNTicks(n) => tick % n == 0,
            ExportFrequency::ManualOnly => false,
        };

        if should_emit {//&& self.config.last_sent_tick.load(Ordering::Relaxed) != tick {
            self.config.last_sent_tick.store(tick, Ordering::Relaxed);
            let data: &T = &*self.field.value.load();
            let _ = app.emit(self.config.event_name, data);

        }
        should_emit
    }
}


pub struct SnapshotCollectionEmitter<K, V>
where
    K: Eq + Hash + Clone,
    V: Serialize + Clone,
{
    pub map: Arc<DashMap<K, V>>,
    pub config: SnapshotEmitterConfig,
}

impl<K, V> SnapshotEmitter for SnapshotCollectionEmitter<K, V>
where
    K: Eq + Hash + Clone,
    V: Serialize + Clone,
{
    fn maybe_emit(&self, tick: u64, app: &AppHandle) -> bool {

        let should_emit = match self.config.frequency {
            ExportFrequency::EveryTick => true,
            ExportFrequency::EveryNTicks(n) => tick % n == 0,
            ExportFrequency::ManualOnly => false,
        };

        if should_emit && self.config.last_sent_tick.load(Ordering::Relaxed) != tick {
            self.config.last_sent_tick.store(tick, Ordering::Relaxed);

            let all: Vec<V> = self.map.iter().map(|entry| entry.value().clone()).collect();
            let _ = app.emit( self.config.event_name, &all);
        }
        should_emit
    }
}