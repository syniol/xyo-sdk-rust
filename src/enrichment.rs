struct EnrichmentRequest {
    content: String,
    country_code: String,
}

struct EnrichmentResponse {
    merchant: String,
    description: String,
    categories: Vec<Categories>,
    logo: String,
}

struct EnrichmentCollectionResponse {
    id: String,
    link: String,
}

enum EnrichmentTransactionCollectionStatus {
    Failed,
    Pending,
    Success,
}

trait Enrichment {
    fn enrich_transaction(rq: EnrichmentRequest) -> EnrichmentResponse;
    fn enrich_transaction_collection(rq: EnrichmentRequest) -> EnrichmentCollectionResponse;
    fn enrich_transaction_collection_status(id: String) -> EnrichmentTransactionCollectionStatus;
}
