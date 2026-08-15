use xyo_sdk::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("YourBearerTokenFromXYODashboard", None)
        .expect("failed to initialize client");

    println!("Initialized XYO SDK client successfully.");
    match client.enrich_transaction("COSTA PICKUP", "GB").await {
        Ok(resp) => {
            println!("Enrichment Success: merchant={}", resp.merchant);
        }
        Err(err) => {
            println!("Encountered expected response (HTTP {}): {}", err.code, err.message);
        }
    }
}


