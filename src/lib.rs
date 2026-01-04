//! # Agentropic
//!
//! Agent-Oriented Programming in Rust - Batteries Included
//!
//! This is the main facade crate that re-exports all components of the Agentropic ecosystem.
//!
//! ## Status
//!
//! ⚠️ **Active Development** - More crates coming soon!
//!
//! ## Currently Available
//!
//! - `agentropic-core` - Agent primitives and traits
//! - `agentropic-messaging` - Communication protocols

#![warn(missing_docs)]

// Re-export available crates
pub use agentropic_core as core;
pub use agentropic_messaging as messaging;

/// Prelude module for convenient imports
pub mod prelude {
    pub use agentropic_core::prelude::*;
    pub use agentropic_messaging::prelude::*;
}