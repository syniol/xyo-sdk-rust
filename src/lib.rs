//! # XYO Financial Rust SDK
//!
//! Official Rust client for the [XYO Financial API](https://xyo.financial), providing
//! institutional-grade merchant transaction enrichment, async batch processing, and secure archive downloads.
//!
//! ## Features
//! - **Synchronous Single Enrichment**: [`Client::enrich_transaction`]
//! - **Asynchronous Bulk Processing**: [`Client::enrich_transactions`] & [`Client::get_enrichment_status`]
//! - **Secure Archive Extraction**: [`Client::download_enrichment_collection`] with SSRF, Zip-Slip, and decompression bomb defenses.
//! - **Resilience**: RFC 7231 / RFC 9110 rate limit handling ([`RateLimitError`]) and dynamic token rotation.
//!
//! ## Quickstart
//! ```no_run
//! use xyo_sdk::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("your-api-token", None)?;
//!     let res = client.enrich_transaction("COSTA COFFEE LONDON", "GB").await?;
//!     println!("Merchant: {} ({})", res.merchant, res.description);
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

pub mod client;
pub mod error;

pub use client::{
    Client, ClientBuilder, DownloadSecurityPolicy, EnrichTransactionCollectionResponse,
    EnrichmentRequest, EnrichmentResponse, EnrichmentStatus, RequestOptions,
};
pub use error::{ClientError, RateLimitError};
