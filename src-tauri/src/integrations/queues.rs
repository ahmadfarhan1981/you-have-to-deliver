use crate::integrations::system_queues::game_speed_manager::GameSpeedManagerCommand;
use crate::integrations::system_queues::sim_manager::SimManagerCommand;
use crate::integrations::system_queues::sim_manager::SimManagerCommand::ResetSim;
use crate::sim::game_speed::components::GameSpeed;
use crate::sim::resources::global::SimManager;
use crossbeam::queue::SegQueue;
use legion::system;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, info, trace, warn};
use crate::integrations::system_queues::team_manager::{TeamAssignmentCommand, TeamManagerCommand};

#[derive(Debug, Default)]
pub struct UICommandQueues {
    pub runtime: ExposedQueue<SimCommand>,
    pub control: ExposedQueue<SimManagerCommand>,
}

pub enum SimCommand {
    GameSpeed(GameSpeedManagerCommand),
    SimManager(SimManagerCommand),
    TeamManager(TeamManagerCommand),
    TeamAssignment(TeamAssignmentCommand),
}

impl fmt::Debug for SimCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimCommand::GameSpeed(_) => write!(f, "SimCommand::GameSpeed(...)"),
            SimCommand::SimManager(_) => write!(f, "SimCommand::SimManager(...)"),
            SimCommand::TeamManager(_) => write!(f, "SimCommand::TeamManager(...)"),
            SimCommand::TeamAssignment(_) => {write!(f, "SimCommand::TeamAssignment(...)")}
        }
    }
}

#[derive(Default)]
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
    pub new_game_manager: SystemCommandQueue<SimManagerCommand>,
    pub team_manager: SystemCommandQueue<TeamManagerCommand>,
    pub team_assignment: SystemCommandQueue<TeamAssignmentCommand>,
}

impl QueueManager {

    pub fn print_summary(&self) {
        info!("{}", self.get_summary_string());
    }

    pub fn new() -> Self {
        Self {
            dispatch: ExposedQueue::<SimCommand>::new(),
            sim_manager_dispatch: ExposedQueue::<SimManagerCommand>::new(),
            sim_manager: SystemCommandQueue::<SimManagerCommand>::new(),
            game_speed_manager: SystemCommandQueue::<GameSpeedManagerCommand>::new(),
            new_game_manager: SystemCommandQueue::<SimManagerCommand>::new(),
            team_manager: SystemCommandQueue::<TeamManagerCommand>::new(),
            team_assignment: SystemCommandQueue::<TeamAssignmentCommand>::new(),
        }
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
        if self.dispatch.queue.is_empty() {
            trace!("Empty dispatch queue, skipping processing... ");
            return;
        }
        let dispatch_time_limit = Duration::from_millis(5);

        let start = Instant::now();

        let mut count = 0;
        while start.elapsed() < dispatch_time_limit {
            if let Some(command) = self.dispatch.queue.pop() {
                count += 1;
                info!("Dispatch command: {:?}", command);
                match command {
                    SimCommand::GameSpeed(cmd) => self.game_speed_manager.queue.push(cmd),
                    SimCommand::SimManager(cmd) => self.sim_manager.queue.push(cmd),
                    SimCommand::TeamManager(cmd) => self.team_manager.queue.push(cmd),
                    SimCommand::TeamAssignment(cmd) => {self.team_assignment.queue.push(cmd)},
                }
            } else {
                trace!("{} items dispatched", count);
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
        if self.sim_manager_dispatch.queue.is_empty() {
            trace!("Empty sim manager dispatch queue, skipping processing... ");
            return;
        }
        let dispatch_time_limit = Duration::from_millis(5);

        let start = Instant::now();

        let mut count = 0;
        while start.elapsed() < dispatch_time_limit {
            if let Some(command) = self.sim_manager_dispatch.queue.pop() {
                count += 1;
                info!("Sim manager command: {:?}", command);
                match command {
                    SimManagerCommand::ResetSim => self
                        .new_game_manager
                        .queue
                        .push(SimManagerCommand::ResetSim),
                    _ => {
                        self.sim_manager.queue.push(command);
                    }
                }
            } else {
                trace!("{} items dispatched", count);
                return;
            }
        }
        warn!(
            "Time limit reached when dispatching.  {} items dispatched",
            count
        );
    }

    pub fn dispatch(&self) -> ExposedQueue<SimCommand> {
        ExposedQueue::<SimCommand> {
            queue: Arc::clone(&self.dispatch.queue),
        }
    }
    pub fn sim_manager_dispatch(&self) -> ExposedQueue<SimManagerCommand> {
        ExposedQueue::<SimManagerCommand> {
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
pub fn handle_dispatch_queue(#[resource] queue_manager: &QueueManager) {
    trace!("Dispatch queue");
    queue_manager.process_dispatch_queue();
}

#[system]
pub fn handle_sim_manager_dispatch_queue(#[resource] queue_manager: &QueueManager) {
    trace!("Sim manager dispatch queue");
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
