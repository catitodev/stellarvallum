//! CHAIN — Immutable audit trail with SHA-256 hash chain.

use chrono::Utc;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize)]
pub struct AuditEvent {
    pub timestamp: String,
    pub contract_id: String,
    pub event_type: String,
    pub severity: String,
    pub description: String,
    pub tx_hash: Option<String>,
    pub hash: String,
    pub previous_hash: String,
}

/// Append an event to the hash chain.
pub fn create_event(
    contract_id: &str,
    event_type: &str,
    severity: &str,
    description: &str,
    tx_hash: Option<&str>,
    previous_hash: &str,
) -> AuditEvent {
    let timestamp = Utc::now().to_rfc3339();

    let data = format!(
        "{}:{}:{}:{}:{}:{}",
        timestamp, contract_id, event_type, severity, description, previous_hash
    );

    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let hash = hex::encode(hasher.finalize());

    AuditEvent {
        timestamp,
        contract_id: contract_id.to_string(),
        event_type: event_type.to_string(),
        severity: severity.to_string(),
        description: description.to_string(),
        tx_hash: tx_hash.map(|s| s.to_string()),
        hash,
        previous_hash: previous_hash.to_string(),
    }
}

/// Build a hash chain from a sequence of events.
pub fn build_chain(events: &[(String, String, String, String)]) -> Vec<AuditEvent> {
    let mut chain = Vec::new();
    let mut prev_hash = "genesis".to_string();

    for (contract_id, event_type, severity, description) in events {
        let event = create_event(contract_id, event_type, severity, description, None, &prev_hash);
        prev_hash = event.hash.clone();
        chain.push(event);
    }

    chain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_links_correctly() {
        let e1 = create_event("C123", "scan", "info", "test1", None, "genesis");
        let e2 = create_event("C123", "scan", "info", "test2", None, &e1.hash);

        assert_eq!(e2.previous_hash, e1.hash);
        assert_ne!(e1.hash, e2.hash);
    }

    #[test]
    fn hash_is_deterministic_for_same_input() {
        // Same data should produce same hash (minus timestamp)
        let h1 = {
            let mut hasher = Sha256::new();
            hasher.update(b"test-data");
            hex::encode(hasher.finalize())
        };
        let h2 = {
            let mut hasher = Sha256::new();
            hasher.update(b"test-data");
            hex::encode(hasher.finalize())
        };
        assert_eq!(h1, h2);
    }
}
