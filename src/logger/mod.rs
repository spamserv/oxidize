//! # Logger
//!
//! Provides global logging initialization using `tracing`.
//! Supports pretty console output and daily rotating JSON log files.
//!
//! ```rust
//! use oxidize::logger::init_logging;
//! let _guard = init_logging();
//! tracing::info!("Logger initialized.");
//! ```
//!
//! ## Exports
//! - [`init_logging`]: Initializes the global tracing subscriber.
//! 
mod logger;

pub use logger::init_logging;