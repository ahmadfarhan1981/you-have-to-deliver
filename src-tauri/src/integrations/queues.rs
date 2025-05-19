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

// type DispatchQueue = DispatchQueue();

#[derive(Default)]
pub struct DispatchQueue {
    queue: Arc<SegQueue<SimCommand>>,
}
impl DispatchQueue {
    fn new() -> DispatchQueue {
        Self {
            queue: Arc::new(SegQueue::<SimCommand>::new()),
        }
    }

    pub fn push(&self, command: SimCommand) {
        self.queue.push(command);
    }
    pub fn len(&self) -> usize { self.queue.len() }
    pub fn is_empty(&self) -> bool { self.queue.is_empty() }
    pub fn clear(&self) { while self.queue.pop().is_some(){} }

}
impl fmt::Debug for DispatchQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.queue.len())
    }
}
pub struct QueueManager {
    dispatch: DispatchQueue, // dispatch queue is an arc because it will be shared by the integration to to the frontend.
    pub sim_manager: SystemCommandQueue<SimManagerCommand>, // other queues are just a segqueue, will only ever access by the queue systems
    pub game_speed_manager: SystemCommandQueue<GameSpeedManagerCommand>,
}

impl QueueManager {
    pub fn new() -> Self {
        Self {
            dispatch: DispatchQueue::new(),
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
    pub fn dispatch(&self) -> DispatchQueue {
        DispatchQueue {
            queue: Arc::clone(&self.dispatch.queue),
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
