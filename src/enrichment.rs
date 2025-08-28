pub struct EnrichmentRequest {
    pub content: String,
    pub country_code: String,
}

pub struct EnrichmentResponse {
    pub merchant: String,
    pub description: String,
    pub categories: Vec<String>,
    pub logo: String,
}

pub struct EnrichmentCollectionResponse {
    pub id: String,
    pub link: String,
}

pub enum EnrichmentTransactionCollectionStatus {
    Failed,
    Pending,
    Success,
}

pub trait Enrichment {
    fn enrich_transaction(&self, rq: &EnrichmentRequest) -> EnrichmentResponse;
    fn enrich_transaction_collection(&self, rq: Vec<&EnrichmentRequest>) -> EnrichmentCollectionResponse;
    fn enrich_transaction_collection_status(&self, id: &str) -> EnrichmentTransactionCollectionStatus;
}
