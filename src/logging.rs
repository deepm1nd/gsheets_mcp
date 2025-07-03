use tracing::{debug, error, info, instrument, Level, warn};
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer()
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_level(true)
            .json() // Use JSON for structured logs; remove for plain text
        )
        .init();
}

// Structured logging setup and macros
// ...implementation skeleton...

pub fn log_info(msg: &str) {
    info!(target: "gsheets_mcp", "{}", msg);
}

pub fn log_error(msg: &str) {
    error!(target: "gsheets_mcp", "{}", msg);
}

pub fn log_debug(msg: &str) {
    debug!(target: "gsheets_mcp", "{}", msg);
}

pub fn log_warn(msg: &str) {
    warn!(target: "gsheets_mcp", "{}", msg);
}

#[macro_export]
macro_rules! log_request {
    ($endpoint:expr, $payload:expr) => {
        tracing::info!(target: "gsheets_mcp", endpoint = $endpoint, payload = ?$payload, "Request received");
    };
}

#[macro_export]
macro_rules! log_response {
    ($endpoint:expr, $result:expr) => {
        tracing::info!(target: "gsheets_mcp", endpoint = $endpoint, result = ?$result, "Response sent");
    };
}
