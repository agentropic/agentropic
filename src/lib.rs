//! A comprehensive Rust framework for building intelligent, autonomous multi-agent systems.
#![warn(missing_docs)]
#![warn(clippy::all)]

pub use agentropic_core as agent_core;
pub use agentropic_messaging as messaging;
pub use agentropic_cognition as cognition;
pub use agentropic_patterns as patterns;
pub use agentropic_runtime as runtime;

/// Unified prelude for convenient imports
pub mod prelude;

// Re-export commonly used types at crate root for convenience
pub use agent_core::{Agent, AgentId};
pub use messaging::{Message, Performative, Router, Mailbox};
pub use runtime::{Runtime, RuntimeConfig};