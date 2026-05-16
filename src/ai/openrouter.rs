//! OpenRouter provider — Optional multi-model AI via OpenRouter API.

use super::*;
use async_trait::async_trait;
use crate::secret::Secret;
use tracing::{debug, info};

pub struct OpenRouterProvider {
    api_key: Secret<String>,
    model: String,
    client: reqwest::Client,
}

impl OpenRouterProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| "anthropic/claude-3.5-sonnet".to_string());
        info!("OpenRouter provider: model={}", model);
        Self {
            api_key: Secret::new(api_key),
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AIProvider for OpenRouterProvider {
    async fn analyze_contract(&self, code: &str) -> AIResult<Value> {
        let body = serde_json::json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": "You are a Soroban smart contract security auditor. Return JSON with score, findings, summary."},
                {"role": "user", "content": format!("Analyze this contract:\n```rust\n{}\n```\nReturn JSON only.", code)}
            ],
            "temperature": 0.1,
            "max_tokens": 4000,
        });

        debug!("Sending to OpenRouter...");

        let resp = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key.expose()))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AIError::ApiError(e.to_string()))?;

        if resp.status() == 429 {
            return Err(AIError::RateLimited);
        }
        if resp.status() == 401 {
            return Err(AIError::AuthFailed);
        }

        let json: Value = resp.json().await.map_err(|e| AIError::InvalidResponse(e.to_string()))?;
        Ok(json)
    }

    fn name(&self) -> &str {
        "openrouter"
    }

    async fn is_available(&self) -> bool {
        true
    }
}
