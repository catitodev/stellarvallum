//! Secret detection — finds hardcoded credentials in any file type.

use super::heuristics::{Finding, Severity};
use regex::Regex;

/// Remove #[cfg(test)] blocks from code to avoid false positives on test fixtures.
fn strip_test_blocks(code: &str) -> String {
    if let Some(test_start) = code.find("#[cfg(test)]") {
        code[..test_start].to_string()
    } else {
        code.to_string()
    }
}

/// Detect hardcoded secrets, API keys, and private keys in source code.
pub fn detect_secrets(code: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Skip test modules — they contain synthetic patterns for testing the detector
    let analysis_code = strip_test_blocks(code);

    let patterns: Vec<(&str, &str, Severity)> = vec![
        // Stellar secret keys (S + 55 base32 chars)
        (r"S[A-Z2-7]{55}", "Stellar Secret Key (S...)", Severity::Critical),
        // Generic API keys
        (r#"(?i)(api[_-]?key|apikey)\s*[=:]\s*["'][^"']{10,}"#, "Hardcoded API Key", Severity::High),
        // OpenRouter / OpenAI keys
        (r"sk-or-[a-zA-Z0-9]{20,}", "OpenRouter API Key", Severity::Critical),
        (r"sk-[a-zA-Z0-9]{20,}", "OpenAI-style API Key", Severity::Critical),
        // AWS keys
        (r"AKIA[0-9A-Z]{16}", "AWS Access Key ID", Severity::Critical),
        // Private keys (PEM)
        (r"-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----", "Private Key (PEM)", Severity::Critical),
        // Generic secrets in assignments
        (r#"(?i)(secret|password|passwd|token)\s*[=:]\s*["'][^"']{8,}"#, "Hardcoded Secret/Password", Severity::High),
        // Hex-encoded keys (64 chars = 32 bytes)
        (r#"(?i)(private[_-]?key|secret[_-]?key)\s*[=:]\s*["'][0-9a-fA-F]{64}["']"#, "Hex Private Key", Severity::Critical),
        // JWT tokens
        (r"eyJ[a-zA-Z0-9_-]{10,}\.[a-zA-Z0-9_-]{10,}\.[a-zA-Z0-9_-]{10,}", "JWT Token", Severity::High),
        // Webhook URLs with tokens
        (r"https://hooks\.slack\.com/services/T[A-Z0-9]+/B[A-Z0-9]+/[a-zA-Z0-9]+", "Slack Webhook URL", Severity::Medium),
    ];

    for (pattern, title, severity) in &patterns {
        let re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => continue,
        };

        for mat in re.find_iter(&analysis_code) {
            let line_num = analysis_code[..mat.start()].lines().count();

            // Skip if it's in a comment or test
            let line = analysis_code.lines().nth(line_num.saturating_sub(1)).unwrap_or("");
            if line.trim_start().starts_with("//")
                || line.trim_start().starts_with('#')
                || line.contains("example")
                || line.contains("placeholder")
                || line.contains("test")
                || line.contains("${")  // Environment variable reference
            {
                continue;
            }

            findings.push(Finding {
                severity: severity.clone(),
                category: "secrets".to_string(),
                title: title.to_string(),
                description: format!(
                    "Potential secret found at line {}. Never commit secrets to source control.",
                    line_num
                ),
                location: Some(format!("{}:{}", file_path, line_num)),
                fix: Some("Move to environment variable or secret manager".to_string()),
                owasp_id: None,
            });
        }
    }

    // Check for .env file content being loaded without gitignore
    if file_path.ends_with(".env") || file_path.contains(".env.") {
        if !file_path.contains(".example") && !file_path.contains(".template") {
            findings.push(Finding {
                severity: Severity::High,
                category: "secrets".to_string(),
                title: "Environment File With Secrets".to_string(),
                description: "This .env file may contain secrets. Ensure it's in .gitignore."
                    .to_string(),
                location: Some(file_path.to_string()),
                fix: Some("Add to .gitignore and use .env.example for templates".to_string()),
                owasp_id: None,
            });
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_stellar_secret_key() {
        let code = r#"let key = "SCZANGBA5YHTNYVVV3C7CAZMCLXPILHSE2YQRCBI5VRGCO3WU66RGQ5R";"#;
        let findings = detect_secrets(code, "src/main.rs");
        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.title.contains("Stellar Secret Key")));
    }

    #[test]
    fn detects_api_key() {
        let code = r#"let api_key = "sk-or-v1-abc123def456ghi789jkl012mno345";"#;
        let findings = detect_secrets(code, "src/config.rs");
        assert!(!findings.is_empty());
    }

    #[test]
    fn ignores_env_var_references() {
        let code = r#"api_key = "${OPENROUTER_API_KEY}""#;
        let findings = detect_secrets(code, "config.toml");
        assert!(findings.is_empty());
    }

    #[test]
    fn ignores_comments() {
        let code = r#"// api_key = "sk-or-v1-abc123def456ghi789jkl012mno345""#;
        let findings = detect_secrets(code, "src/main.rs");
        assert!(findings.is_empty());
    }

    #[test]
    fn detects_private_key_pem() {
        let code = "-----BEGIN PRIVATE KEY-----\nMIIE...\n-----END PRIVATE KEY-----";
        let findings = detect_secrets(code, "certs/key.pem");
        assert!(!findings.is_empty());
    }
}
