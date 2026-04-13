//! Error types for S3 operations.

use thiserror::Error;

/// Top-level error type for all S3 storage operations.
#[derive(Debug, Error)]
pub enum S3Error {
    /// Configuration error.
    #[error("config error: {0}")]
    Config(#[from] ConfigError),

    /// Client connection or initialization error.
    #[error("client error: {0}")]
    Client(#[from] ClientError),

    /// Pre-signed URL generation error.
    #[error("presign error: {0}")]
    Presign(#[from] PresignError),

    /// Object operation error.
    #[error("operation error: {0}")]
    Operation(#[from] OperationError),
}

/// Configuration-related errors.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Invalid endpoint URL.
    #[error("invalid endpoint URL: {0}")]
    InvalidEndpoint(String),

    /// Missing required configuration field.
    #[error("missing config: {0}")]
    MissingField(String),

    /// Invalid region.
    #[error("invalid region: {0}")]
    InvalidRegion(String),
}

/// Client initialization and connection errors.
#[derive(Debug, Error)]
pub enum ClientError {
    /// The target bucket does not exist or is inaccessible.
    #[error("bucket not found: {0}")]
    BucketNotFound(String),

    /// AWS SDK error during client setup.
    #[error("sdk error: {0}")]
    SdkError(String),

    /// Credential loading failed.
    #[error("credentials error: {0}")]
    CredentialsError(String),
}

/// Pre-signed URL generation errors.
#[derive(Debug, Error)]
pub enum PresignError {
    /// The requested expiration duration is invalid.
    #[error("invalid expiration: {0}")]
    InvalidExpiration(String),

    /// Presigning failed.
    #[error("presign failed: {0}")]
    PresignFailed(String),

    /// The object key is empty or invalid.
    #[error("invalid key: {0}")]
    InvalidKey(String),
}

/// Object CRUD operation errors.
#[derive(Debug, Error)]
pub enum OperationError {
    /// Object not found (404).
    #[error("object not found: {key}")]
    NotFound { key: String },

    /// Upload failed.
    #[error("upload failed for {key}: {reason}")]
    UploadFailed { key: String, reason: String },

    /// Download failed.
    #[error("download failed for {key}: {reason}")]
    DownloadFailed { key: String, reason: String },

    /// Delete failed.
    #[error("delete failed for {key}: {reason}")]
    DeleteFailed { key: String, reason: String },

    /// Copy failed.
    #[error("copy failed from {src} to {dst}: {reason}")]
    CopyFailed {
        src: String,
        dst: String,
        reason: String,
    },

    /// List failed.
    #[error("list failed for prefix {prefix}: {reason}")]
    ListFailed { prefix: String, reason: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error_display() {
        let err = S3Error::Config(ConfigError::InvalidEndpoint("bad-url".into()));
        assert!(err.to_string().contains("invalid endpoint URL"));
    }

    #[test]
    fn test_operation_not_found() {
        let err = S3Error::Operation(OperationError::NotFound {
            key: "docs/file.pdf".into(),
        });
        assert!(err.to_string().contains("docs/file.pdf"));
    }

    #[test]
    fn test_presign_error() {
        let err = S3Error::Presign(PresignError::InvalidExpiration("negative".into()));
        assert!(err.to_string().contains("negative"));
    }

    #[test]
    fn test_copy_failed_display() {
        let err = S3Error::Operation(OperationError::CopyFailed {
            src: "a/b.txt".into(),
            dst: "c/d.txt".into(),
            reason: "access denied".into(),
        });
        let msg = err.to_string();
        assert!(msg.contains("a/b.txt"));
        assert!(msg.contains("c/d.txt"));
        assert!(msg.contains("access denied"));
    }
}
