use xyo_sdk::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Obtain your API Bearer token from the XYO Financial Dashboard:
    // https://xyo.financial/dashboard
    let api_token = std::env::var("XYO_API_TOKEN").unwrap_or_else(|_| "your-bearer-token".to_string());

    // Initialize the XYO Financial client with default production endpoint
    let client = Client::new(api_token, None)?;

    println!("Enriching single financial transaction...");

    match client.enrich_transaction("COSTA PICKUP", "GB").await {
        Ok(enriched) => {
            println!("--- Enrichment Result ---");
            println!("Merchant:    {}", enriched.merchant);
            println!("Description: {}", enriched.description);
            println!("Categories:  {:?}", enriched.categories);
            println!("Location:    {}", enriched.location);
            println!("Address:     {}", enriched.address);
            println!("Logo (B64):  {}", if enriched.logo.is_empty() { "None" } else { "Available" });
        }
        Err(err) => {
            eprintln!("Error enriching transaction (HTTP {}): {}", err.code, err.message);
        }
    }

    Ok(())
}
