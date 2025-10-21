//! # Config
//!
//! Centralized configuration for the Oxidize blockchain.
//! Contains constants defining system parameters such as
//! difficulty, rewards, fees, and network settings.
//!
//! ```rust
//! use oxidize::config::*;
//! println!("Difficulty: {}", BLOCKCHAIN_INITIAL_DIFFICULTY);
//! ```
//!
//! ## Exports
//! - [`constants`]: Blockchain configuration constants.
mod constants;

pub use constants::*;