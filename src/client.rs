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
    http_client: i32,
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
    Client{
        config,
        http_client: 0,
    }
}
