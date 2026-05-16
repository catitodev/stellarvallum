//! File path canonicalization and traversal prevention.

use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Cannot resolve path '{path}': {source}")]
    ResolveError { path: String, source: std::io::Error },

    #[error("Path traversal detected: '{path}' is outside allowed directory '{allowed}'")]
    TraversalDetected { path: String, allowed: String },
}

/// Validate that a file path resolves within an allowed base directory.
/// Returns the canonicalized path on success.
pub fn validate_path(input_path: &str, allowed_base: &Path) -> Result<PathBuf, PathError> {
    let canonical = std::fs::canonicalize(input_path).map_err(|e| PathError::ResolveError {
        path: input_path.to_string(),
        source: e,
    })?;

    let allowed_canonical =
        std::fs::canonicalize(allowed_base).map_err(|e| PathError::ResolveError {
            path: allowed_base.display().to_string(),
            source: e,
        })?;

    if !canonical.starts_with(&allowed_canonical) {
        return Err(PathError::TraversalDetected {
            path: canonical.display().to_string(),
            allowed: allowed_canonical.display().to_string(),
        });
    }

    Ok(canonical)
}

/// Resolve a path without traversal check (for input files that may be anywhere).
/// Still canonicalizes to prevent symlink tricks.
pub fn resolve_path(input_path: &str) -> Result<PathBuf, PathError> {
    std::fs::canonicalize(input_path).map_err(|e| PathError::ResolveError {
        path: input_path.to_string(),
        source: e,
    })
}
