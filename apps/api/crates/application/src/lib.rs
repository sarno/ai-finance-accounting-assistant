//! Application crate — use-case services, commands, queries, and port traits.
//!
//! This crate depends on the domain crate only.
//! It defines **port traits** (Repository, StoragePort, etc.) that are
//! implemented by the infrastructure crate.

pub mod dto;
pub mod errors;
pub mod ports;
pub mod services;
