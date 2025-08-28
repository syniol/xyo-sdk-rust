use xyo_http::{get_body_from_request_response, get_status_code, request, HttpMethod};

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
    http_client: fn(method: HttpMethod, path: &str, data: &str) -> String,
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
        let status_code = get_status_code(resp.clone());
        if status_code != 200 {
            return Err(ClientError {
                message: String::from("response is not 200"),
                code: status_code,
            });
        }

        let response_body = get_body_from_request_response(resp);
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
        let status_code = get_status_code(resp.clone());
        if status_code != 200 {
            return Err(ClientError {
                message: String::from("response is not 200"),
                code: status_code,
            });
        }

        let response_body = get_body_from_request_response(resp);
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
        let status_code: i16 = get_status_code(resp.clone());
        if status_code != 200 {
            return Err(ClientError {
                message: String::from("response is not 200"),
                code: status_code,
            });
        }

        let response_body = get_body_from_request_response(resp);
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

        fn mocked_request_call(_: HttpMethod, _: &str, _: &str) -> String {
            let mocked_enrichment_response: EnrichmentResponse = EnrichmentResponse {
                merchant: String::from("Syniol Limited"),
                description: String::from("Software and Platform Consultancy"),
                categories: vec![String::from("Software")],
                logo: String::from("base64/png-dsadsadasdasdasdasdsa"),
            };

            String::from(format!(
                "HTTP/1.1 200 OK\r\nServer: nginx/1.22.1\r\nContent-Type: application/json\r\n\n{}",
                serde_json::to_string(&mocked_enrichment_response).unwrap(),
            ))
        }

        let client = Client {
            http_client: mocked_request_call,
            config: ClientConfig {
                api_key: "".to_string(),
            },
        };

        let resp = client.enrich_transaction(&EnrichmentRequest {
            content: String::from("Syniol Tech"),
            country_code: String::from("GB"),
        });

        println!("{}", resp.unwrap().logo);
    }
}
