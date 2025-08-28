use xyo_http::{get_body_from_request_response, get_status_code, request, HttpMethod};

use crate::enrichment::{
    Enrichment, EnrichmentCollectionResponse, EnrichmentRequest, EnrichmentResponse,
    EnrichmentTransactionCollectionStatus, EnrichmentTransactionCollectionStatusResponse,
};

pub struct ClientConfig {
    pub api_key: String,
}

pub struct Client {
    pub config: ClientConfig,
    pub http_client: fn(method: HttpMethod, path: &str, data: &str) -> String,
}

impl Enrichment for Client {
    fn enrich_transaction(&self, rq: &EnrichmentRequest) -> EnrichmentResponse {
        let resp = (&self.http_client)(
            HttpMethod::POST,
            "/api/v1/transaction",
            serde_json::to_string(rq).unwrap().as_str(),
        );
        if get_status_code(resp.clone()) != 200 {
            todo!("throw an error")
        }

        let response_body = get_body_from_request_response(resp);
        let result: EnrichmentResponse = serde_json::from_str(response_body.as_str()).unwrap();

        result
    }

    fn enrich_transaction_collection(
        &self,
        rq: Vec<&EnrichmentRequest>,
    ) -> EnrichmentCollectionResponse {
        let resp = (&self.http_client)(
            HttpMethod::POST,
            "/api/v1/transactions",
            serde_json::to_string(&rq).unwrap().as_str(),
        );
        if get_status_code(resp.clone()) != 200 {
            todo!("throw an error")
        }

        let response_body = get_body_from_request_response(resp);
        let result: EnrichmentCollectionResponse =
            serde_json::from_str(response_body.as_str()).unwrap();

        result
    }

    fn enrich_transaction_collection_status(
        &self,
        id: &str,
    ) -> EnrichmentTransactionCollectionStatus {
        let resp = (&self.http_client)(
            HttpMethod::GET,
            format!("/api/v1/transactions/status/{}", id).as_str(),
            "",
        );
        if get_status_code(resp.clone()) != 200 {
            todo!("throw an error")
        }

        let response_body = get_body_from_request_response(resp);
        let result: EnrichmentTransactionCollectionStatusResponse =
            serde_json::from_str(response_body.as_str()).unwrap();

        result.status
    }
}

pub fn new(config: ClientConfig) -> Client {
    Client {
        config,
        http_client: request,
    }
}
