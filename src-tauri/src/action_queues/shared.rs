use crate::integrations::queues::SystemCommandQueue;
use crossbeam::queue::SegQueue;
use std::time::{Duration, Instant};
use tracing::{debug, trace, warn};

pub fn timed_dispatch<T, F>(queue: &SystemCommandQueue<T>, limit: Duration, mut handler: F)
where
    F: FnMut(T),
{
    if queue.queue.is_empty() {
        trace!("queue is empty");
        return;
    }
    let start = Instant::now();
    let mut count = 0;

    while start.elapsed() < limit {
        match queue.queue.pop() {
            Some(item) => {
                handler(item);
                count += 1;
            }
            None => {
                debug!("{count} items dispatched");
                return;
            }
        }
    }

    warn!("Time limit reached when dispatching. {count} items dispatched");
}
