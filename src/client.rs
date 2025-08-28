use std::string;
use xyo_http::{request, HttpMethod};

use crate::enrichment::{
    Enrichment,
    EnrichmentCollectionResponse,
    EnrichmentRequest,
    EnrichmentResponse,
    EnrichmentTransactionCollectionStatus,
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
        let resp = (&self.http_client)(HttpMethod::POST, "/api/v1/transaction", "");

        EnrichmentResponse{
            merchant: String::from(""),
            description: String::from(""),
            categories: vec![String::from("")],
            logo: String::from(""),
        }
    }

    fn enrich_transaction_collection(&self, rq: Vec<&EnrichmentRequest>) -> EnrichmentCollectionResponse {
        let resp = (&self.http_client)(HttpMethod::POST, "/api/v1/transactions", "");

        EnrichmentCollectionResponse{
            id: String::from(""),
            link: String::from(""),
        }
    }

    fn enrich_transaction_collection_status(&self, id: &str) -> EnrichmentTransactionCollectionStatus {
        let resp = (&self.http_client)(
            HttpMethod::GET,
            format!("/api/v1/transactions/status/{}", id).as_str(),
            "",
        );

        EnrichmentTransactionCollectionStatus::Failed
    }
}

pub fn new(config: ClientConfig) -> Client {
    Client{
        config,
        http_client: request,
    }
}
