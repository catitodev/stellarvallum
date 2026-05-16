//! SHIELD — Static analysis and vulnerability detection.
//! Supports multiple scan profiles for different project types.

mod heuristics;
mod profiles;
mod secrets;

use crate::config::Config;
use crate::utils::csv_sanitizer;
use heuristics::{Finding, Severity};
use profiles::ScanProfile;
use serde::Serialize;
use std::path::Path;
use tracing::info;

#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub score: u8,
    pub profile: String,
    pub files_scanned: usize,
    pub findings: Vec<Finding>,
    pub summary: String,
    pub mainnet_ready: bool,
}

/// Run a SHIELD scan on a file or directory.
pub async fn scan(
    _config: &Config,
    path: &str,
    profile: &str,
    _mode: &str,
    output_format: &str,
    out_file: Option<&str>,
) -> anyhow::Result<()> {
    let target = Path::new(path);

    // Detect profile
    let scan_profile = if profile == "auto" {
        profiles::detect_profile(target).await
    } else {
        profiles::parse_profile(profile)
    };

    info!("SHIELD scan: {} [profile: {:?}]", path, scan_profile);

    // Collect files to scan
    let files = collect_files(target, &scan_profile).await?;
    info!("Files to scan: {}", files.len());

    // Run analysis
    let mut all_findings = Vec::new();

    for file_path in &files {
        let code = match tokio::fs::read_to_string(file_path).await {
            Ok(c) => c,
            Err(_) => continue, // Skip binary/unreadable files
        };

        let relative = file_path
            .strip_prefix(path)
            .unwrap_or(file_path.as_path())
            .display()
            .to_string();

        // Run profile-specific heuristics
        let mut findings = match scan_profile {
            ScanProfile::Contract => heuristics::run_contract_heuristics(&code, &relative),
            ScanProfile::Dapp => heuristics::run_dapp_heuristics(&code, &relative),
            ScanProfile::Backend => heuristics::run_backend_heuristics(&code, &relative),
            ScanProfile::Config => heuristics::run_config_heuristics(&code, &relative),
            ScanProfile::Pipeline => heuristics::run_pipeline_heuristics(&code, &relative),
            ScanProfile::Full => {
                let mut f = heuristics::run_contract_heuristics(&code, &relative);
                f.extend(heuristics::run_backend_heuristics(&code, &relative));
                f.extend(heuristics::run_config_heuristics(&code, &relative));
                f
            }
        };

        // Always run secrets detection
        findings.extend(secrets::detect_secrets(&code, &relative));

        all_findings.extend(findings);
    }

    // Deduplicate
    all_findings.dedup_by(|a, b| a.title == b.title && a.location == b.location);

    let score = calculate_score(&all_findings);
    let mainnet_ready =
        score >= 80 && !all_findings.iter().any(|f| f.severity == Severity::Critical);

    let result = ScanResult {
        score,
        profile: format!("{:?}", scan_profile),
        files_scanned: files.len(),
        findings: all_findings,
        summary: format!(
            "{} files scanned. {} issues found. Score: {}/100.",
            files.len(),
            result_findings_count(&score),
            score
        ),
        mainnet_ready,
    };

    // Fix summary with actual count
    let result = ScanResult {
        summary: format!(
            "{} files scanned. {} issues found. Score: {}/100.",
            result.files_scanned,
            result.findings.len(),
            result.score
        ),
        ..result
    };

    // Output
    match output_format {
        "csv" => output_csv(&result, out_file).await?,
        _ => output_json(&result, out_file).await?,
    }

    // Print summary
    println!("\n🔍 SHIELD SCAN COMPLETE");
    println!("═══════════════════════════════════════");
    println!("Profile: {:?}", scan_profile);
    println!("Files scanned: {}", result.files_scanned);
    println!("Score: {}/100", result.score);
    println!("Findings: {}", result.findings.len());
    println!(
        "Mainnet Ready: {}",
        if result.mainnet_ready {
            "✅ YES"
        } else {
            "❌ NO"
        }
    );

    if !result.findings.is_empty() {
        println!();
        for f in &result.findings {
            let icon = match f.severity {
                Severity::Critical => "🔴",
                Severity::High => "🟠",
                Severity::Medium => "🟡",
                Severity::Low => "🔵",
                Severity::Info => "⚪",
            };
            println!("  {} [{:?}] {} — {}", icon, f.severity, f.title, f.description);
            if let Some(loc) = &f.location {
                println!("     at {}", loc);
            }
        }
    }

    Ok(())
}

fn calculate_score(findings: &[Finding]) -> u8 {
    let deductions: i16 = findings
        .iter()
        .map(|f| match f.severity {
            Severity::Critical => 25,
            Severity::High => 15,
            Severity::Medium => 8,
            Severity::Low => 3,
            Severity::Info => 0,
        })
        .sum();
    (100i16 - deductions).max(0).min(100) as u8
}

fn result_findings_count(_score: &u8) -> usize {
    0 // placeholder, replaced in actual result
}

async fn collect_files(
    target: &Path,
    profile: &ScanProfile,
) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();

    if target.is_file() {
        files.push(target.to_path_buf());
        return Ok(files);
    }

    // Walk directory
    let extensions = profile.file_extensions();
    walk_dir(target, &extensions, &mut files, 0, 5)?;

    Ok(files)
}

fn walk_dir(
    dir: &Path,
    extensions: &[&str],
    files: &mut Vec<std::path::PathBuf>,
    depth: usize,
    max_depth: usize,
) -> anyhow::Result<()> {
    if depth > max_depth {
        return Ok(());
    }

    let entries = std::fs::read_dir(dir)?;
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();

        // Skip hidden dirs, target, node_modules
        if name.starts_with('.') || name == "target" || name == "node_modules" {
            continue;
        }

        if path.is_dir() {
            walk_dir(&path, extensions, files, depth + 1, max_depth)?;
        } else if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy();
            if extensions.iter().any(|e| *e == ext_str.as_ref()) {
                files.push(path);
            }
        }
    }

    Ok(())
}

async fn output_json(result: &ScanResult, out_file: Option<&str>) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(result)?;
    if let Some(path) = out_file {
        tokio::fs::write(path, &json).await?;
        println!("📄 Report: {}", path);
    }
    Ok(())
}

async fn output_csv(result: &ScanResult, out_file: Option<&str>) -> anyhow::Result<()> {
    let header = "severity,category,title,description,location,owasp_id";
    let mut rows = vec![header.to_string()];
    for f in &result.findings {
        let row = csv_sanitizer::csv_row(&[
            &format!("{:?}", f.severity),
            &f.category,
            &f.title,
            &f.description,
            f.location.as_deref().unwrap_or(""),
            f.owasp_id.as_deref().unwrap_or(""),
        ]);
        rows.push(row);
    }
    let csv = rows.join("\n");
    if let Some(path) = out_file {
        tokio::fs::write(path, &csv).await?;
        println!("📄 CSV: {}", path);
    } else {
        println!("{}", csv);
    }
    Ok(())
}
