pub struct EnrichmentRequest {
    pub content: String,
    pub country_code: String,
}

pub struct EnrichmentResponse {
    merchant: String,
    description: String,
    categories: Vec<String>,
    logo: String,
}

pub struct EnrichmentCollectionResponse {
    id: String,
    link: String,
}

pub enum EnrichmentTransactionCollectionStatus {
    Failed,
    Pending,
    Success,
}

pub trait Enrichment {
    fn enrich_transaction(rq: EnrichmentRequest) -> EnrichmentResponse;
    fn enrich_transaction_collection(rq: &Vec<EnrichmentRequest>) -> EnrichmentCollectionResponse;
    fn enrich_transaction_collection_status(id: String) -> EnrichmentTransactionCollectionStatus;
}

pub fn health() -> bool {
    return true
}
