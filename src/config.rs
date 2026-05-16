//! Configuration management for StellarVallum.

use crate::secret::Secret;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub ai: AIConfig,
    pub shield: ShieldConfig,
    pub spear: SpearConfig,
    pub dashboard: DashboardConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub name: String,
    pub rpc_url: String,
    pub horizon_url: String,
    pub friendbot_url: String,
    pub passphrase: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AIConfig {
    pub provider: String,
    pub model: Option<String>,
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShieldConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpearConfig {
    pub enabled: bool,
    pub max_transactions: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DashboardConfig {
    pub enabled: bool,
    pub port: u16,
}

impl Config {
    /// Get the AI API key as a Secret (never logged).
    pub fn ai_api_key(&self) -> Option<Secret<String>> {
        self.ai.api_key.clone().map(Secret::new)
    }
}

/// Load configuration from TOML file.
pub async fn load(path: &str) -> anyhow::Result<Config> {
    let path = Path::new(path);

    if !path.exists() {
        info!("Config not found at {}, using defaults", path.display());
        return Ok(default_config());
    }

    let content = tokio::fs::read_to_string(path).await?;
    let config: Config = toml::from_str(&content)?;

    info!(
        "Config loaded: network={}, ai={}",
        config.network.name, config.ai.provider
    );

    Ok(config)
}

/// Display current configuration (redacts secrets).
pub fn display(config: &Config) -> anyhow::Result<()> {
    let mut display_config = config.clone();
    display_config.ai.api_key = config.ai.api_key.as_ref().map(|_| "[REDACTED]".to_string());
    println!("{}", toml::to_string_pretty(&display_config)?);
    Ok(())
}

/// Interactive setup helper.
pub async fn setup() -> anyhow::Result<()> {
    println!("StellarVallum Configuration");
    println!("═══════════════════════════");
    println!();
    println!("Network: Stellar Testnet");
    println!("  RPC:      https://soroban-testnet.stellar.org:443");
    println!("  Horizon:  https://horizon-testnet.stellar.org");
    println!("  Friendbot: https://friendbot.stellar.org");
    println!();
    println!("To get started:");
    println!("  1. Create a testnet keypair at https://lab.stellar.org/");
    println!("  2. Fund it: curl 'https://friendbot.stellar.org?addr=G...'");
    println!("  3. Scan:    stellarvallum scan --wasm ./contract.wasm");
    Ok(())
}

fn default_config() -> Config {
    Config {
        network: NetworkConfig {
            name: "testnet".to_string(),
            rpc_url: "https://soroban-testnet.stellar.org:443".to_string(),
            horizon_url: "https://horizon-testnet.stellar.org".to_string(),
            friendbot_url: "https://friendbot.stellar.org".to_string(),
            passphrase: "Test SDF Network ; September 2015".to_string(),
        },
        ai: AIConfig {
            provider: "none".to_string(),
            model: None,
            api_key: None,
            endpoint: None,
        },
        shield: ShieldConfig { enabled: true },
        spear: SpearConfig {
            enabled: true,
            max_transactions: 1000,
        },
        dashboard: DashboardConfig {
            enabled: true,
            port: 8501,
        },
    }
}
