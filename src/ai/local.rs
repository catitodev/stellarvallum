//! Local LLM provider — Ollama/LM Studio. 100% private.

#![allow(dead_code)]

use super::*;
use async_trait::async_trait;
use tracing::{debug, info};

pub struct LocalProvider {
    endpoint: String,
    model: String,
    client: reqwest::Client,
}

impl LocalProvider {
    pub fn new(endpoint: String, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| "codellama:13b".to_string());
        info!("Local LLM: {} at {}", model, endpoint);
        Self {
            endpoint,
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AIProvider for LocalProvider {
    async fn analyze_contract(&self, code: &str) -> AIResult<Value> {
        let body = serde_json::json!({
            "model": self.model,
            "prompt": format!("Analyze this Soroban contract for vulnerabilities. Return JSON.\n```rust\n{}\n```", code),
            "stream": false,
        });

        debug!("Sending to local LLM: {}", self.endpoint);

        let resp = self.client
            .post(format!("{}/api/generate", self.endpoint))
            .json(&body)
            .send()
            .await
            .map_err(|e| AIError::ProviderUnavailable(format!("Cannot reach {}: {}", self.endpoint, e)))?;

        let json: Value = resp.json().await.map_err(|e| AIError::InvalidResponse(e.to_string()))?;
        Ok(json)
    }

    fn name(&self) -> &str {
        "local"
    }

    async fn is_available(&self) -> bool {
        self.client.get(&self.endpoint).send().await.is_ok()
    }
}
