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
