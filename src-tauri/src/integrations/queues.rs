use crate::sim::game_speed::components::GameSpeed;
use crossbeam::queue::SegQueue;
use owo_colors::OwoColorize;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};
use legion::system;
use tracing::{debug, info, trace, warn};
use crate::integrations::system_queues::game_speed_manager::GameSpeedManagerCommand;
use crate::integrations::system_queues::sim_manager::SimManagerCommand;
use crate::sim::resources::global::SimManager;

pub enum SimCommand {
    GameSpeed(GameSpeedManagerCommand),
    SimManager(SimManagerCommand),
}


impl fmt::Debug for SimCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimCommand::GameSpeed(_) => write!(f, "SimCommand::GameSpeed(...)"),
            SimCommand::SimManager(_) => write!(f, "SimCommand::SimManager(...)"),
        }
    }
}




pub struct SystemCommandQueue<T> {
    pub queue: SegQueue<T>,
}
impl<T> SystemCommandQueue<T> {
    fn new() -> SystemCommandQueue<T> {
        Self {
            queue: SegQueue::<T>::new(),
        }
    }
}

pub struct QueueManager {
    dispatch: ExposedQueue<SimCommand>, // dispatch queue is an arc because it will be shared by the integration to to the frontend.
    sim_manager_dispatch: ExposedQueue<SimManagerCommand>,
    pub sim_manager: SystemCommandQueue<SimManagerCommand>, // other queues are just a segqueue, will only ever access by the queue systems
    pub game_speed_manager: SystemCommandQueue<GameSpeedManagerCommand>,
}

impl QueueManager {
    pub fn new() -> Self {
        Self {
            dispatch: ExposedQueue::<SimCommand>::new(),
            sim_manager_dispatch: ExposedQueue::<SimManagerCommand>::new(),
            sim_manager: SystemCommandQueue::<SimManagerCommand>::new(),
            game_speed_manager: SystemCommandQueue::<GameSpeedManagerCommand>::new(),
        }
    }
    pub fn print_summary(&self) {
        println!("{}", self.get_summary_string());
    }

    pub fn get_summary_string(&self) -> String {
        format!(
            "[Queue Summary] Dispatch: {}, Sim manager: {}, Game speed manager {}",
            self.dispatch().queue.len(),
            self.sim_manager.queue.len(),
            self.game_speed_manager.queue.len(),
        )
    }

    pub fn process_dispatch_queue(&self) {
        if self.dispatch.queue.is_empty(){
            debug!("Empty dispatch queue, skipping processing... ");
            return;
        }
        let dispatch_time_limit = Duration::from_millis(5);

        let start = Instant::now();

        let mut count = 0;
        while start.elapsed() < dispatch_time_limit {
            if let Some(command) = self.dispatch.queue.pop() {
                count += 1;
                trace!("Dispatch command: {:?}", command);
                match command {
                    SimCommand::GameSpeed(cmd) => self.game_speed_manager.queue.push(cmd),
                    SimCommand::SimManager(cmd) => self.sim_manager.queue.push(cmd),
                }
            } else {
                debug!("{} items dispatched", count);
                return;
            }
        }
        warn!(
            "Time limit reached when dispatching.  {} items dispatched",
            count
        );
    }

    ///some duplicate code with teh dispatch queue, and subsystems queue,
    /// SimManagerQueue bypasses the main loop for lifecycle tasks like NewGame, and it must must run within ECS systems to access World/Resources.
    /// So it gets its own dispatch queue that is outside of the suspended state killswitch.
    pub fn process_sim_manager_dispatch_queue(&self) {
        if self.sim_manager_dispatch.queue.is_empty(){
            debug!("Empty sim manager dispatch queue, skipping processing... ");
            return;
        }
        let dispatch_time_limit = Duration::from_millis(5);

        let start = Instant::now();

        let mut count = 0;
        while start.elapsed() < dispatch_time_limit {
            if let Some(command) = self.sim_manager_dispatch.queue.pop() {
                count += 1;
                trace!("Sim manager command: {:?}", command);
                match command {
                    _ => {self.sim_manager.queue.push(command);},
                }
            } else {
                debug!("{} items dispatched", count);
                return;
            }
        }
        warn!(
            "Time limit reached when dispatching.  {} items dispatched",
            count
        );
    }

    pub fn dispatch(&self) -> ExposedQueue<SimCommand> {
        ExposedQueue::<SimCommand>{
            queue: Arc::clone(&self.dispatch.queue),

        }
    }
    pub fn sim_manager_dispatch(&self) -> ExposedQueue<SimManagerCommand> {
        ExposedQueue::<SimManagerCommand>{
            queue: Arc::clone(&self.sim_manager_dispatch.queue),
        }
    }
}
impl fmt::Debug for QueueManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_summary_string())
    }
}

#[system]
pub fn handle_dispatch_queue(#[resource]queue_manager: &QueueManager ) {
    info!("handle_dispatch_queue");
    queue_manager.process_dispatch_queue();

}

#[system]
pub fn handle_sim_manager_dispatch_queue(#[resource]queue_manager: &QueueManager ) {
    info!("handle_dispatch_queue");
    queue_manager.process_sim_manager_dispatch_queue();
}



pub struct ExposedQueue<T> {
    queue: Arc<SegQueue<T>>,
}

impl<T> ExposedQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(SegQueue::new()),
        }
    }

    pub fn push(&self, item: T) {
        self.queue.push(item);
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn clear(&self) {
        while self.queue.pop().is_some() {}
    }

    pub fn inner(&self) -> Arc<SegQueue<T>> {
        Arc::clone(&self.queue)
    }
}

// Optional Debug if T: Debug
impl<T> fmt::Debug for ExposedQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExposedQueue(len: {})", self.queue.len())
    }
}
impl<T> Default for ExposedQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}
