use xyo_sdk::enrichment::enrichment::EnrichmentRequest;

fn main() {
    let zzz: EnrichmentRequest = EnrichmentRequest{
        content: "".to_string(),
        country_code: "".to_string(),
    };

    println!("Successfully imported XYO SDK and created enrichment request");
}
