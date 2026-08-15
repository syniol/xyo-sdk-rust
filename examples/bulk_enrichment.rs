use std::time::Duration;
use tokio::time::sleep;
use xyo_sdk::client::{Client, EnrichmentRequest, EnrichmentStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = std::env::var("XYO_API_TOKEN").unwrap_or_else(|_| "your-bearer-token".to_string());
    let client = Client::new(api_token, None)?;

    // Prepare a batch of transactions to enrich
    let batch = vec![
        EnrichmentRequest {
            content: "UBER TRIP".to_string(),
            country_code: "GB".to_string(),
        },
        EnrichmentRequest {
            content: "NETFLIX.COM".to_string(),
            country_code: "US".to_string(),
        },
        EnrichmentRequest {
            content: "AMAZON RETAIL".to_string(),
            country_code: "GB".to_string(),
        },
    ];

    println!("Submitting bulk enrichment batch of {} items...", batch.len());

    // Submit batch (optional tenant user tracking: Some("tenant-user-123") or None)
    let job = client.enrich_transactions(batch, Some("tenant-user-123")).await?;
    println!("Batch accepted! Job ID: {}", job.id);
    println!("Download Link: {}", job.link);

    // Poll status until ready or failed
    println!("Polling job status...");
    for attempt in 1..=5 {
        sleep(Duration::from_millis(500)).await;
        let status = client.get_enrichment_status(&job.id, Some("tenant-user-123")).await?;

        match status {
            EnrichmentStatus::Ready => {
                println!("Job is READY! Results archive available at: {}", job.link);
                println!("Downloading and unpacking enrichment collection results...");
                let results = client.download_enrichment_collection(&job.link).await?;
                println!("Downloaded {} enriched records:", results.len());
                for (i, res) in results.iter().enumerate() {
                    println!("  [{i}] Merchant: {} ({})", res.merchant, res.description);
                }
                break;
            }
            EnrichmentStatus::Pending => {
                println!("Attempt {attempt}: Job is still PENDING...");
            }
            EnrichmentStatus::Failed => {
                eprintln!("Job FAILED during processing.");
                break;
            }
        }
    }

    Ok(())
}

