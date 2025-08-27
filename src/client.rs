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
    // todo: find an appropriate package for http request since rust std library has none!
    http_client: fn(method: &str, path: &str, data: usize) -> String,
}

impl Enrichment for Client {
    fn enrich_transaction(&self, rq: &EnrichmentRequest) -> EnrichmentResponse {
        todo!()
    }

    fn enrich_transaction_collection(&self, rq: Vec<&EnrichmentRequest>) -> EnrichmentCollectionResponse {
        todo!()
    }

    fn enrich_transaction_collection_status(&self, id: String) -> EnrichmentTransactionCollectionStatus {
        todo!()
    }
}

pub fn new(config: ClientConfig) -> Client {
    fn sss(method: &str, path: &str, data: usize) -> String {
        return format!("https://{}{}", method, path);
    }

    Client{
        config,
        http_client: sss,
    }
}
