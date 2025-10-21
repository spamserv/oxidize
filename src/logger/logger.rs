//! # Logging Initialization
//!
//! Sets up the global `tracing` subscriber with console and file output.
//! Logs are rotated daily and formatted as JSON in `./logs/blockchain.log`.
//! 
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

/// Initialize tracing subscriber for global logging
pub fn init_logging() -> tracing_appender::non_blocking::WorkerGuard {
    // Daily rotating file
    let file_appender = rolling::daily("./logs", "blockchain.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Create EnvFilter (fallback to INFO if RUST_LOG not set)
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Console layer
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .pretty()
        .with_filter(env_filter.clone());

    // File JSON layer
    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .json()
        .with_filter(env_filter.clone());

    // Combine layers
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    guard // return guard to keep logs flushed
}

