use crate::enrichment::enrichment::EnrichmentRequest;

mod enrichment;
mod client;

fn main() {
    let sss = EnrichmentRequest{
        content: "Syniol Limited".to_string(),
        country_code: "GB".to_string(),
    };

    println!("Country code: {}", sss.country_code);
    println!("XYO SDK");
}
