use std::sync::atomic::AtomicBool;

pub struct ResetRequest {
    pub should_reset: AtomicBool,
}
