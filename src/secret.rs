//! Secret<T> — Wrapper type that prevents accidental logging of sensitive values.

use serde::{Serialize, Serializer};
use std::fmt;

/// A wrapper that redacts its inner value in Debug, Display, and Serialize.
/// Use `.expose()` to access the actual value.
#[derive(Clone)]
pub struct Secret<T> {
    inner: T,
}

impl<T> Secret<T> {
    pub fn new(value: T) -> Self {
        Self { inner: value }
    }

    /// Explicitly access the secret value. Use with care.
    pub fn expose(&self) -> &T {
        &self.inner
    }
}

impl<T> fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl<T> fmt::Display for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl<T: Serialize> Serialize for Secret<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str("[REDACTED]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_is_redacted() {
        let s = Secret::new("my-api-key".to_string());
        assert_eq!(format!("{:?}", s), "[REDACTED]");
    }

    #[test]
    fn display_is_redacted() {
        let s = Secret::new("sk-secret-123");
        assert_eq!(format!("{}", s), "[REDACTED]");
    }

    #[test]
    fn expose_returns_value() {
        let s = Secret::new("the-real-value".to_string());
        assert_eq!(s.expose(), "the-real-value");
    }

    #[test]
    fn serialize_is_redacted() {
        let s = Secret::new("secret");
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "\"[REDACTED]\"");
    }
}
