//! Integration tests against a local MinIO instance.
//!
//! Run with: cargo test --test integration_test -- --ignored
//! Requires: MinIO on localhost:9000 with minioadmin/minioadmin.

use antarez_s3_storage::{S3Client, S3Config};

fn minio_config() -> S3Config {
    S3Config::minio("test-bucket", "http://localhost:9000")
}

/// Check if MinIO is reachable.
async fn minio_available() -> bool {
    reqwest::get("http://localhost:9000/minio/health/live")
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

#[tokio::test]
#[ignore = "requires local MinIO"]
async fn test_client_creation() {
    if !minio_available().await {
        eprintln!("MinIO not available, skipping");
        return;
    }

    let config = minio_config();
    let client = S3Client::new(config).await;
    assert!(client.is_ok());
}

#[tokio::test]
#[ignore = "requires local MinIO"]
async fn test_put_get_delete_roundtrip() {
    if !minio_available().await {
        eprintln!("MinIO not available, skipping");
        return;
    }

    let client = S3Client::new(minio_config()).await.unwrap();

    // Create bucket if needed (MinIO auto-creates via the mc tool, but
    // we skip bucket creation here — assume test-bucket exists).

    let key = "integration-test/hello.txt";
    let data = b"Hello, S3!";

    // Upload
    let result = client.put_object(key, data, Some("text/plain")).await;
    assert!(result.is_ok(), "upload failed: {:?}", result.err());

    let upload = result.unwrap();
    assert_eq!(upload.key, key);
    assert_eq!(upload.size, 10);

    // Download
    let downloaded = client.get_object(key).await.unwrap();
    assert_eq!(downloaded, data);

    // Head
    let meta = client.head_object(key).await.unwrap();
    assert_eq!(meta.key, key);
    assert_eq!(meta.size, 10);

    // Delete
    client.delete_object(key).await.unwrap();

    // Verify deleted
    let result = client.get_object(key).await;
    assert!(result.is_err());
}

#[tokio::test]
#[ignore = "requires local MinIO"]
async fn test_list_objects() {
    if !minio_available().await {
        eprintln!("MinIO not available, skipping");
        return;
    }

    let client = S3Client::new(minio_config()).await.unwrap();

    // Upload a few objects
    for i in 0..3 {
        let key = format!("list-test/file-{i}.txt");
        client
            .put_object(&key, format!("content {i}").as_bytes(), None)
            .await
            .unwrap();
    }

    // List
    let result = client
        .list_objects("list-test/", Some("/"), None, None)
        .await
        .unwrap();
    assert!(result.objects.len() >= 3);

    // Cleanup
    for i in 0..3 {
        let key = format!("list-test/file-{i}.txt");
        client.delete_object(&key).await.ok();
    }
}
