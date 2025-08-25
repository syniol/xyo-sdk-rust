use xyo_sdk::client;
use xyo_sdk::enrichment::{Enrichment, EnrichmentRequest};

fn main() {
    let client = client::new(client::ClientConfig {
        api_key: String::from("hello"),
    });

    // client.enrich_transaction(&EnrichmentRequest {
    //     content: String::from("Syniol AI Payment Enrichment Software"),
    //     country_code: String::from("GB"),
    // });

    println!("Successfully imported XYO SDK and created enrichment request");
}
