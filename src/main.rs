#![forbid(unsafe_code)]

//! StellarVallum - The First Wall for Soroban
//!
//! Testnet-First Security Framework for Soroban Smart Contracts

mod ai;
mod chain;
mod config;
mod dashboard;
mod network;
mod secret;
mod shield;
mod spear;
mod utils;

use clap::{Parser, Subcommand};
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "stellarvallum")]
#[command(about = "The First Wall for Soroban - Testnet-First Security")]
#[command(version = "0.2.0-testnet")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Network to use (testnet only in beta)
    #[arg(short, long, default_value = "testnet")]
    network: String,

    /// Configuration file path
    #[arg(short, long, default_value = "./config/vallum.toml")]
    config: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan project or file for vulnerabilities (SHIELD)
    Scan {
        /// Path to file or directory to scan
        #[arg(short, long)]
        path: String,

        /// Scan profile: auto, contract, dapp, backend, config, pipeline
        #[arg(short = 'P', long, default_value = "auto")]
        profile: String,

        /// AI mode: none (default), openrouter, local
        #[arg(short, long, default_value = "none")]
        mode: String,

        /// Output format: json, csv
        #[arg(short, long, default_value = "json")]
        output: String,

        /// Output file path
        #[arg(short = 'f', long)]
        out_file: Option<String>,
    },

    /// Deploy contract to testnet
    DeployTestnet {
        /// Path to WASM file
        #[arg(short, long)]
        wasm: String,

        /// Source account secret key (S...)
        #[arg(short, long)]
        secret_key: String,
    },

    /// Run adversarial tests (SPEAR)
    Spear {
        /// Path to WASM file to deploy and test
        #[arg(short, long)]
        wasm: String,

        /// Source account secret key (S...)
        #[arg(short, long)]
        secret_key: String,

        /// Attack vectors to test (comma-separated or "all")
        #[arg(short, long, default_value = "all")]
        attacks: String,

        /// Max transactions per vector
        #[arg(long, default_value = "10")]
        max_tx: u32,
    },

    /// Start dashboard server
    Dashboard {
        /// Dashboard port
        #[arg(short, long, default_value = "8501")]
        port: u16,
    },

    /// Configure settings
    Config {
        /// Show current config
        #[arg(short, long)]
        show: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose {
        "stellarvallum=debug"
    } else {
        "stellarvallum=info"
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();

    info!("StellarVallum v0.2.0-testnet — The First Wall for Soroban");

    // Load configuration
    let cfg = config::load(&cli.config).await?;

    // Validate network
    if cli.network != "testnet" {
        warn!("Only testnet is supported in Beta. Forcing testnet.");
    }

    match cli.command {
        Commands::Scan {
            path,
            profile,
            mode,
            output,
            out_file,
        } => {
            shield::scan(&cfg, &path, &profile, &mode, &output, out_file.as_deref()).await?;
        }
        Commands::DeployTestnet { wasm, secret_key } => {
            network::deploy_testnet(&cfg, &wasm, &secret_key).await?;
        }
        Commands::Spear {
            wasm,
            secret_key,
            attacks,
            max_tx,
        } => {
            spear::run(&cfg, &wasm, &secret_key, &attacks, max_tx).await?;
        }
        Commands::Dashboard { port } => {
            dashboard::start(&cfg, port).await?;
        }
        Commands::Config { show } => {
            if show {
                config::display(&cfg)?;
            } else {
                config::setup().await?;
            }
        }
    }

    Ok(())
}
