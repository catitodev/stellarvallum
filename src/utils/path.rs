//! File path canonicalization and traversal prevention.

use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Cannot resolve path '{path}': {source}")]
    ResolveError { path: String, source: std::io::Error },

    #[error("Path traversal detected: '{path}' is outside allowed directory '{allowed}'")]
    TraversalDetected { path: String, allowed: String },

    #[error("Path does not exist: '{0}'")]
    NotFound(String),
}

/// Validate that a file path exists and is within the allowed base directory.
/// Returns the canonicalized absolute path.
pub fn validate(input: &str, allowed_base: &Path) -> Result<PathBuf, PathError> {
    let input_path = Path::new(input);

    // For input paths, check existence first
    if !input_path.exists() {
        return Err(PathError::NotFound(input.to_string()));
    }

    let canonical = std::fs::canonicalize(input_path).map_err(|e| PathError::ResolveError {
        path: input.to_string(),
        source: e,
    })?;

    let base_canonical =
        std::fs::canonicalize(allowed_base).map_err(|e| PathError::ResolveError {
            path: allowed_base.display().to_string(),
            source: e,
        })?;

    if !canonical.starts_with(&base_canonical) {
        return Err(PathError::TraversalDetected {
            path: canonical.display().to_string(),
            allowed: base_canonical.display().to_string(),
        });
    }

    Ok(canonical)
}

/// Validate an output path — the file doesn't need to exist yet,
/// but its parent directory must be within the allowed base.
pub fn validate_output(input: &str, allowed_base: &Path) -> Result<PathBuf, PathError> {
    let input_path = Path::new(input);

    let parent = input_path.parent().unwrap_or(Path::new("."));

    if !parent.exists() {
        std::fs::create_dir_all(parent).map_err(|e| PathError::ResolveError {
            path: parent.display().to_string(),
            source: e,
        })?;
    }

    let parent_canonical = std::fs::canonicalize(parent).map_err(|e| PathError::ResolveError {
        path: parent.display().to_string(),
        source: e,
    })?;

    let base_canonical =
        std::fs::canonicalize(allowed_base).map_err(|e| PathError::ResolveError {
            path: allowed_base.display().to_string(),
            source: e,
        })?;

    if !parent_canonical.starts_with(&base_canonical) {
        return Err(PathError::TraversalDetected {
            path: parent_canonical.display().to_string(),
            allowed: base_canonical.display().to_string(),
        });
    }

    let file_name = input_path
        .file_name()
        .ok_or_else(|| PathError::NotFound(input.to_string()))?;

    Ok(parent_canonical.join(file_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn valid_path_within_base() {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("test.wasm");
        std::fs::write(&file, b"test").unwrap();

        let result = validate(file.to_str().unwrap(), dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn nonexistent_path_rejected() {
        let dir = TempDir::new().unwrap();
        let result = validate("/nonexistent/file.wasm", dir.path());
        assert!(matches!(result, Err(PathError::NotFound(_))));
    }
}
