//! Pre-signed URL generation for uploads and downloads.
//!
//! Pre-signed URLs allow clients to upload or download objects directly
//! to/from S3 without needing AWS credentials. The URL is valid for a
//! limited time.

use std::time::Duration;

use aws_sdk_s3::presigning::PresigningConfig;
use chrono::Utc;

use crate::client::S3Client;
use crate::error::{PresignError, S3Error};
use crate::types::PresignedUrl;

impl S3Client {
    /// Generate a pre-signed URL for downloading (GET) an object.
    ///
    /// # Arguments
    /// * `key` — Object key (path within the bucket).
    /// * `expires_in` — How long the URL should be valid. If `None`,
    ///   uses the default from config.
    ///
    /// # Errors
    /// Returns `S3Error::Presign` if the key is empty or presigning fails.
    pub async fn presigned_download(
        &self,
        key: &str,
        expires_in: Option<Duration>,
    ) -> Result<PresignedUrl, S3Error> {
        validate_key(key)?;

        let expiry = expires_in.unwrap_or(self.config.presign_expiry);
        let presign_config = PresigningConfig::expires_in(expiry)
            .map_err(|e| S3Error::Presign(PresignError::InvalidExpiration(e.to_string())))?;

        let presigned = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presign_config)
            .await
            .map_err(|e| S3Error::Presign(PresignError::PresignFailed(e.to_string())))?;

        Ok(PresignedUrl {
            url: presigned.uri().to_string(),
            method: "GET".to_string(),
            key: key.to_string(),
            expires_at: Utc::now() + chrono::Duration::from_std(expiry).unwrap_or_default(),
        })
    }

    /// Generate a pre-signed URL for uploading (PUT) an object.
    ///
    /// # Arguments
    /// * `key` — Object key (path within the bucket).
    /// * `content_type` — Optional MIME type (e.g., "application/pdf").
    /// * `expires_in` — How long the URL should be valid. If `None`,
    ///   uses the default from config.
    ///
    /// # Errors
    /// Returns `S3Error::Presign` if the key is empty or presigning fails.
    pub async fn presigned_upload(
        &self,
        key: &str,
        content_type: Option<&str>,
        expires_in: Option<Duration>,
    ) -> Result<PresignedUrl, S3Error> {
        validate_key(key)?;

        let expiry = expires_in.unwrap_or(self.config.presign_expiry);
        let presign_config = PresigningConfig::expires_in(expiry)
            .map_err(|e| S3Error::Presign(PresignError::InvalidExpiration(e.to_string())))?;

        let mut builder = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key);

        if let Some(ct) = content_type {
            builder = builder.content_type(ct);
        }

        // Apply server-side encryption if configured
        if self.config.server_side_encryption {
            builder = builder.server_side_encryption(
                aws_sdk_s3::types::ServerSideEncryption::Aes256,
            );
        }

        let presigned = builder
            .presigned(presign_config)
            .await
            .map_err(|e| S3Error::Presign(PresignError::PresignFailed(e.to_string())))?;

        Ok(PresignedUrl {
            url: presigned.uri().to_string(),
            method: "PUT".to_string(),
            key: key.to_string(),
            expires_at: Utc::now() + chrono::Duration::from_std(expiry).unwrap_or_default(),
        })
    }

    /// Generate a pre-signed URL for deleting an object.
    ///
    /// Less common but useful for client-side deletion flows.
    pub async fn presigned_delete(
        &self,
        key: &str,
        expires_in: Option<Duration>,
    ) -> Result<PresignedUrl, S3Error> {
        validate_key(key)?;

        let expiry = expires_in.unwrap_or(self.config.presign_expiry);
        let presign_config = PresigningConfig::expires_in(expiry)
            .map_err(|e| S3Error::Presign(PresignError::InvalidExpiration(e.to_string())))?;

        let presigned = self
            .client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presign_config)
            .await
            .map_err(|e| S3Error::Presign(PresignError::PresignFailed(e.to_string())))?;

        Ok(PresignedUrl {
            url: presigned.uri().to_string(),
            method: "DELETE".to_string(),
            key: key.to_string(),
            expires_at: Utc::now() + chrono::Duration::from_std(expiry).unwrap_or_default(),
        })
    }
}

/// Validate that the object key is not empty and doesn't contain problematic characters.
fn validate_key(key: &str) -> Result<(), S3Error> {
    if key.is_empty() {
        return Err(S3Error::Presign(PresignError::InvalidKey(
            "key cannot be empty".into(),
        )));
    }
    if key.starts_with('/') {
        return Err(S3Error::Presign(PresignError::InvalidKey(
            "key must not start with '/'".into(),
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_key_empty() {
        assert!(validate_key("").is_err());
    }

    #[test]
    fn test_validate_key_leading_slash() {
        assert!(validate_key("/bad/key").is_err());
    }

    #[test]
    fn test_validate_key_ok() {
        assert!(validate_key("documents/file.pdf").is_ok());
        assert!(validate_key("a").is_ok());
    }
}
