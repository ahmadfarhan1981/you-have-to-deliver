use crate::integrations::snapshots::SnapshotField;
use crate::integrations::ui::AppContext;
use crate::sim::resources::global::TickCounter;
use dashmap::DashMap;
use legion::system;
use serde::Serialize;
use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use dashmap::mapref::one::Ref;
use tauri::{AppHandle, Emitter};
use tracing::{debug, info, instrument, trace};
use tracing::field::debug;

trait SnapshotEmitter {
    fn maybe_emit(&self, tick: u64, last_update_map:  &DashMap<&'static str, u64>, app: &AppHandle) -> bool;
    fn emit(&self, tick: Option<u64>, app: &AppHandle);
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

    #[instrument(skip_all, level = "debug")]
    pub fn maybe_emit_all(&self, tick: u64, last_update_map:  &DashMap<&'static str, u64>, app: &AppHandle) {
        for emitter in &self.emitters {
            let _did_emit = emitter.maybe_emit(tick, last_update_map, app);
        }
    }
    pub fn force_emit_all(&self, app: &AppHandle) {
        for emitter in &self.emitters {
            let _did_emit = emitter.emit(None, app);
        }
    }

}

#[system]
pub fn run_snapshot_emitters(
    #[resource] registry: &Arc<SnapshotEmitRegistry>,
    #[resource] app_context: &Arc<AppContext>,
    #[resource] tick_counter: &Arc<TickCounter>,
    #[resource] data_last_update: &Arc< DashMap<&'static str, u64>>,
) {
    let current_tick = tick_counter.value(); // However you expose tick as u64
    registry.maybe_emit_all(current_tick, &data_last_update, &app_context.app_handle);
}

#[derive(Debug, Default)]
pub struct SnapshotFieldEmitter<T> {
    pub field: Arc<SnapshotField<T>>,
    pub config: SnapshotEmitterConfig,
}

impl<T: Serialize + std::fmt::Debug> SnapshotEmitter for SnapshotFieldEmitter<T> {
    fn maybe_emit(&self, tick: u64, last_update_map:  &DashMap<&'static str, u64>, app: &AppHandle) -> bool {
        let should_emit = match self.config.frequency {
            ExportFrequency::EveryTick => true,
            ExportFrequency::EveryNTicks(n) => tick % n == 0,
            ExportFrequency::ManualOnly => false,
        };
        let mut last_update: u64 = 0;
        let mut always_emit = false;
        match last_update_map.get(self.config.event_name){
            None => {
                debug!("Last data update time not found. Assume data always needs update");
                always_emit = true;
            }
            Some(cell) => { last_update = *cell.value()}
        }
        let  last_sent = self.config.last_sent_tick.load(Ordering::Relaxed);
        trace!("Event name {}, always emit? {}. last update? {} ", self.config.event_name, always_emit, last_update);
        if should_emit && ( always_emit || last_sent < last_update  ) {
            self.emit( Some(tick), app);
        }
        should_emit
    }
    fn emit(&self, tick: Option<u64>, app: &AppHandle) {
        //&& self.config.last_sent_tick.load(Ordering::Relaxed) != tick {
        if let Some(tick) = tick {
            self.config.last_sent_tick.store(tick, Ordering::Relaxed);
        }
        let data: &T = &*self.field.value.load();
        // info!("Snapshot field: {:?} {:?} {:?}", self.config.event_name, data,  &*self.field.value.load_full());
        let _ = app.emit(self.config.event_name, data);
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
    fn maybe_emit(&self, tick: u64, last_update_map:  &DashMap<&'static str, u64>, app: &AppHandle) -> bool {
        let should_emit = match self.config.frequency {
            ExportFrequency::EveryTick => true,
            ExportFrequency::EveryNTicks(n) => tick % n == 0,
            ExportFrequency::ManualOnly => false,
        };
        let mut last_update: u64 = 0;
        let mut always_emit = false;
        match last_update_map.get(self.config.event_name){
            None => {
                debug!("Last data update time not found. Assume data always needs update");
                always_emit = true;
            }
            Some(cell) => { last_update = *cell.value()}
        }

        let  last_sent = self.config.last_sent_tick.load(Ordering::Relaxed);
        trace!("Event name {}, always emit? {}. last update? {}  map:{:?}", self.config.event_name, always_emit, last_update, last_update_map);
        if should_emit  && ( always_emit || last_sent < last_update  )  && last_sent!= tick {
           self.emit(Some(tick), app);
        }
        should_emit
    }

    fn emit(&self, tick: Option<u64>, app: &AppHandle) {
        if let Some(tick) = tick {
            self.config.last_sent_tick.store(tick, Ordering::Relaxed);
        }
        let all: Vec<V> = self.map.iter().map(|entry| entry.value().clone()).collect();
        let _ = app.emit(self.config.event_name, &all);
    }
}
