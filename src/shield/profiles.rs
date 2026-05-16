//! Scan profiles — auto-detection of project type.

use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum ScanProfile {
    Contract, // Soroban smart contract (Rust + soroban-sdk)
    Dapp,     // Frontend dApp (JS/TS + @stellar/stellar-sdk)
    Backend,  // Backend service (Rust/JS interacting with Stellar)
    Config,   // Configuration files (TOML, YAML, .env)
    Pipeline, // CI/CD pipelines (GitHub Actions, etc.)
    Full,     // Run all profiles
}

impl ScanProfile {
    /// File extensions to scan for this profile.
    pub fn file_extensions(&self) -> Vec<&'static str> {
        match self {
            ScanProfile::Contract => vec!["rs", "toml"],
            ScanProfile::Dapp => vec!["ts", "tsx", "js", "jsx", "json"],
            ScanProfile::Backend => vec!["rs", "ts", "js", "toml", "json"],
            ScanProfile::Config => vec!["toml", "yaml", "yml", "json", "env"],
            ScanProfile::Pipeline => vec!["yml", "yaml"],
            ScanProfile::Full => vec![
                "rs", "ts", "tsx", "js", "jsx", "toml", "yaml", "yml", "json", "env",
            ],
        }
    }
}

/// Parse a profile string from CLI.
pub fn parse_profile(s: &str) -> ScanProfile {
    match s.to_lowercase().as_str() {
        "contract" => ScanProfile::Contract,
        "dapp" | "frontend" => ScanProfile::Dapp,
        "backend" | "server" | "api" => ScanProfile::Backend,
        "config" => ScanProfile::Config,
        "pipeline" | "cicd" | "ci" => ScanProfile::Pipeline,
        "full" | "all" => ScanProfile::Full,
        _ => ScanProfile::Full,
    }
}

/// Auto-detect the project type by examining files in the directory.
pub async fn detect_profile(path: &Path) -> ScanProfile {
    if path.is_file() {
        return detect_from_file(path);
    }

    // Check for Cargo.toml with soroban-sdk
    let cargo_toml = path.join("Cargo.toml");
    if cargo_toml.exists() {
        if let Ok(content) = tokio::fs::read_to_string(&cargo_toml).await {
            if content.contains("soroban-sdk") {
                return ScanProfile::Contract;
            }
            // Rust backend that interacts with Stellar
            if content.contains("stellar") || content.contains("soroban-rpc") {
                return ScanProfile::Backend;
            }
        }
    }

    // Check for package.json with Stellar SDK
    let package_json = path.join("package.json");
    if package_json.exists() {
        if let Ok(content) = tokio::fs::read_to_string(&package_json).await {
            if content.contains("@stellar/stellar-sdk") || content.contains("soroban-client") {
                // Check if it's a frontend or backend
                if content.contains("react") || content.contains("next") || content.contains("vue")
                {
                    return ScanProfile::Dapp;
                }
                return ScanProfile::Backend;
            }
        }
    }

    // Check for CI/CD files
    if path.join(".github/workflows").exists() {
        // Has CI but also other code — run full
        return ScanProfile::Full;
    }

    // Default to full scan
    ScanProfile::Full
}

fn detect_from_file(path: &Path) -> ScanProfile {
    match path.extension().and_then(|e| e.to_str()) {
        Some("rs") => ScanProfile::Contract,
        Some("ts") | Some("tsx") | Some("js") | Some("jsx") => ScanProfile::Dapp,
        Some("toml") | Some("yaml") | Some("yml") | Some("env") => ScanProfile::Config,
        _ => ScanProfile::Full,
    }
}
