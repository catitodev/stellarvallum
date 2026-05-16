//! Heuristic engine — Multi-profile vulnerability detection.

use regex::Regex;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub severity: Severity,
    pub category: String,
    pub title: String,
    pub description: String,
    pub location: Option<String>,
    pub fix: Option<String>,
    pub owasp_id: Option<String>,
}

// ═══════════════════════════════════════════════════════════════
// PROFILE: CONTRACT (Soroban smart contracts)
// ═══════════════════════════════════════════════════════════════

pub fn run_contract_heuristics(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();
    f.extend(check_access_control(code, file));
    f.extend(check_input_validation(code, file));
    f.extend(check_arithmetic(code, file));
    f.extend(check_reentrancy(code, file));
    f.extend(check_upgradeability(code, file));
    f.extend(check_ttl_archival(code, file));
    f.extend(check_resource_exhaustion(code, file));
    f.extend(check_error_handling(code, file));
    f.extend(check_unchecked_calls(code, file));
    f
}

// ═══════════════════════════════════════════════════════════════
// PROFILE: DAPP (Frontend applications)
// ═══════════════════════════════════════════════════════════════

pub fn run_dapp_heuristics(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();

    // Hardcoded RPC URLs (should be env vars)
    if code.contains("soroban-testnet.stellar.org") || code.contains("horizon-testnet.stellar.org") {
        if !code.contains("process.env") && !code.contains("import.meta.env") {
            f.push(Finding {
                severity: Severity::Medium,
                category: "dapp_config".into(),
                title: "Hardcoded RPC URL".into(),
                description: "RPC/Horizon URL hardcoded — use environment variables".into(),
                location: Some(file.to_string()),
                fix: Some("Use process.env.NEXT_PUBLIC_RPC_URL or similar".into()),
                owasp_id: None,
            });
        }
    }

    // Contract IDs hardcoded (should be configurable)
    let contract_re = Regex::new(r"C[A-Z2-7]{55}").unwrap();
    if contract_re.is_match(code) && !code.contains("process.env") && !code.contains("import.meta.env") {
        f.push(Finding {
            severity: Severity::Low,
            category: "dapp_config".into(),
            title: "Hardcoded Contract ID".into(),
            description: "Contract ID hardcoded — consider making configurable for multi-network".into(),
            location: Some(file.to_string()),
            fix: Some("Use environment variable for contract IDs".into()),
            owasp_id: None,
        });
    }

    // Missing error handling on wallet connection
    if code.contains("signTransaction") && !code.contains("catch") && !code.contains("try") {
        f.push(Finding {
            severity: Severity::Medium,
            category: "dapp_reliability".into(),
            title: "Unhandled Wallet Error".into(),
            description: "signTransaction called without try/catch error handling".into(),
            location: Some(file.to_string()),
            fix: Some("Wrap wallet interactions in try/catch".into()),
            owasp_id: None,
        });
    }

    // localStorage for sensitive data
    if code.contains("localStorage.setItem") && (code.contains("key") || code.contains("secret") || code.contains("token")) {
        f.push(Finding {
            severity: Severity::High,
            category: "dapp_security".into(),
            title: "Sensitive Data in localStorage".into(),
            description: "Storing potentially sensitive data in localStorage (accessible via XSS)".into(),
            location: Some(file.to_string()),
            fix: Some("Use secure session storage or avoid storing secrets client-side".into()),
            owasp_id: None,
        });
    }

    f
}

// ═══════════════════════════════════════════════════════════════
// PROFILE: BACKEND (Server-side Stellar interaction)
// ═══════════════════════════════════════════════════════════════

pub fn run_backend_heuristics(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();

    // Only check source code files, not config/manifest files
    let is_source = file.ends_with(".rs") || file.ends_with(".ts") || file.ends_with(".js");
    if !is_source {
        return f;
    }

    // HTTP without TLS for Stellar endpoints
    let http_re = Regex::new(r#"http://[^"'\s]*(stellar|soroban|horizon)"#).unwrap();
    if http_re.is_match(code) {
        f.push(Finding {
            severity: Severity::High,
            category: "backend_security".into(),
            title: "HTTP Without TLS".into(),
            description: "Stellar endpoint accessed over plain HTTP — use HTTPS".into(),
            location: Some(file.to_string()),
            fix: Some("Change http:// to https://".into()),
            owasp_id: None,
        });
    }

    // Missing rate limiting patterns
    if (code.contains("axum") || code.contains("actix") || code.contains("rocket"))
        && !code.contains("rate_limit")
        && !code.contains("RateLimit")
        && !code.contains("tower::limit")
        && !code.contains("RequestBodyLimit")
        && !code.contains("ServiceBuilder")
        && !code.contains("127.0.0.1") // localhost-only is a valid mitigation
    {
        f.push(Finding {
            severity: Severity::Medium,
            category: "backend_security".into(),
            title: "No Rate Limiting".into(),
            description: "Web server without visible rate limiting configuration".into(),
            location: Some(file.to_string()),
            fix: Some("Add tower::limit::RateLimitLayer or equivalent".into()),
            owasp_id: None,
        });
    }

    // Secret key used directly (not from env)
    if code.contains("Keypair::from_secret") || code.contains("from_secret_key") {
        let has_env = code.contains("std::env::var") || code.contains("env::var") || code.contains("dotenv");
        if !has_env {
            f.push(Finding {
                severity: Severity::High,
                category: "backend_security".into(),
                title: "Secret Key Without Env Var".into(),
                description: "Secret key loaded without environment variable — may be hardcoded".into(),
                location: Some(file.to_string()),
                fix: Some("Load secret keys from environment variables only".into()),
                owasp_id: None,
            });
        }
    }

    f
}

// ═══════════════════════════════════════════════════════════════
// PROFILE: CONFIG (Configuration files)
// ═══════════════════════════════════════════════════════════════

pub fn run_config_heuristics(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();

    // Actual values in config (not env var references)
    if file.ends_with(".toml") || file.ends_with(".yaml") || file.ends_with(".yml") {
        // Check for actual secret values (not ${VAR} references)
        let key_value_re = Regex::new(r#"(?i)(api_key|secret|password|token)\s*[=:]\s*["']([^$][^"']+)["']"#).unwrap();
        for cap in key_value_re.captures_iter(code) {
            let value = &cap[2];
            if value.len() > 5 && !value.contains("example") && !value.contains("changeme") {
                f.push(Finding {
                    severity: Severity::High,
                    category: "config_security".into(),
                    title: "Secret Value in Config File".into(),
                    description: format!("Config file contains what appears to be a real secret value for '{}'", &cap[1]),
                    location: Some(file.to_string()),
                    fix: Some("Use ${ENV_VAR} reference or move to .env file".into()),
                    owasp_id: None,
                });
            }
        }
    }

    // Mainnet passphrase in testnet config
    if code.contains("Public Global Stellar Network ; September 2015") && file.contains("testnet") {
        f.push(Finding {
            severity: Severity::Critical,
            category: "config_security".into(),
            title: "Mainnet Passphrase in Testnet Config".into(),
            description: "Mainnet network passphrase found in what appears to be testnet configuration".into(),
            location: Some(file.to_string()),
            fix: Some("Use 'Test SDF Network ; September 2015' for testnet".into()),
            owasp_id: None,
        });
    }

    f
}

// ═══════════════════════════════════════════════════════════════
// PROFILE: PIPELINE (CI/CD)
// ═══════════════════════════════════════════════════════════════

pub fn run_pipeline_heuristics(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();

    // Secrets in plain text in workflow files
    if code.contains("PRIVATE_KEY:") || code.contains("SECRET_KEY:") {
        if !code.contains("${{") {
            f.push(Finding {
                severity: Severity::Critical,
                category: "pipeline_security".into(),
                title: "Secret in Pipeline Config".into(),
                description: "Secret appears to be hardcoded in CI/CD config instead of using ${{ secrets.* }}".into(),
                location: Some(file.to_string()),
                fix: Some("Use ${{ secrets.YOUR_SECRET }} for sensitive values".into()),
                owasp_id: None,
            });
        }
    }

    // Deploy without approval gate
    if code.contains("deploy") && !code.contains("environment:") && !code.contains("approval") {
        f.push(Finding {
            severity: Severity::Medium,
            category: "pipeline_security".into(),
            title: "Deploy Without Approval Gate".into(),
            description: "Deployment step without environment protection or approval requirement".into(),
            location: Some(file.to_string()),
            fix: Some("Add 'environment: production' with required reviewers".into()),
            owasp_id: None,
        });
    }

    f
}

// ═══════════════════════════════════════════════════════════════
// CONTRACT HEURISTICS (detailed implementations)
// ═══════════════════════════════════════════════════════════════

fn check_access_control(code: &str, file: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let privileged = ["transfer", "mint", "burn", "admin", "upgrade", "set_admin", "withdraw", "pause", "unpause"];
    let fn_re = Regex::new(r"(?m)^\s*pub\s+fn\s+(\w+)").unwrap();

    for cap in fn_re.captures_iter(code) {
        let fn_name = &cap[1];
        let fn_start = cap.get(0).unwrap().start();
        if !privileged.iter().any(|p| fn_name.contains(p)) { continue; }

        let fn_body = get_fn_body(code, fn_start);
        if !fn_body.contains("require_auth") {
            let line = code[..fn_start].lines().count();
            findings.push(Finding {
                severity: Severity::Critical, category: "access_control".into(),
                title: "Missing Authorization".into(),
                description: format!("'{}' lacks require_auth", fn_name),
                location: Some(format!("{}:{}:{}", file, fn_name, line)),
                fix: Some("Add address.require_auth()".into()),
                owasp_id: Some("SC01".into()),
            });
        }
    }
    findings
}

fn check_input_validation(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();
    if code.contains("from_val") && !code.contains("try_from_val") {
        f.push(Finding {
            severity: Severity::High, category: "input_validation".into(),
            title: "Unsafe Val Conversion".into(), description: "from_val without try_from_val".into(),
            location: Some(file.to_string()), fix: Some("Use try_from_val".into()),
            owasp_id: Some("SC05".into()),
        });
    }
    if (code.contains("Vec<") || code.contains("Map<")) && !code.contains(".len()") && code.contains("pub fn") {
        f.push(Finding {
            severity: Severity::Medium, category: "input_validation".into(),
            title: "Unbounded Collection".into(), description: "Vec/Map without length check".into(),
            location: Some(file.to_string()), fix: Some("Add length validation".into()),
            owasp_id: Some("SC05".into()),
        });
    }
    f
}

fn check_arithmetic(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();
    let has_arith = code.contains(" + ") || code.contains(" - ") || code.contains(" * ") || code.contains(" / ");
    let has_checked = code.contains("checked_") || code.contains("saturating_");
    if has_arith && !has_checked {
        f.push(Finding {
            severity: Severity::Medium, category: "arithmetic_error".into(),
            title: "Unchecked Arithmetic".into(), description: "No checked/saturating ops".into(),
            location: Some(file.to_string()), fix: Some("Use checked_add/sub/mul/div".into()),
            owasp_id: Some("SC07".into()),
        });
    }
    if code.contains(" / ") && !code.contains("!= 0") && !code.contains("> 0") {
        f.push(Finding {
            severity: Severity::High, category: "arithmetic_error".into(),
            title: "Division Without Zero Check".into(), description: "No zero-divisor check".into(),
            location: Some(file.to_string()), fix: Some("Check divisor != 0".into()),
            owasp_id: Some("SC07".into()),
        });
    }
    f
}

fn check_reentrancy(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();
    let fn_re = Regex::new(r"(?m)^\s*pub\s+fn\s+(\w+)").unwrap();
    for cap in fn_re.captures_iter(code) {
        let fn_name = &cap[1];
        let fn_start = cap.get(0).unwrap().start();
        let body = get_fn_body(code, fn_start);
        if let Some(pos) = body.find("invoke_contract") {
            let after = &body[pos..];
            if after.contains("storage()") || after.contains(".set(") {
                let line = code[..fn_start].lines().count();
                f.push(Finding {
                    severity: Severity::Critical, category: "reentrancy".into(),
                    title: "CEI Violation".into(),
                    description: format!("'{}' modifies state after invoke_contract", fn_name),
                    location: Some(format!("{}:{}:{}", file, fn_name, line)),
                    fix: Some("Move state changes before external calls".into()),
                    owasp_id: Some("SC08".into()),
                });
            }
        }
    }
    f
}

fn check_upgradeability(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();
    if !code.contains("update_current_contract_wasm") { return f; }
    let fn_re = Regex::new(r"(?m)^\s*pub\s+fn\s+(\w+)").unwrap();
    for cap in fn_re.captures_iter(code) {
        let fn_name = &cap[1];
        let fn_start = cap.get(0).unwrap().start();
        let body = get_fn_body(code, fn_start);
        if body.contains("update_current_contract_wasm") && !body.contains("require_auth") {
            let line = code[..fn_start].lines().count();
            f.push(Finding {
                severity: Severity::Critical, category: "upgradeability".into(),
                title: "Unprotected Upgrade".into(),
                description: format!("'{}' upgrades without auth", fn_name),
                location: Some(format!("{}:{}:{}", file, fn_name, line)),
                fix: Some("Add admin.require_auth() before upgrade".into()),
                owasp_id: Some("SC10".into()),
            });
        }
    }
    f
}

fn check_ttl_archival(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();
    let has_ttl = code.contains("extend_ttl") || code.contains("bump(");
    if code.contains("persistent()") && !has_ttl {
        f.push(Finding {
            severity: Severity::High, category: "ttl_archival".into(),
            title: "No TTL Extension".into(), description: "Persistent storage without extend_ttl".into(),
            location: Some(file.to_string()), fix: Some("Call extend_ttl".into()), owasp_id: None,
        });
    }
    if code.contains("instance()") && !has_ttl {
        f.push(Finding {
            severity: Severity::Medium, category: "ttl_archival".into(),
            title: "Instance Without TTL".into(), description: "Instance may be archived".into(),
            location: Some(file.to_string()), fix: Some("Extend instance TTL".into()), owasp_id: None,
        });
    }
    f
}

fn check_resource_exhaustion(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();
    let loop_re = Regex::new(r"(?m)(loop\s*\{|while\s+[^{]*\{)").unwrap();
    for cap in loop_re.captures_iter(code) {
        let start = cap.get(0).unwrap().start();
        let ctx = &code[start..std::cmp::min(start + 200, code.len())];
        if !ctx.contains("break") && !ctx.contains("< ") && !ctx.contains("MAX") {
            let line = code[..start].lines().count();
            f.push(Finding {
                severity: Severity::High, category: "resource_exhaustion".into(),
                title: "Unbounded Loop".into(), description: "Loop without termination limit".into(),
                location: Some(format!("{}:line:{}", file, line)),
                fix: Some("Add iteration limit".into()), owasp_id: None,
            });
        }
    }
    f
}

fn check_error_handling(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();
    let re = Regex::new(r"panic!\s*\(").unwrap();
    for mat in re.find_iter(code) {
        if !code[mat.start()..].starts_with("panic_with_error!") {
            let line = code[..mat.start()].lines().count();
            f.push(Finding {
                severity: Severity::High, category: "error_handling".into(),
                title: "Improper panic!".into(), description: "Use panic_with_error! instead".into(),
                location: Some(format!("{}:line:{}", file, line)),
                fix: Some("Replace with panic_with_error!".into()), owasp_id: None,
            });
        }
    }
    f
}

fn check_unchecked_calls(code: &str, file: &str) -> Vec<Finding> {
    let mut f = Vec::new();
    if code.contains("invoke_contract") && !code.contains("match ") && !code.contains("if let ") {
        f.push(Finding {
            severity: Severity::High, category: "unchecked_external_call".into(),
            title: "Unchecked External Call".into(), description: "invoke_contract result not handled".into(),
            location: Some(file.to_string()), fix: Some("Handle with match or if-let".into()),
            owasp_id: Some("SC06".into()),
        });
    }
    f
}

fn get_fn_body(code: &str, start: usize) -> &str {
    let rest = &code[start..];
    if let Some(next) = rest[1..].find("pub fn") {
        &rest[..next + 1]
    } else {
        rest
    }
}
