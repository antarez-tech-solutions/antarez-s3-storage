# antarez-s3-storage

A generic Rust library for S3-compatible object storage operations with pre-signed URL support.

Provides a clean, async interface for uploading, downloading, and managing objects in S3-compatible storage (AWS S3, MinIO, etc.) via pre-signed URLs. Designed to be domain-agnostic — usable by any Antarez service that needs file storage.

## Status

**Not yet implemented.** This repo reserves the crate name and establishes the project structure.

## Planned Features

- Generate pre-signed upload URLs (PUT)
- Generate pre-signed download URLs (GET)
- Object lifecycle management (delete, copy, list)
- Multipart upload support for large files
- Configurable storage backends (AWS S3, MinIO)
- Async API built on `aws-sdk-s3`

## Usage

This crate will be consumed as a git dependency:

```toml
[dependencies]
antarez-s3-storage = { git = "git@github.com:antarez-tech-solutions/antarez-s3-storage.git", branch = "main" }
```

## License

This repository and all contributions are licensed under the [LGPL 3.0](https://www.gnu.org/licenses/lgpl-3.0.html), unless otherwise specified in subdirectory LICENSE files.
