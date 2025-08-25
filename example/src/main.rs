use xyo_sdk::enrichment::EnrichmentRequest;

fn main() {
    let _: EnrichmentRequest = EnrichmentRequest{
        content: "Syniol AI Payment Enrichment Software".to_string(),
        country_code: "GB".to_string(),
    };

    println!("Successfully imported XYO SDK and created enrichment request");
}
