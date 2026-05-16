//! AI Provider System — Optional, agnostic, with No-AI default.

use async_trait::async_trait;
use serde_json::Value;
use thiserror::Error;

pub mod local;
pub mod no_ai;
pub mod openrouter;

#[derive(Error, Debug)]
pub enum AIError {
    #[error("Provider not available: {0}")]
    ProviderUnavailable(String),
    #[error("API request failed: {0}")]
    ApiError(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Rate limited")]
    RateLimited,
    #[error("Auth failed")]
    AuthFailed,
}

pub type AIResult<T> = Result<T, AIError>;

#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn analyze_contract(&self, code: &str) -> AIResult<Value>;
    fn name(&self) -> &str;
    async fn is_available(&self) -> bool;
}
