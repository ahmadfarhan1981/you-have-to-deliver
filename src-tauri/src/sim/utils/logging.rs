use std::sync::Once;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::{fmt, EnvFilter};

/// Keeps global init from running twice
static INIT: Once = Once::new();

/// Initialize simple stdout logger with filtering and context info
pub fn init_logging() {
    INIT.call_once(|| {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        tracing_subscriber::fmt()
            .with_timer(ChronoLocal::new("%Y-%m-%dT%H:%M:%S%:z".to_string()))
            .with_env_filter(env_filter)
            .with_ansi(true)
            .with_line_number(true)
            .with_thread_names(true)
            .with_target(false)
            .with_level(true)
            .with_span_events(fmt::format::FmtSpan::CLOSE)
            .init();
    });
}
