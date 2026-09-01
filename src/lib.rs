//! XYO Financial Rust SDK.
//!
//! This crate provides the official Rust client for the XYO Financial API.

pub mod client;
pub mod error;

pub use client::{
    Client, ClientBuilder, DownloadSecurityPolicy, EnrichTransactionCollectionResponse,
    EnrichmentRequest, EnrichmentResponse, EnrichmentStatus, RequestOptions,
};
pub use error::{ClientError, RateLimitError};
