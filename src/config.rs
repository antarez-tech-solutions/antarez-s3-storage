//! S3 client configuration and backend selection.

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Storage backend type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BackendType {
    /// AWS S3.
    Aws,
    /// MinIO or any S3-compatible server.
    Minio,
}

impl Default for BackendType {
    fn default() -> Self {
        Self::Aws
    }
}

/// Configuration for the S3 storage client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    /// S3 bucket name.
    pub bucket: String,

    /// AWS region (e.g., "us-east-1").
    pub region: String,

    /// Storage backend type.
    #[serde(default)]
    pub backend: BackendType,

    /// Custom endpoint URL (required for MinIO, optional for AWS).
    /// Example: "http://localhost:9000"
    pub endpoint: Option<String>,

    /// Force path-style addressing (required for MinIO).
    /// AWS uses virtual-hosted style by default.
    #[serde(default)]
    pub force_path_style: bool,

    /// Default expiration for pre-signed URLs.
    #[serde(with = "humantime_serde", default = "default_presign_expiry")]
    pub presign_expiry: Duration,

    /// Enable server-side encryption (AES-256).
    #[serde(default = "default_true")]
    pub server_side_encryption: bool,
}

fn default_presign_expiry() -> Duration {
    Duration::from_secs(3600) // 1 hour
}

fn default_true() -> bool {
    true
}

impl S3Config {
    /// Create a config for AWS S3.
    pub fn aws(bucket: impl Into<String>, region: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            region: region.into(),
            backend: BackendType::Aws,
            endpoint: None,
            force_path_style: false,
            presign_expiry: default_presign_expiry(),
            server_side_encryption: true,
        }
    }

    /// Create a config for MinIO with server-side encryption OFF.
    ///
    /// SSE stays off by default: sending `x-amz-server-side-encryption:
    /// AES256` to a MinIO without KMS/KES configured makes uploads fail.
    /// Enable it via [`S3Config::minio_with_sse`] only when the MinIO
    /// deployment has SSE enabled (KMS-backed) — enabling the flag and
    /// enabling SSE on the deployment must land together.
    pub fn minio(
        bucket: impl Into<String>,
        endpoint: impl Into<String>,
    ) -> Self {
        Self {
            bucket: bucket.into(),
            region: "us-east-1".to_string(), // MinIO default
            backend: BackendType::Minio,
            endpoint: Some(endpoint.into()),
            force_path_style: true,
            presign_expiry: default_presign_expiry(),
            server_side_encryption: false,
        }
    }

    /// Create a config for MinIO with server-side encryption (AES-256) ON.
    ///
    /// Parity with the AWS path, which defaults SSE on. Requires the MinIO
    /// deployment to have SSE enabled (KMS/KES); otherwise uploads will
    /// be rejected.
    pub fn minio_with_sse(
        bucket: impl Into<String>,
        endpoint: impl Into<String>,
    ) -> Self {
        Self {
            server_side_encryption: true,
            ..Self::minio(bucket, endpoint)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aws_config() {
        let config = S3Config::aws("my-bucket", "us-west-2");
        assert_eq!(config.bucket, "my-bucket");
        assert_eq!(config.backend, BackendType::Aws);
        assert!(config.endpoint.is_none());
        assert!(config.server_side_encryption);
    }

    #[test]
    fn test_minio_config() {
        let config = S3Config::minio("local-bucket", "http://localhost:9000");
        assert_eq!(config.backend, BackendType::Minio);
        assert_eq!(config.endpoint.as_deref(), Some("http://localhost:9000"));
        assert!(config.force_path_style);
        assert!(!config.server_side_encryption, "MinIO default stays SSE-off");
    }

    #[test]
    fn test_minio_with_sse_config() {
        let config = S3Config::minio_with_sse("local-bucket", "http://localhost:9000");
        assert_eq!(config.backend, BackendType::Minio);
        assert_eq!(config.endpoint.as_deref(), Some("http://localhost:9000"));
        assert!(config.force_path_style);
        assert!(config.server_side_encryption, "SSE variant enables encryption");
    }

    #[test]
    fn test_json_roundtrip() {
        let config = S3Config::aws("test", "eu-west-1");
        let json = serde_json::to_string(&config).unwrap();
        let restored: S3Config = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.bucket, "test");
        assert_eq!(restored.region, "eu-west-1");
    }
}
