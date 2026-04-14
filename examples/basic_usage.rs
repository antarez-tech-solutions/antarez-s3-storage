//! Basic usage example for antarez-s3-storage.
//!
//! Run: cargo run --example basic_usage
//!
//! Requires either:
//! - AWS credentials in the environment, or
//! - MinIO running locally (set STORAGE_BACKEND=minio)

use antarez_s3_storage::{S3Client, S3Config};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = std::env::var("STORAGE_BACKEND").unwrap_or("aws".into());

    let config = if backend == "minio" {
        let endpoint =
            std::env::var("MINIO_ENDPOINT").unwrap_or("http://localhost:9000".into());
        S3Config::minio("example-bucket", endpoint)
    } else {
        let bucket = std::env::var("S3_BUCKET").expect("S3_BUCKET required");
        let region = std::env::var("AWS_REGION").unwrap_or("us-east-1".into());
        S3Config::aws(bucket, region)
    };

    println!("Connecting to {backend} backend...");
    let client = S3Client::new(config).await?;

    // Check bucket
    let exists = client.bucket_exists().await?;
    println!("Bucket '{}' exists: {exists}", client.bucket());

    // Upload
    let key = "example/hello.txt";
    let data = b"Hello from antarez-s3-storage!";
    let result = client.put_object(key, data, Some("text/plain")).await?;
    println!(
        "Uploaded: {} ({} bytes, etag: {:?})",
        result.key, result.size, result.etag
    );

    // Generate download URL
    let url = client
        .presigned_download(key, Some(Duration::from_secs(300)))
        .await?;
    println!("Download URL (5 min): {}", url.url);

    // Download
    let downloaded = client.get_object(key).await?;
    println!("Downloaded {} bytes", downloaded.len());

    // List
    let list = client.list_objects("example/", None, None, None).await?;
    println!("Objects under 'example/': {}", list.objects.len());

    // Cleanup
    client.delete_object(key).await?;
    println!("Deleted {key}");

    Ok(())
}
