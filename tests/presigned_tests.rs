//! Unit tests for pre-signed URL key validation and config.

use antarez_s3_storage::{BackendType, S3Config};

#[test]
fn test_aws_config_defaults() {
    let config = S3Config::aws("my-bucket", "us-east-1");
    assert_eq!(config.bucket, "my-bucket");
    assert_eq!(config.region, "us-east-1");
    assert_eq!(config.backend, BackendType::Aws);
    assert!(config.server_side_encryption);
    assert!(!config.force_path_style);
    assert!(config.endpoint.is_none());
}

#[test]
fn test_minio_config_defaults() {
    let config = S3Config::minio("local-bucket", "http://localhost:9000");
    assert_eq!(config.backend, BackendType::Minio);
    assert!(config.force_path_style);
    assert_eq!(config.endpoint.as_deref(), Some("http://localhost:9000"));
    assert!(!config.server_side_encryption);
}

#[test]
fn test_config_json_roundtrip() {
    let config = S3Config::aws("test-bucket", "eu-west-1");
    let json = serde_json::to_string_pretty(&config).unwrap();

    let restored: S3Config = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.bucket, "test-bucket");
    assert_eq!(restored.region, "eu-west-1");
    assert_eq!(restored.backend, BackendType::Aws);
}

#[test]
fn test_object_meta_serialization() {
    use antarez_s3_storage::ObjectMeta;

    let meta = ObjectMeta {
        key: "documents/report.pdf".into(),
        size: 4096,
        content_type: Some("application/pdf".into()),
        last_modified: None,
        etag: Some("\"abc123\"".into()),
    };

    let json = serde_json::to_string(&meta).unwrap();
    assert!(json.contains("report.pdf"));
    assert!(json.contains("4096"));
}
