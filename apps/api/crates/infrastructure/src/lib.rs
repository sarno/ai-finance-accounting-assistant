//! Infrastructure crate — concrete implementations of port traits.
//!
//! This crate owns:
//!   - SQLx repository implementations
//!   - Local/S3 storage implementation
//!   - OpenAI/OCR client implementation
//!   - Database connection pool management

pub mod db;
pub mod repositories;
pub mod storage;
pub mod ai_client;
