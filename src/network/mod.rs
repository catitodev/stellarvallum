//! Network module — Real Soroban RPC and Horizon integration.
//! Testnet-only in Beta.

pub mod address;
pub mod rpc;

use crate::config::Config;
use rpc::SorobanClient;
use tracing::info;

/// Deploy a contract to Stellar testnet via real Soroban RPC.
pub async fn deploy_testnet(
    config: &Config,
    wasm_path: &str,
    secret_key: &str,
) -> anyhow::Result<()> {
    info!("Deploying contract to testnet: {}", wasm_path);

    address::validate_secret_key(secret_key)?;

    let wasm_bytes = tokio::fs::read(wasm_path).await?;
    info!("WASM loaded: {} bytes", wasm_bytes.len());

    let client = SorobanClient::new(&config.network)?;
    client.validate_network().await?;

    let public_key = address::public_key_from_secret(secret_key)?;
    client.ensure_funded(&public_key).await?;

    let contract_id = client.deploy_contract(&wasm_bytes, secret_key).await?;

    println!("✅ Contract deployed!");
    println!("   Contract ID: {}", contract_id);
    println!("   Network: testnet");
    println!(
        "   Explorer: https://stellar.expert/explorer/testnet/contract/{}",
        contract_id
    );

    Ok(())
}
