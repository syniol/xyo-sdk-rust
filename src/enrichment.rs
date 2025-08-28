use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct EnrichmentRequest {
    pub content: String,
    pub country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnrichmentResponse {
    pub merchant: String,
    pub description: String,
    pub categories: Vec<String>,
    pub logo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnrichmentCollectionResponse {
    pub id: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EnrichmentTransactionCollectionStatus {
    Failed,
    Pending,
    Success,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnrichmentTransactionCollectionStatusResponse {
    pub status: EnrichmentTransactionCollectionStatus,
}

pub trait Enrichment {
    fn enrich_transaction(&self, rq: &EnrichmentRequest) -> EnrichmentResponse;
    fn enrich_transaction_collection(
        &self,
        rq: Vec<&EnrichmentRequest>,
    ) -> EnrichmentCollectionResponse;
    fn enrich_transaction_collection_status(
        &self,
        id: &str,
    ) -> EnrichmentTransactionCollectionStatus;
}
