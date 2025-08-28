use xyo_sdk::client;
// use xyo_sdk::enrichment::Enrichment;
// use xyo_sdk::enrichment::{Enrichment, EnrichmentRequest};

fn main() {
    let _ = client::new(client::ClientConfig {
        api_key: String::from("YourAPIKeyFromXYO.FinancialDashboard"),
    });

    // client.enrich_transaction(&EnrichmentRequest {
    //     content: String::from("Syniol AI Payment Enrichment Software"),
    //     country_code: String::from("GB"),
    // });
    //
    println!("Successfully imported XYO SDK");
}
