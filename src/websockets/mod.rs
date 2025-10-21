//! # WebSockets Module
//!
//! Provides client and server functionality for WebSocket communication, including
//! subscription management and topic-based messaging.
//!
//! ## Features
//! - WebSocket client connection and messaging
//! - WebSocket server with client management, broadcasting, and message handling
//! - Subscription manager for topic-based event distribution
//! 

pub mod client;
pub mod server;
pub mod subscription_manager;

pub use client::WebSocketClient;
pub use server::WebSocketServer;
pub use subscription_manager::SubscriptionManager;