// For complex range queries and overlaps
#[derive(Debug, Default)]
pub struct IntervalTree {
    // Simplified interval tree - in practice you'd want a proper implementation
    intervals: Vec<(u64, u64, u64)>, // (start_tick, end_tick, event_id)
}

impl IntervalTree {
    pub fn insert(&mut self, start: u64, end: u64, event_id: u64) {
        self.intervals.push((start, end, event_id));
        // In a real implementation, you'd maintain a balanced tree structure
    }

    pub fn query_overlapping(&self, start: u64, end: u64) -> Vec<u64> {
        self.intervals.iter()
            .filter(|(s, e, _)| *s < end && *e > start)
            .map(|(_, _, id)| *id)
            .collect()
    }

    pub fn remove_event(&mut self, event_id: u64) {
        self.intervals.retain(|(_, _, id)| *id != event_id);
    }
}
