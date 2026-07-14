use xyo_http::{request, HttpClientError, HttpMethod, HttpResponse};

use crate::enrichment::{
    Enrichment, EnrichmentCollectionResponse, EnrichmentRequest, EnrichmentResponse,
    EnrichmentTransactionCollectionStatus, EnrichmentTransactionCollectionStatusResponse,
};
use crate::error::ClientError;

pub struct ClientConfig {
    pub api_key: String,
}

pub struct Client {
    pub config: ClientConfig,
    http_client: fn(
        method: HttpMethod,
        path: &str,
        headers: &[(&str, &str)],
        data: &str,
    ) -> Result<HttpResponse, HttpClientError>,
}

impl Enrichment for Client {
    fn enrich_transaction(
        &self,
        rq: &EnrichmentRequest,
    ) -> Result<EnrichmentResponse, ClientError> {
        let payload = serde_json::to_string(rq).map_err(|e| ClientError {
            code: 400,
            message: format!("Failed to serialize request: {}", e),
        })?;

        let auth_header = format!("Bearer {}", self.config.api_key);
        let headers = [("Authorization", auth_header.as_str())];

        let result = (&self.http_client)(
            HttpMethod::POST,
            "/api/v1/transaction",
            &headers,
            &payload,
        )
        .map_err(|e| ClientError {
            code: e.code,
            message: e.message,
        })?;

        if result.status_code != 200 {
            return Err(ClientError {
                message: result.body,
                code: result.status_code,
            });
        }

        let response: EnrichmentResponse =
            serde_json::from_str(&result.body).map_err(|e| ClientError {
                code: 500,
                message: format!("Failed to deserialize response: {}", e),
            })?;

        Ok(response)
    }

    fn enrich_transaction_collection(
        &self,
        rq: &[EnrichmentRequest],
    ) -> Result<EnrichmentCollectionResponse, ClientError> {
        let payload = serde_json::to_string(rq).map_err(|e| ClientError {
            code: 400,
            message: format!("Failed to serialize request: {}", e),
        })?;

        let auth_header = format!("Bearer {}", self.config.api_key);
        let headers = [("Authorization", auth_header.as_str())];

        let result = (&self.http_client)(
            HttpMethod::POST,
            "/api/v1/transactions",
            &headers,
            &payload,
        )
        .map_err(|e| ClientError {
            code: e.code,
            message: e.message,
        })?;

        if result.status_code != 200 {
            return Err(ClientError {
                message: result.body,
                code: result.status_code,
            });
        }

        let response: EnrichmentCollectionResponse =
            serde_json::from_str(&result.body).map_err(|e| ClientError {
                code: 500,
                message: format!("Failed to deserialize response: {}", e),
            })?;

        Ok(response)
    }

    fn enrich_transaction_collection_status(
        &self,
        id: &str,
    ) -> Result<EnrichmentTransactionCollectionStatus, ClientError> {
        let auth_header = format!("Bearer {}", self.config.api_key);
        let headers = [("Authorization", auth_header.as_str())];
        let path = format!("/api/v1/transactions/status/{}", id);

        let result = (&self.http_client)(HttpMethod::GET, &path, &headers, "")
            .map_err(|e| ClientError {
                code: e.code,
                message: e.message,
            })?;

        if result.status_code != 200 {
            return Err(ClientError {
                message: result.body,
                code: result.status_code,
            });
        }

        let response: EnrichmentTransactionCollectionStatusResponse =
            serde_json::from_str(&result.body).map_err(|e| ClientError {
                code: 500,
                message: format!("Failed to deserialize response: {}", e),
            })?;

        Ok(response.status)
    }
}

pub fn new(config: ClientConfig) -> Client {
    Client {
        config,
        http_client: request,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_when_enrich_transaction_has_ok_status_code() {
        use xyo_http::{HttpMethod, HttpResponse};

        fn mocked_request_call(
            _method: HttpMethod,
            _path: &str,
            _headers: &[(&str, &str)],
            _request_data: &str,
        ) -> Result<HttpResponse, HttpClientError> {
            let mocked_enrichment_response: EnrichmentResponse = EnrichmentResponse {
                merchant: String::from("Syniol Limited"),
                description: String::from("Software and Platform Consultancy"),
                categories: vec![String::from("Software")],
                logo: String::from("base64/png-dsadsadasdasdasdasdsa"),
                location: Some(String::from("London, United Kingdom")),
                address: Some(String::from("")),
            };

            Ok(HttpResponse {
                status_code: 200,
                body: serde_json::to_string(&mocked_enrichment_response).unwrap(),
            })
        }

        let client = Client {
            http_client: mocked_request_call,
            config: ClientConfig {
                api_key: "MyAPIKeyFromDashboardXYO.Financial".to_string(),
            },
        };

        let resp = client.enrich_transaction(&EnrichmentRequest {
            content: String::from("Syniol Tech"),
            country_code: String::from("GB"),
        });

        assert!(resp.is_ok());
        let actual: EnrichmentResponse = resp.unwrap();

        assert_eq!("Syniol Limited", actual.merchant);
        assert_eq!("Software and Platform Consultancy", actual.description);
        assert_eq!(vec![String::from("Software")], actual.categories);
        assert_eq!("base64/png-dsadsadasdasdasdasdsa", actual.logo);
        assert_eq!("London, United Kingdom", actual.location.unwrap());
        assert_eq!("", actual.address.unwrap());
    }

    #[test]
    fn it_errors_when_enrich_transaction_has_not_ok_status_code() {
        use xyo_http::{HttpMethod, HttpResponse};

        fn mocked_request_call(
            _method: HttpMethod,
            _path: &str,
            _headers: &[(&str, &str)],
            _request_data: &str,
        ) -> Result<HttpResponse, HttpClientError> {
            Ok(HttpResponse {
                status_code: 400,
                body: "mocked error response".to_string(),
            })
        }

        let client = Client {
            http_client: mocked_request_call,
            config: ClientConfig {
                api_key: "MyAPIKeyFromDashboardXYO.Financial".to_string(),
            },
        };

        let resp = client.enrich_transaction(&EnrichmentRequest {
            content: String::from("Syniol Tech"),
            country_code: String::from("GB"),
        });

        assert!(resp.is_err());
        let actual = resp.unwrap_err();

        assert_eq!(actual.message, "mocked error response");
        assert_eq!(actual.code, 400);
    }
}
