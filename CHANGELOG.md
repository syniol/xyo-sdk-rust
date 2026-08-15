# 📝 Changelog

All notable changes to the XYO Financial SDK for Rust will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **ClientBuilder & Configuration (`Client::builder`)**: Introduced fluent `ClientBuilder` for flexible configuration of base URLs, HTTP timeouts, connection pool settings, and custom `reqwest::Client` instances ([#21](https://github.com/xyo-financial/sdk-rust/pull/21)).
- **Dynamic Token Rotation (`Client::with_token_supplier`)**: Added token supplier callback (`Fn() -> String + Send + Sync`) supporting runtime secret and API key rotation for enterprise key vaults without client reinitialization ([#20](https://github.com/xyo-financial/sdk-rust/pull/20)).
- **Structured Tracing Telemetry**: Integrated `tracing` instrumentation with structured debug spans and logs for API requests, batch submissions, and status polling workflows ([#22](https://github.com/xyo-financial/sdk-rust/pull/22)).
- **Error Classification Helpers**: Added programmatic error inspection methods to `ClientError` (`is_auth()`, `is_rate_limited()`, `is_server_error()`, `is_retryable()`, `is_not_found()`) for resilient error handling ([#22](https://github.com/xyo-financial/sdk-rust/pull/22)).
- **Download Security Policy (`DownloadSecurityPolicy`)**: Configurable multi-cloud security policy for `download_enrichment_collection` supporting custom allowlists and trusted origins ([#21](https://github.com/xyo-financial/sdk-rust/pull/21), [#23](https://github.com/xyo-financial/sdk-rust/pull/23)).
- **Client-Side Validation (`EnrichmentRequest::validate`)**: Added pre-flight input validation enforcing ISO 3166-1 alpha-2 country codes, non-empty payloads, and character length limits prior to network requests ([#20](https://github.com/xyo-financial/sdk-rust/pull/20), [#24](https://github.com/xyo-financial/sdk-rust/pull/24)).
- **Chained Release Pipeline Workflow (`.github/workflows/release.yml`)**: Automated GitHub Release workflow triggered on successful crates.io publishing (`workflow_run`), creating GitHub Releases with tar.gz archives, SHA-256 checksums, SPDX SBOM generation (`anchore/sbom-action`), GitHub build provenance attestations (`actions/attest-build-provenance`), and example verification ([#25](https://github.com/xyo-financial/sdk-rust/pull/25)).

### Changed
- **Zero Panic Vectors**: Refactored `Client::new` and `Client::with_token_supplier` to return `Result<Self, ClientError>`, eliminating all panic vectors (`unwrap` / `expect`) from library construction ([#24](https://github.com/xyo-financial/sdk-rust/pull/24)).
- **Async Archive Decompression Offloading**: Offloaded CPU-intensive gzip decompression, tar entry parsing, and JSON deserialization in `download_enrichment_collection` to `tokio::task::spawn_blocking` to prevent async runtime worker thread starvation ([#21](https://github.com/xyo-financial/sdk-rust/pull/21)).
- **Bounded Stream Archive Downloads**: Replaced unbounded buffer reads with upfront `Content-Length` checks and bounded streaming chunks to protect against out-of-memory exhaustion ([#21](https://github.com/xyo-financial/sdk-rust/pull/21)).
- **Header Serialization**: Standardized raw header dispatch for `x-api-user` header unwrapping across transaction enrichment and polling endpoints ([#21](https://github.com/xyo-financial/sdk-rust/pull/21), [#23](https://github.com/xyo-financial/sdk-rust/pull/23)).
- **Safe Debug Formatting**: Implemented `std::fmt::Debug` for `Client` and `ClientBuilder` with automatic API token redaction (`[REDACTED]`) to prevent credential leakage in logs ([#21](https://github.com/xyo-financial/sdk-rust/pull/21)).
- **Thread-Safe Test Harness**: Replaced process-global `std::env::set_var` test mutations with thread-safe harness testing, removing mutexes and guaranteeing safety across multi-threaded test runners under modern Rust compilers ([#24](https://github.com/xyo-financial/sdk-rust/pull/24)).
- **Dependency Hygiene**: Relaxed rigid pinned (`=`) dependency versions in `Cargo.toml` to standard SemVer ranges and removed unused blocking reqwest dependencies in OpenAPI client ([#21](https://github.com/xyo-financial/sdk-rust/pull/21), [#24](https://github.com/xyo-financial/sdk-rust/pull/24)).
- **License Documentation**: Aligned `LICENSE` with canonical Apache-2.0 text for standard license scanner compliance ([399bbf8](https://github.com/xyo-financial/sdk-rust/commit/399bbf8af72e782b9d561a5f41c735812443d54f)).

### Security
- **Strict Zero-Trust Domain Allowlist**: Enforced strict domain whitelisting on bulk archive downloads defaulting to official XYO domains (`api.xyo.financial`, `download.xyo.financial`) and permitted origin allowlists ([#18](https://github.com/xyo-financial/sdk-rust/pull/18), [#21](https://github.com/xyo-financial/sdk-rust/pull/21), [#23](https://github.com/xyo-financial/sdk-rust/pull/23)).
- **SSRF Protocol & Host Isolation**: Restricted URL schemes in archive downloads to `http`/`https` and isolated Bearer authorization headers to prevent credential leakage when downloading from external storage or CDNs ([#17](https://github.com/xyo-financial/sdk-rust/pull/17)).
- **Decompression Bomb & Zip-Slip Defenses**: Implemented strict limits on decompression archives (`DEFAULT_MAX_TAR_ENTRIES` 50,000, `DEFAULT_MAX_ENTRY_BYTES` 10 MiB, `DEFAULT_MAX_ARCHIVE_BYTES` 100 MiB) and path traversal checks ([#17](https://github.com/xyo-financial/sdk-rust/pull/17)).
- **HTTP Header Injection Defense (CWE-113)**: Validated `api_user` parameter against CRLF control characters (`\r`, `\n`) to prevent HTTP response splitting / header injection ([#20](https://github.com/xyo-financial/sdk-rust/pull/20)).
- **Log Injection & Error Sanitization (CWE-117 / CWE-209)**: Sanitized control characters in archive filenames for error logs and eliminated HTTP response body previews on unexpected Content-Types to prevent SSRF data exfiltration ([#17](https://github.com/xyo-financial/sdk-rust/pull/17), [#18](https://github.com/xyo-financial/sdk-rust/pull/18)).
- **WAF Security Challenge Diagnostics**: Added explicit detection and actionable error reporting when responses return HTML/WAF challenge pages instead of expected archive payloads ([#17](https://github.com/xyo-financial/sdk-rust/pull/17)).

### Fixed
- **Empty Batch Request Defense**: Prevented network requests with empty batch arrays in `enrich_transactions` with upfront validation error ([#24](https://github.com/xyo-financial/sdk-rust/pull/24)).
- **OpenAPI Header Double-Quoting**: Fixed quotation bug where JSON strings wrapped headers in OpenAPI client dispatch ([#21](https://github.com/xyo-financial/sdk-rust/pull/21)).
- **Null-Safe Deserialization**: Added custom deserializer handling `null` string values as empty strings for nullable response fields ([#21](https://github.com/xyo-financial/sdk-rust/pull/21)).

## [2.0.0] - 2026-08-12

### Added
- **Async Tokio Client**: High-level asynchronous `Client` powered by `tokio` and `reqwest` with retry policy, rate limiting, and exponential backoff.
- **OpenAPI Integration**: Added `xyo-openapi-client` crate generated from official OpenAPI specifications.
- **WireMock Test Suite**: Comprehensive mock integration test suite covering API interactions.
- **Examples**: Added runnable examples for quickstart, bulk enrichment, and error handling.

### Changed
- **Package Renaming**: Renamed generated OpenAPI client to `xyo-openapi-client` (v2.0.0) in `openapi/Cargo.toml` and updated workspace dependency to resolve crates.io name collision.
- **Documentation**: Updated `README.md`, `CONTRIBUTING.md`, and example applications for v2.0.0.
- **Licensing**: Relicensed repository under Apache-2.0.
- **Version Bump**: Incremented `xyo-sdk` version to `2.0.0`.

### Removed
- **Legacy Synchronous Client**: Removed `xyo-http` crate in favor of the unified async SDK architecture.

## [1.1.9] - 2026-08-07

### Changed
- **Repository Migration**: Updated repository URLs from `https://github.com/syniol/xyo-sdk-rust` to `https://github.com/xyo-financial/sdk-rust` in `Cargo.toml`, `xyo-http/Cargo.toml`, and `README.md`.
- **CI & Assets**: Updated workflow status badges and mascot image links in `README.md` to point to the new organization repository.
- **Version Bump**: Incremented `xyo-sdk` and `xyo-http` versions to `1.1.9` in `Cargo.toml` and `xyo-http/Cargo.toml`.

## [1.1.8] - 2026-07-20

### Changed
- **Relicensed to BSD-3-Clause**: Updated project license terms to BSD 3-Clause in `LICENSE`, `Cargo.toml`, and documentation.
- **Publish Workflow**: Refined release tag push filter pattern to `v[0-9]+.[0-9]+.[0-9]+` in `.github/workflows/crates_xyo_http_publish.yml`.
- **Version Bump**: Incremented `xyo-sdk` and `xyo-http` versions to `1.1.8` in `Cargo.toml` and `xyo-http/Cargo.toml`.

[Unreleased]: https://github.com/xyo-financial/sdk-rust/compare/v2.0.0...HEAD
[2.0.0]: https://github.com/xyo-financial/sdk-rust/compare/v1.1.9...v2.0.0
[1.1.9]: https://github.com/xyo-financial/sdk-rust/compare/v1.1.8...v1.1.9
[1.1.8]: https://github.com/xyo-financial/sdk-rust/releases/tag/v1.1.8
