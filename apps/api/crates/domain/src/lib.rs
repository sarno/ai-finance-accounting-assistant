//! Domain crate — pure business entities, value objects, and domain rules.
//!
//! This crate has **zero** I/O dependencies.
//! It must not depend on `sqlx`, `axum`, `reqwest`, or any external service.

pub mod entities;
pub mod value_objects;
pub mod errors;
pub mod rules;
