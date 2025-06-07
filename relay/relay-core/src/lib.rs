//! Relay Core - The heart of the AI collaboration framework
//!
//! This crate provides the core functionality for Relay:
//! - Privacy-first filesystem monitoring
//! - Async agent communication
//! - Capability-based security
//! - Audit logging

pub mod agent;
pub mod daemon;
pub mod privacy;
pub mod security;
pub mod storage;

#[derive(Debug, thiserror::Error)]
pub enum RelayError {
    #[error("Privacy violation: {0}")]
    PrivacyViolation(String),
    
    #[error("Agent error: {0}")]
    AgentError(String),
    
    #[error("Security error: {0}")]
    SecurityError(String),
    
    #[error("Storage error: {0}")]
    StorageError(#[from] sqlx::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, RelayError>;
