use xyo_http::{get_body_from_request_response, get_status_code, request, HttpMethod, HttpClientError};

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
    http_client: fn(method: HttpMethod, path: &str, data: &str) -> Result<String, HttpClientError>,
}

impl Enrichment for Client {
    fn enrich_transaction(
        &self,
        rq: &EnrichmentRequest,
    ) -> Result<EnrichmentResponse, ClientError> {
        let resp = (&self.http_client)(
            HttpMethod::POST,
            "/api/v1/transaction",
            serde_json::to_string(rq).unwrap().as_str(),
        );
        if resp.is_err() {
            let err = resp.err().unwrap();
            return Err(ClientError{
                code: err.code,
                message: err.message,
            })
        }

        let result = resp.ok().unwrap();
        let status_code = get_status_code(result.clone());
        if status_code != 200 {
            return Err(ClientError {
                message: get_body_from_request_response(result),
                code: status_code,
            });
        }

        let response_body = get_body_from_request_response(result);
        let result: EnrichmentResponse = serde_json::from_str(response_body.as_str()).unwrap();

        Ok(result)
    }

    fn enrich_transaction_collection(
        &self,
        rq: Vec<&EnrichmentRequest>,
    ) -> Result<EnrichmentCollectionResponse, ClientError> {
        let resp = (&self.http_client)(
            HttpMethod::POST,
            "/api/v1/transactions",
            serde_json::to_string(&rq).unwrap().as_str(),
        );
        if resp.is_err() {
            let err = resp.err().unwrap();
            return Err(ClientError{
                code: err.code,
                message: err.message,
            })
        }

        let result = resp.ok().unwrap();
        let status_code = get_status_code(result.clone());
        if status_code != 200 {
            return Err(ClientError {
                message: get_body_from_request_response(result),
                code: status_code,
            });
        }

        let response_body = get_body_from_request_response(result);
        let result: EnrichmentCollectionResponse =
            serde_json::from_str(response_body.as_str()).unwrap();

        Ok(result)
    }

    fn enrich_transaction_collection_status(
        &self,
        id: &str,
    ) -> Result<EnrichmentTransactionCollectionStatus, ClientError> {
        let resp = (&self.http_client)(
            HttpMethod::GET,
            format!("/api/v1/transactions/status/{}", id).as_str(),
            "",
        );
        if resp.is_err() {
            let err = resp.err().unwrap();
            return Err(ClientError{
                code: err.code,
                message: err.message,
            })
        }

        let result = resp.ok().unwrap();
        let status_code: i16 = get_status_code(result.clone());
        if status_code != 200 {
            return Err(ClientError {
                message: get_body_from_request_response(result),
                code: status_code,
            });
        }

        let response_body = get_body_from_request_response(result);
        let result: EnrichmentTransactionCollectionStatusResponse =
            serde_json::from_str(response_body.as_str()).unwrap();

        Ok(result.status)
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
        use xyo_http::HttpMethod;

        fn mocked_request_call(_method: HttpMethod, _path: &str, _request_data: &str) -> Result<String, HttpClientError> {
            let mocked_enrichment_response: EnrichmentResponse = EnrichmentResponse {
                merchant: String::from("Syniol Limited"),
                description: String::from("Software and Platform Consultancy"),
                categories: vec![String::from("Software")],
                logo: String::from("base64/png-dsadsadasdasdasdasdsa"),
                location: Some(String::from("London, United Kingdom")),
                address: Some(String::from("")),
            };

            Ok(String::from(format!(
                "HTTP/1.1 200 OK\r\nServer: nginx/1.22.1\r\nContent-Type: application/json\r\n\n{}",
                serde_json::to_string(&mocked_enrichment_response).unwrap(),
            )))
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

        let actual:EnrichmentResponse = resp.unwrap();

        assert_eq!("Syniol Limited", actual.merchant);
        assert_eq!("Software and Platform Consultancy", actual.description);
        assert_eq!(vec![String::from("Software")], actual.categories);
        assert_eq!("base64/png-dsadsadasdasdasdasdsa", actual.logo);
        assert_eq!("London, United Kingdom", actual.location.unwrap());
        assert_eq!("", actual.address.unwrap());
    }

    #[test]
    fn it_errors_when_enrich_transaction_has_not_ok_status_code() {
        use xyo_http::HttpMethod;

        fn mocked_request_call(_method: HttpMethod, _path: &str, _request_data: &str) -> Result<String, HttpClientError> {
            let mocked_enrichment_response_err = "mocked error response";

            Ok(String::from(format!(
                "HTTP/1.1 400 OK\r\nServer: nginx/1.22.1\r\nContent-Type: application/json\r\n{}",
                mocked_enrichment_response_err,
            )))
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

        let actual = resp.unwrap_err();

        assert_eq!(actual.message, "mocked error response");
        assert_eq!(actual.code, 400);
    }
}
