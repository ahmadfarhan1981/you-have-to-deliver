use crate::integrations::ui::AppContext;
use crate::sim::resources::global::TickCounter;
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use legion::system;
use serde::Serialize;
use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tracing::field::debug;
use tracing::{debug, info, instrument, trace};
use arc_swap::ArcSwap;

pub trait SnapshotEmitter {
    fn maybe_emit(&self, tick: u64, last_update_map:  &DashMap<&'static str, u64>, app: &AppHandle) -> bool;
    fn emit(&self, tick: Option<u64>, app: &AppHandle);
    
    fn reset(&self);
}
#[derive(Debug, Default)]
pub enum ExportFrequency {
    #[default]
    EveryTick,
    EveryNTicks(u64),
    ManualOnly,
}

#[derive(Debug)]
pub struct SnapshotEmitterConfig {
    pub frequency: ExportFrequency,
    pub event_name: SnapshotEvent,
    pub last_sent_tick: AtomicU64,
}

pub struct SnapshotEmitRegistry {
    pub emitters: Vec<Box<dyn SnapshotEmitter + Send + Sync>>,
    last_update_map: DashMap<&'static str, u64>,
}

impl SnapshotEmitRegistry {
    pub fn new() -> Self {
        Self {
            emitters: Vec::new(),
            last_update_map: DashMap::<&'static str, u64>::new(),
        }
    }
    
    pub fn register<E: SnapshotEmitter + Send + Sync + 'static>(&mut self, emitter: E) {
        self.emitters.push(Box::new(emitter));
    }

    #[instrument(skip_all, level = "trace")]
    pub fn maybe_emit_all(&self, tick: u64, app: &AppHandle) {
        for emitter in &self.emitters {
            let _did_emit = emitter.maybe_emit(tick, &self.last_update_map, app);
        }
    }

    pub fn mark_data_updated(&self, event: SnapshotEvent, tick: u64) {
        self.last_update_map.insert(event.as_str(), tick);
    }
    
    pub fn reset(&self) {
        for emitter in &self.emitters {
            emitter.reset();
            self.last_update_map.clear();
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
) {
    let current_tick = tick_counter.value(); // However you expose tick as u64
    registry.maybe_emit_all(current_tick, &app_context.app_handle);
}

#[derive(Debug)]
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
        match last_update_map.get(self.config.event_name.into()){
            None => {
                trace!("Last data update time not found. Assume data always needs update {}", self.config.event_name );
                always_emit = true;
            }
            Some(cell) => { last_update = *cell.value()}
        }
        let  last_sent = self.config.last_sent_tick.load(Ordering::Relaxed);
        debug!("Event name {}, always emit? {}. last update? {} {}", self.config.event_name, always_emit, last_update, last_sent);
        if should_emit && ( always_emit || last_sent <= last_update  ) {//use `<=` , some new game scenario will result in last sent == last lets send the update if last sent and last update is the same.
            self.emit( Some(tick), app);
        }
        should_emit
    }
    fn emit(&self, tick: Option<u64>, app: &AppHandle) {
        debug!("Event name {} emitting..", self.config.event_name );
        //&& self.config.last_sent_tick.load(Ordering::Relaxed) != tick {
        if let Some(tick) = tick {
            self.config.last_sent_tick.store(tick, Ordering::Relaxed);
        }
        let data: &T = &*self.field.value.load();
        debug!("Data {:?} ", data );
        // info!("Snapshot field: {:?} {:?} {:?}", self.config.event_name, data,  &*self.field.value.load_full());
        let _ = app.emit(self.config.event_name.into(), data);
    }

    fn reset(&self) {
        self.config.last_sent_tick.store(0, Ordering::Relaxed);
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
        match last_update_map.get(self.config.event_name.into()){
            None => {
                trace!("{} Last data update time not found. Assume data always needs update", self.config.event_name );
                always_emit = true;
            }
            Some(cell) => { last_update = *cell.value()}
        }

        let  last_sent = self.config.last_sent_tick.load(Ordering::Relaxed);
        trace!("Event name {}, always emit? {}. last update? {}  map:{:?}", self.config.event_name, always_emit, last_update, last_update_map);
        if should_emit  && ( always_emit || last_sent <= last_update  )  && last_sent!= tick {
           self.emit(Some(tick), app);
        }
        should_emit
    }

    fn emit(&self, tick: Option<u64>, app: &AppHandle) {
        trace!("Event name {} emitting..", self.config.event_name );
        if let Some(tick) = tick {
            self.config.last_sent_tick.store(tick, Ordering::Relaxed);
        }
        let all: Vec<V> = self.map.iter().map(|entry| entry.value().clone()).collect();
        let _ = app.emit(self.config.event_name.into(), &all);
    }

    fn reset(&self) {
        self.config.last_sent_tick.store(0, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SnapshotEvent {
    GameSpeed,
    Persons,
    DebugDisplay,
    Teams,
    Company,
    Stress,
    StressHistory,
    MonthlyAvailability,
    Thoughts,
    CalendarEvents
}

impl SnapshotEvent {
    pub const fn as_str(self) -> &'static str {
        match self {
            SnapshotEvent::GameSpeed => "game_speed_snapshot",
            SnapshotEvent::Persons => "persons_snapshot",
            SnapshotEvent::DebugDisplay => "debug_display_snapshot",
            SnapshotEvent::Teams => "teams_snapshot",
            SnapshotEvent::Company => "company_snapshot",
            SnapshotEvent::Stress => "stress_snapshot",
            SnapshotEvent::StressHistory => "stress_history_snapshot",
            SnapshotEvent::MonthlyAvailability => "monthly_availability_snapshot",
            SnapshotEvent::Thoughts => "thoughts_snapshot",
            SnapshotEvent::CalendarEvents => "calendar_events_snapshot"
        }
    }
}

// For DashMap keys, you need the string, so implement some helper traits
impl From<SnapshotEvent> for &'static str {
    fn from(event: SnapshotEvent) -> Self {
        event.as_str()
    }
}

impl std::fmt::Display for SnapshotEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Default)]
pub struct SnapshotField<T> {
    pub value: ArcSwap<Arc<T>>,
}

impl<T> From<T> for SnapshotField<T> {
    fn from(value: T) -> Self {
        Self {
            value: ArcSwap::from_pointee(value.into()),
        }
    }
}

impl<T> Clone for SnapshotField<T> {
    fn clone(&self) -> Self {
        Self {
            value: ArcSwap::from(self.value.load_full()),
        }
    }
}

// NOTE: SnapshotCollection is currently unused.
// It was prototyped for symmetry with SnapshotField, but direct DashMap use is more practical.
// Keeping it around in case future abstractions or save/load tools benefit from a wrapper.

pub struct SnapshotCollection<K, V>
where
    K: Eq + Hash + Clone,
    V: Serialize + Clone,
{
    pub map: DashMap<K, V>,
}

impl<K, V> From<DashMap<K, V>> for SnapshotCollection<K, V>
where
    K: Eq + Hash + Clone,
    V: Serialize + Clone,
{
    fn from(map: DashMap<K, V>) -> Self {
        Self { map }
    }
}