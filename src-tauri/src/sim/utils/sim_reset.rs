use std::sync::atomic::AtomicBool;

pub struct ResetRequest {
    pub should_reset: AtomicBool,
}
impl Default for ResetRequest {
    fn default() -> Self {
        Self { should_reset: AtomicBool::new(false) }
    }
}