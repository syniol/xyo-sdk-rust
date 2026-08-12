//! XYO Financial SDK – thin async wrapper over the OpenAPI-generated client.
//!
//! # Example
//! ```no_run
//! use xyo_sdk::client::{Client, EnrichmentRequest};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("your-bearer-token", None);
//!     let resp = client.enrich_transaction("COSTA PICKUP", "GB").await.unwrap();
//!     println!("{}", resp.merchant);
//! }
//! ```

use xyo_openapi_client::apis::configuration::Configuration;
use xyo_openapi_client::apis::enrichment_api;
use xyo_openapi_client::models::{EnrichmentRequest as ApiEnrichmentRequest, EnrichTransactionsRequestInner};
use serde::{Deserialize, Serialize};

use crate::error::ClientError;

// ── Re-exported response types ────────────────────────────────────────────────

/// Response from a single-transaction enrichment.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnrichmentResponse {
    #[serde(default)]
    pub merchant: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub logo: String,
    /// Empty string when the API returns null / empty.
    #[serde(default)]
    pub location: String,
    /// Empty string when the API returns null / empty.
    #[serde(default)]
    pub address: String,
}

/// Response from a bulk enrichment submission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichTransactionCollectionResponse {
    /// Work-item ID used to poll for completion.
    pub id: String,
    /// URL of the downloadable tar.gz results archive.
    pub link: String,
}

/// Processing state of a bulk enrichment job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnrichmentStatus {
    Ready,
    Pending,
    Failed,
}

/// A single transaction to submit for enrichment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichmentRequest {
    /// Payment description (max 128 chars).
    pub content: String,
    /// ISO 3166-1 alpha-2 country code (e.g. "GB").
    pub country_code: String,
}

// ── Client ────────────────────────────────────────────────────────────────────

/// Async client for the XYO Financial Transaction Enrichment API.
pub struct Client {
    configuration: Configuration,
}

impl Client {
    /// Construct a new client.
    ///
    /// * `bearer_token` – the API Bearer token.
    /// * `base_url`     – override the server URL (default: `https://api.xyo.financial`).
    pub fn new(bearer_token: impl Into<String>, base_url: Option<String>) -> Self {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(bearer_token.into());
        if let Some(url) = base_url {
            configuration.base_path = url;
        }
        Client { configuration }
    }

    // ── enrichTransaction ─────────────────────────────────────────────────────

    /// Enrich a single financial transaction synchronously.
    pub async fn enrich_transaction(
        &self,
        content: impl Into<String>,
        country_code: impl Into<String>,
    ) -> Result<EnrichmentResponse, ClientError> {
        let body = ApiEnrichmentRequest::new(content.into(), country_code.into());

        let resp = enrichment_api::enrich_transaction(&self.configuration, Some(body))
            .await
            .map_err(map_error)?;

        Ok(EnrichmentResponse {
            merchant: resp.merchant,
            description: resp.description,
            categories: resp.categories,
            logo: resp.logo,
            location: resp.location,
            address: resp.address,
        })
    }

    // ── enrichTransactions ────────────────────────────────────────────────────

    /// Enrich a collection of financial transactions asynchronously.
    ///
    /// Returns a job `id` that can be polled with [`Client::get_enrichment_status`].
    pub async fn enrich_transactions(
        &self,
        requests: impl IntoIterator<Item = EnrichmentRequest>,
        api_user: Option<&str>,
    ) -> Result<EnrichTransactionCollectionResponse, ClientError> {
        let items: Vec<EnrichTransactionsRequestInner> = requests
            .into_iter()
            .map(|r| EnrichTransactionsRequestInner {
                content: Some(r.content),
                country_code: Some(r.country_code),
            })
            .collect();

        let x_api_user = api_user.map(serde_json::Value::from);

        let resp = enrichment_api::enrich_transactions(&self.configuration, x_api_user, Some(items))
            .await
            .map_err(map_error)?;

        Ok(EnrichTransactionCollectionResponse {
            id: resp.id,
            link: resp.link,
        })
    }

    // ── getEnrichmentStatus ───────────────────────────────────────────────────

    /// Get the status of an asynchronous bulk enrichment job.
    pub async fn get_enrichment_status(
        &self,
        id: &str,
        api_user: Option<&str>,
    ) -> Result<EnrichmentStatus, ClientError> {
        let resp = enrichment_api::get_enrichment_status(&self.configuration, id, api_user)
            .await
            .map_err(map_error)?;


        use xyo_openapi_client::models::enrichment_collection_status_response::Status;
        Ok(match resp.status {
            Status::Ready => EnrichmentStatus::Ready,
            Status::Pending => EnrichmentStatus::Pending,
            Status::Failed => EnrichmentStatus::Failed,
        })
    }

    // ── downloadEnrichmentCollection ──────────────────────────────────────────

    /// Download and unpack an enrichment collection archive (`.tar.gz`) from a bulk job.
    ///
    /// Performs an HTTP GET request to `download_url` with `Authorization: Bearer <token>`
    /// and `Accept: application/gzip`, decompresses the `.tar.gz` archive using
    /// [`flate2::read::GzDecoder`], iterates tar entries with [`tar::Archive`], and parses
    /// each `.json` file entry with [`serde_json::from_reader`] into an [`EnrichmentResponse`].
    pub async fn download_enrichment_collection(
        &self,
        download_url: &str,
    ) -> Result<Vec<EnrichmentResponse>, ClientError> {
        let url = if download_url.starts_with("http://") || download_url.starts_with("https://") {
            download_url.to_string()
        } else {
            format!(
                "{}/{}",
                self.configuration.base_path.trim_end_matches('/'),
                download_url.trim_start_matches('/')
            )
        };

        let mut req_builder = self.configuration.client.get(&url);

        if let Some(ref user_agent) = self.configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent);
        }
        if let Some(ref token) = self.configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.header(reqwest::header::ACCEPT, "application/gzip");

        let resp = req_builder.send().await.map_err(|e| ClientError {
            code: e.status().map(|s| s.as_u16()).unwrap_or(0),
            message: e.to_string(),
        })?;

        let status = resp.status();
        if status.is_client_error() || status.is_server_error() {
            let message = resp.text().await.unwrap_or_default();
            return Err(ClientError {
                code: status.as_u16(),
                message,
            });
        }

        let bytes = resp.bytes().await.map_err(|e| ClientError {
            code: e.status().map(|s| s.as_u16()).unwrap_or(0),
            message: e.to_string(),
        })?;

        let gz_decoder = flate2::read::GzDecoder::new(std::io::Cursor::new(bytes));
        let mut archive = tar::Archive::new(gz_decoder);

        let entries = archive.entries().map_err(|e| ClientError {
            code: 0,
            message: format!("Failed to read tar archive: {}", e),
        })?;

        let mut results = Vec::new();
        for entry_res in entries {
            let mut entry = entry_res.map_err(|e| ClientError {
                code: 0,
                message: format!("Failed to read tar entry: {}", e),
            })?;

            let is_file = entry.header().entry_type().is_file();
            let path_buf = entry
                .path()
                .map_err(|e| ClientError {
                    code: 0,
                    message: format!("Failed to read tar entry path: {}", e),
                })?
                .into_owned();

            if is_file {
                if let Some(ext) = path_buf.extension() {
                    if ext == "json" {
                        let item: EnrichmentResponse = serde_json::from_reader(&mut entry).map_err(|e| ClientError {
                            code: 0,
                            message: format!("Failed to parse JSON from {}: {}", path_buf.display(), e),
                        })?;
                        results.push(item);
                    }
                }
            }
        }

        Ok(results)
    }
}

// ── Error mapping ─────────────────────────────────────────────────────────────

fn map_error<T: std::fmt::Debug>(err: xyo_openapi_client::apis::Error<T>) -> ClientError {
    match err {
        xyo_openapi_client::apis::Error::ResponseError(rc) => ClientError {
            code: rc.status.as_u16(),
            message: rc.content,
        },
        xyo_openapi_client::apis::Error::Reqwest(e) => ClientError {
            code: e.status().map(|s| s.as_u16()).unwrap_or(0),
            message: e.to_string(),
        },
        xyo_openapi_client::apis::Error::Serde(e) => ClientError {
            code: 0,
            message: e.to_string(),
        },
        xyo_openapi_client::apis::Error::Io(e) => ClientError {
            code: 0,
            message: e.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xyo_openapi_client::apis::ResponseContent;

    #[test]
    fn test_client_new_default_base_url() {
        let client = Client::new("my-token", None);
        assert_eq!(client.configuration.base_path, "https://api.xyo.financial");
        assert_eq!(
            client.configuration.bearer_access_token,
            Some("my-token".to_string())
        );
    }

    #[test]
    fn test_client_new_custom_base_url() {
        let client = Client::new("my-token", Some("https://sandbox.api.xyo.financial".to_string()));
        assert_eq!(
            client.configuration.base_path,
            "https://sandbox.api.xyo.financial"
        );
        assert_eq!(
            client.configuration.bearer_access_token,
            Some("my-token".to_string())
        );
    }

    #[test]
    fn test_client_new_with_string_and_str() {
        let token_str = "token-1";
        let token_string = "token-2".to_string();

        let client1 = Client::new(token_str, None);
        let client2 = Client::new(token_string, None);

        assert_eq!(
            client1.configuration.bearer_access_token,
            Some("token-1".to_string())
        );
        assert_eq!(
            client2.configuration.bearer_access_token,
            Some("token-2".to_string())
        );
    }

    #[test]
    fn test_enrichment_response_serde() {
        let json_str = r#"{
            "merchant": "Uber",
            "description": "Ridesharing service",
            "categories": ["Transportation", "Taxi"],
            "logo": "data:image/png;base64,123",
            "location": "San Francisco, CA",
            "address": "1455 Market St"
        }"#;

        let parsed: EnrichmentResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(parsed.merchant, "Uber");
        assert_eq!(parsed.description, "Ridesharing service");
        assert_eq!(parsed.categories, vec!["Transportation", "Taxi"]);
        assert_eq!(parsed.logo, "data:image/png;base64,123");
        assert_eq!(parsed.location, "San Francisco, CA");
        assert_eq!(parsed.address, "1455 Market St");

        let serialized = serde_json::to_string(&parsed).unwrap();
        assert!(serialized.contains("Uber"));
    }

    #[test]
    fn test_enrich_transaction_collection_response_serde() {
        let json_str = r#"{
            "id": "work-item-12345",
            "link": "https://download.xyo.financial/file.tar.gz"
        }"#;

        let parsed: EnrichTransactionCollectionResponse = serde_json::from_str(json_str).unwrap();
        assert_eq!(parsed.id, "work-item-12345");
        assert_eq!(parsed.link, "https://download.xyo.financial/file.tar.gz");

        let serialized = serde_json::to_string(&parsed).unwrap();
        assert!(serialized.contains("work-item-12345"));
    }

    #[test]
    fn test_enrichment_status_serde_and_variants() {
        let ready = EnrichmentStatus::Ready;
        let pending = EnrichmentStatus::Pending;
        let failed = EnrichmentStatus::Failed;

        let json_ready = serde_json::to_string(&ready).unwrap();
        let json_pending = serde_json::to_string(&pending).unwrap();
        let json_failed = serde_json::to_string(&failed).unwrap();

        assert_eq!(
            serde_json::from_str::<EnrichmentStatus>(&json_ready).unwrap(),
            EnrichmentStatus::Ready
        );
        assert_eq!(
            serde_json::from_str::<EnrichmentStatus>(&json_pending).unwrap(),
            EnrichmentStatus::Pending
        );
        assert_eq!(
            serde_json::from_str::<EnrichmentStatus>(&json_failed).unwrap(),
            EnrichmentStatus::Failed
        );
    }

    #[test]
    fn test_enrichment_request_serde() {
        let req = EnrichmentRequest {
            content: "COSTA COFFEE".to_string(),
            country_code: "GB".to_string(),
        };

        let json_str = serde_json::to_string(&req).unwrap();
        let parsed: EnrichmentRequest = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed.content, "COSTA COFFEE");
        assert_eq!(parsed.country_code, "GB");
    }

    #[test]
    fn test_map_error_response_error() {
        let err: xyo_openapi_client::apis::Error<()> =
            xyo_openapi_client::apis::Error::ResponseError(ResponseContent {
                status: reqwest::StatusCode::FORBIDDEN,
                content: "Forbidden action".to_string(),
                entity: None,
            });

        let client_err = map_error(err);
        assert_eq!(client_err.code, 403);
        assert_eq!(client_err.message, "Forbidden action");
    }

    #[test]
    fn test_map_error_serde() {
        let serde_err: serde_json::Error = serde_json::from_str::<i32>("not an integer").unwrap_err();
        let err: xyo_openapi_client::apis::Error<()> = xyo_openapi_client::apis::Error::Serde(serde_err);

        let client_err = map_error(err);
        assert_eq!(client_err.code, 0);
        assert!(!client_err.message.is_empty());
    }

    #[test]
    fn test_map_error_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "connection reset");
        let err: xyo_openapi_client::apis::Error<()> = xyo_openapi_client::apis::Error::Io(io_err);

        let client_err = map_error(err);
        assert_eq!(client_err.code, 0);
        assert!(client_err.message.contains("connection reset"));
    }
}

