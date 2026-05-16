//! No-AI mode — Pure Rust heuristics (default). Free, private, instant.

#![allow(dead_code)]

use super::*;
use async_trait::async_trait;

pub struct NoAIProvider;

#[async_trait]
impl AIProvider for NoAIProvider {
    async fn analyze_contract(&self, _code: &str) -> AIResult<Value> {
        Ok(serde_json::json!({
            "provider": "no-ai",
            "note": "Using pure Rust heuristics via SHIELD module"
        }))
    }

    fn name(&self) -> &str {
        "no-ai"
    }

    async fn is_available(&self) -> bool {
        true
    }
}
