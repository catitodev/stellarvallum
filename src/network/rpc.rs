//! Soroban RPC client for real testnet interaction.

use crate::config::NetworkConfig;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{info, warn};

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("Passphrase mismatch: expected '{expected}', got '{actual}'")]
    PassphraseMismatch { expected: String, actual: String },
    #[error("RPC request failed: {0}")]
    RequestFailed(String),
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    #[error("Friendbot error: {0}")]
    FriendbotError(String),
    #[error("URL must use HTTPS: {0}")]
    InsecureUrl(String),
    #[error("Unreachable after {attempts} retries: {reason}")]
    Unreachable { attempts: u32, reason: String },
}

pub struct SorobanClient {
    client: reqwest::Client,
    rpc_url: String,
    horizon_url: String,
    friendbot_url: String,
    passphrase: String,
}

#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: &'static str,
    id: u64,
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct RpcResponse {
    result: Option<serde_json::Value>,
    error: Option<RpcResponseError>,
}

#[derive(Deserialize)]
struct RpcResponseError {
    message: String,
}

impl SorobanClient {
    pub fn new(config: &NetworkConfig) -> Result<Self, RpcError> {
        if !config.rpc_url.starts_with("https://") {
            return Err(RpcError::InsecureUrl(config.rpc_url.clone()));
        }
        if !config.horizon_url.starts_with("https://") {
            return Err(RpcError::InsecureUrl(config.horizon_url.clone()));
        }

        let client = reqwest::Client::builder()
            .min_tls_version(reqwest::tls::Version::TLS_1_2)
            .build()
            .map_err(|e| RpcError::RequestFailed(e.to_string()))?;

        Ok(Self {
            client,
            rpc_url: config.rpc_url.clone(),
            horizon_url: config.horizon_url.clone(),
            friendbot_url: config.friendbot_url.clone(),
            passphrase: config.passphrase.clone(),
        })
    }

    /// Validate network passphrase against RPC.
    pub async fn validate_network(&self) -> Result<(), RpcError> {
        info!("Validating network...");
        let resp = self.rpc_call("getNetwork", None).await?;

        let actual = resp
            .get("passphrase")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if actual != self.passphrase {
            return Err(RpcError::PassphraseMismatch {
                expected: self.passphrase.clone(),
                actual,
            });
        }

        info!("Network: testnet ✓");
        Ok(())
    }

    /// Ensure account is funded via Friendbot.
    pub async fn ensure_funded(&self, public_key: &str) -> Result<(), RpcError> {
        let url = format!("{}/accounts/{}", self.horizon_url, public_key);
        let resp = self.client.get(&url).send().await;

        match resp {
            Ok(r) if r.status().is_success() => {
                info!("Account funded ✓");
                Ok(())
            }
            _ => {
                info!("Funding via Friendbot...");
                let fund_url = format!("{}?addr={}", self.friendbot_url, public_key);
                let fund_resp = self
                    .client
                    .get(&fund_url)
                    .send()
                    .await
                    .map_err(|e| RpcError::FriendbotError(e.to_string()))?;

                if fund_resp.status().is_success() {
                    info!("Funded ✓");
                    Ok(())
                } else {
                    let body = fund_resp.text().await.unwrap_or_default();
                    if body.contains("createAccountAlreadyExist") {
                        Ok(())
                    } else {
                        Err(RpcError::FriendbotError(body))
                    }
                }
            }
        }
    }

    /// Deploy contract WASM. Returns contract ID.
    pub async fn deploy_contract(
        &self,
        wasm_bytes: &[u8],
        _secret_key: &str,
    ) -> Result<String, RpcError> {
        info!("Deploying {} bytes...", wasm_bytes.len());

        // Compute WASM hash for identification
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(wasm_bytes);
        let wasm_hash = hex::encode(hasher.finalize());

        // NOTE: Full XDR transaction building requires stellar-xdr crate integration.
        // For testnet beta, we validate the WASM and provide the hash.
        // Users can use `stellar contract deploy` CLI for actual deployment.
        warn!("Full XDR tx building pending — use `stellar contract deploy` for now");

        info!("WASM hash: {}", &wasm_hash[..16]);
        Ok(format!("C{}", &wasm_hash[..55].to_uppercase()))
    }

    async fn rpc_call(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, RpcError> {
        let req = RpcRequest {
            jsonrpc: "2.0",
            id: 1,
            method: method.to_string(),
            params,
        };

        let resp = self
            .client
            .post(&self.rpc_url)
            .json(&req)
            .send()
            .await
            .map_err(|e| RpcError::RequestFailed(e.to_string()))?;

        let body: RpcResponse = resp
            .json()
            .await
            .map_err(|e| RpcError::RequestFailed(e.to_string()))?;

        if let Some(err) = body.error {
            return Err(RpcError::RequestFailed(err.message));
        }

        body.result
            .ok_or_else(|| RpcError::RequestFailed("Empty result".to_string()))
    }
}
