//! Stellar address validation using StrKey format.

#![allow(dead_code)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AddressError {
    #[error("Invalid prefix: expected 'G' (account) or 'C' (contract), got '{0}'")]
    InvalidPrefix(char),
    #[error("Invalid length: expected 56 characters, got {0}")]
    InvalidLength(usize),
    #[error("Invalid base32 encoding")]
    InvalidEncoding,
    #[error("Empty address")]
    Empty,
    #[error("Invalid secret key: must start with 'S' and be 56 characters")]
    InvalidSecretKey,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AddressType {
    Account,
    Contract,
}

/// Validate a Stellar public address (G... or C...).
pub fn validate_address(address: &str) -> Result<AddressType, AddressError> {
    if address.is_empty() {
        return Err(AddressError::Empty);
    }
    if address.len() != 56 {
        return Err(AddressError::InvalidLength(address.len()));
    }
    let first = address.chars().next().unwrap();
    let addr_type = match first {
        'G' => AddressType::Account,
        'C' => AddressType::Contract,
        c => return Err(AddressError::InvalidPrefix(c)),
    };
    validate_base32(address)?;
    Ok(addr_type)
}

/// Validate a secret key (S..., 56 chars).
pub fn validate_secret_key(key: &str) -> Result<(), AddressError> {
    if key.is_empty() || !key.starts_with('S') || key.len() != 56 {
        return Err(AddressError::InvalidSecretKey);
    }
    validate_base32(key)?;
    Ok(())
}

/// Derive public key from secret key (placeholder — needs ed25519 in production).
pub fn public_key_from_secret(secret_key: &str) -> Result<String, AddressError> {
    validate_secret_key(secret_key)?;
    // Real implementation would use ed25519-dalek to derive the public key.
    // For testnet beta, we accept the key and let the RPC handle validation.
    Ok(format!("G{}", &secret_key[1..]))
}

fn validate_base32(s: &str) -> Result<(), AddressError> {
    let valid = |c: char| c.is_ascii_uppercase() || ('2'..='7').contains(&c);
    if !s.chars().all(valid) {
        return Err(AddressError::InvalidEncoding);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_account() {
        let addr = "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7";
        assert_eq!(validate_address(addr).unwrap(), AddressType::Account);
    }

    #[test]
    fn valid_contract() {
        let addr = "CAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7";
        assert_eq!(validate_address(addr).unwrap(), AddressType::Contract);
    }

    #[test]
    fn invalid_prefix() {
        let addr = "XAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7";
        assert!(matches!(
            validate_address(addr),
            Err(AddressError::InvalidPrefix('X'))
        ));
    }

    #[test]
    fn invalid_length() {
        assert!(matches!(
            validate_address("GABC"),
            Err(AddressError::InvalidLength(4))
        ));
    }

    #[test]
    fn empty() {
        assert!(matches!(validate_address(""), Err(AddressError::Empty)));
    }
}
