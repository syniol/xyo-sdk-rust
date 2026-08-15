use xyo_sdk::client::Client;
use xyo_sdk::error::ClientError;

#[tokio::main]
async fn main() {
    // Example using an invalid token to demonstrate structured error handling
    let client = Client::new("invalid-token", None).expect("failed to construct client");

    println!("Attempting API call with invalid authentication...");

    match client.enrich_transaction("SPOTIFY PREMIUM", "SE").await {
        Ok(resp) => {
            println!("Enrichment succeeded: {}", resp.merchant);
        }
        Err(ClientError { code, message }) => {
            println!("Encountered ClientError:");
            println!("  HTTP Status Code: {}", code);
            println!("  Error Message:    {}", message);

            match code {
                401 => eprintln!("  Resolution: Verify your API key at https://xyo.financial/dashboard"),
                400 | 422 => eprintln!("  Resolution: Check transaction content and ISO country code format"),
                404 => eprintln!("  Resolution: Merchant or resource not found"),
                500..=599 => eprintln!("  Resolution: XYO API server error - retry with exponential backoff"),
                0 => eprintln!("  Resolution: Network/transport error - verify internet connection and DNS"),
                _ => eprintln!("  Resolution: Unexpected error code"),
            }
        }
    }
}
