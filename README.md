<p align="center">
  <a href="https://xyo.financial" target="_blank" rel="noopener noreferrer">
    <img alt="XYO Financial Rust Mascot" width="380" src="https://raw.githubusercontent.com/xyo-financial/sdk-rust/main/docs/mascot.png" />
  </a>
</p>

<h1 align="center">XYO Financial SDK for Rust</h1>

<p align="center">
  <a href="https://crates.io/crates/xyo-sdk"><img src="https://img.shields.io/crates/v/xyo-sdk.svg?color=blue" alt="Crates.io Version" /></a>
  <a href="https://docs.rs/xyo-sdk"><img src="https://docs.rs/xyo-sdk/badge.svg" alt="Documentation" /></a>
  <a href="https://github.com/xyo-financial/sdk-rust/actions/workflows/makefile.yml"><img src="https://github.com/xyo-financial/sdk-rust/actions/workflows/makefile.yml/badge.svg?branch=main" alt="CI Build Pipeline" /></a>
  <img src="https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust&logoColor=white" alt="Rust Edition" />
  <img src="https://img.shields.io/badge/Runtime-Tokio_Async-blueviolet" alt="Tokio Async" />
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg" alt="License" /></a>
</p>

<p align="center">
  <strong>The official Rust SDK for <a href="https://xyo.financial">XYO Financial</a>.</strong><br>
  Seamlessly enrich raw financial transactions into clean merchant profiles, intelligent business categorizations, high-res logos, and geolocated address metadata using AI-powered enrichment pipelines.
</p>

---

## 📖 Summary

The **XYO Financial SDK for Rust** provides an institutional-grade, asynchronous client library for integrating XYO's AI-driven transaction enrichment engine into high-performance financial systems, payment gateways, and banking microservices.

Engineered for Tier-1 banks, payment service providers (PSPs), neo-banks, and quantitative financial institutions, this SDK transforms raw, cryptic merchant statement strings (e.g. `AMZN MKTP UK*1M23456`, `SQ *COSTA GREENWICH`) into structured, verified merchant records complete with official merchant identities, industry categories, logos, geocoded locations, and physical addresses.

Maintained by [Syniol Limited](https://syniol.com) as the official Rust distribution for [XYO.Financial](https://xyo.financial).

---

## 🏗 Architectural Principles

1. **Async & Non-Blocking**: Built natively on [Tokio](https://tokio.rs) and [Reqwest](https://docs.rs/reqwest) for zero-cost async I/O and seamless concurrency under heavy transactional load.
2. **Thread-Safe & Concurrent**: `Client` is `Send + Sync`, enabling safe sharing across worker pools, Tokio tasks, and Actix/Axum web handlers as an application singleton.
3. **Type-Safe Domain Modeling**: Strictly typed request and response structs eliminate serialization ambiguities and runtime protocol mismatches at compile time.
4. **Structured Error Handling**: Returns `ClientError` capturing HTTP status codes and detailed RFC 7807 problem descriptions for robust automated retry and fallback workflows.
5. **Zero `unsafe` Footprint**: 100% safe Rust code with clean dependency boundaries to satisfy rigorous enterprise security and compliance audits.

---

## ⚙️ System Requirements

- **Rust**: Version `1.70.0` or newer (2021 edition).
- **Async Runtime**: [Tokio](https://crates.io/crates/tokio) `1.x` with multi-thread runtime support.
- **Network**: Outbound HTTPS connectivity to `api.xyo.financial` over port `443` (TLS 1.2+ mandatory).
- **Authentication**: A valid API Bearer token obtained from the [XYO Financial Dashboard](https://xyo.financial/dashboard).

---

## 📦 Installation

Add `xyo-sdk` and `tokio` to your `Cargo.toml`:

```toml
[dependencies]
xyo-sdk = "2.0.0"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

Or add via `cargo-cli`:

```bash
cargo add xyo-sdk
cargo add tokio --features rt-multi-thread,macros
```

---

## 🚀 Quickstart Guide

### 1. Client Initialization

Initialize the `Client` with your API Bearer token. Passing `None` as the second argument selects the default production endpoint (`https://api.xyo.financial`):

```rust
use xyo_sdk::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = std::env::var("XYO_API_TOKEN").unwrap_or_else(|_| "your-bearer-token".to_string());
    
    // Default production client
    let client = Client::new(api_token, None);

    let resp = client.enrich_transaction("COSTA PICKUP", "GB").await?;
    println!("Enriched Merchant: {}", resp.merchant);

    Ok(())
}
```

---

### 2. Single Transaction Enrichment (`enrich_transaction`)

Enrich a single financial transaction synchronously in real-time. Ideal for payment authorization hooks, banking mobile apps, and interactive transaction ledgers:

```rust
use xyo_sdk::client::Client;
use xyo_sdk::error::ClientError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = std::env::var("XYO_API_TOKEN").unwrap_or_else(|_| "your-bearer-token".to_string());
    let client = Client::new(api_token, None);

    match client.enrich_transaction("COSTA PICKUP", "GB").await {
        Ok(resp) => {
            println!("--- Transaction Enrichment Result ---");
            println!("Merchant:    {}", resp.merchant);
            println!("Description: {}", resp.description);
            println!("Categories:  {:?}", resp.categories);
            println!("Logo (B64):  {}", if resp.logo.is_empty() { "N/A" } else { "Available" });
            println!("Location:    {}", resp.location);
            println!("Address:     {}", resp.address);
        }
        Err(ClientError { code, message }) => {
            eprintln!("Enrichment failed (HTTP {}): {}", code, message);
        }
    }

    Ok(())
}
```

#### Response Fields:

| Field | Type | Description |
|:---|:---|:---|
| `merchant` | `String` | Official, normalized merchant name (e.g. `"Costa Coffee"`). |
| `description` | `String` | Comprehensive description of the merchant business. |
| `categories` | `Vec<String>` | Standardized merchant classification categories. |
| `logo` | `String` | Base64-encoded merchant brand logo (if available). |
| `location` | `String` | Geocoded city/region (or empty string if not available). |
| `address` | `String` | Verified street address (or empty string if not available). |

---

### 3. Bulk Transaction Enrichment (`enrich_transactions`)

Submit large collections of transactions asynchronously for ETL processing, nightly batch reconciliation, and core banking statement generation:

```rust
use xyo_sdk::client::{Client, EnrichmentRequest};
use xyo_sdk::error::ClientError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = std::env::var("XYO_API_TOKEN").unwrap_or_else(|_| "your-bearer-token".to_string());
    let client = Client::new(api_token, None);

    let batch = vec![
        EnrichmentRequest {
            content: "Syniol AI Payment Enrichment Software".to_string(),
            country_code: "GB".to_string(),
        },
        EnrichmentRequest {
            content: "UBER TRIP HELP.UBER.COM".to_string(),
            country_code: "US".to_string(),
        },
        EnrichmentRequest {
            content: "SPOTIFY PREMIUM".to_string(),
            country_code: "SE".to_string(),
        },
    ];

    // Optional tenant/user identifier (e.g. Some("user-sub-123") or None)
    let api_user = Some("tenant-dept-finops");

    match client.enrich_transactions(batch, api_user).await {
        Ok(job) => {
            println!("Bulk Job ID:     {}", job.id);
            println!("Download Link:   {}", job.link);
        }
        Err(ClientError { code, message }) => {
            eprintln!("Bulk submission failed (HTTP {}): {}", code, message);
        }
    }

    Ok(())
}
```

---

### 4. Bulk Job Status Polling (`get_enrichment_status`)

Check the processing status of an asynchronous bulk enrichment batch until results are ready for ingestion:

```rust
use std::time::Duration;
use tokio::time::sleep;
use xyo_sdk::client::{Client, EnrichmentStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        std::env::var("XYO_API_TOKEN").unwrap_or_else(|_| "your-bearer-token".to_string()),
        None,
    );

    let job_id = "job-bulk-999";
    let api_user = Some("tenant-dept-finops");

    println!("Polling job {}...", job_id);

    loop {
        match client.get_enrichment_status(job_id, api_user).await? {
            EnrichmentStatus::Ready => {
                println!("Job {} is READY! Proceed to download results archive.", job_id);
                break;
            }
            EnrichmentStatus::Pending => {
                println!("Job is still PENDING. Waiting 2 seconds...");
                sleep(Duration::from_secs(2)).await;
            }
            EnrichmentStatus::Failed => {
                eprintln!("Job {} FAILED on the server.", job_id);
                break;
            }
        }
    }

    Ok(())
}
```

---

### 5. Bulk Results Download (`download_enrichment_collection`)

Once a bulk enrichment job transitions to `EnrichmentStatus::Ready`, download and decompress the `.tar.gz` archive containing individual JSON enrichment records directly into a vector of [`EnrichmentResponse`]:

```rust
use xyo_sdk::client::{Client, EnrichmentRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = std::env::var("XYO_API_TOKEN").unwrap_or_else(|_| "your-bearer-token".to_string());
    let client = Client::new(api_token, None);

    let batch = vec![
        EnrichmentRequest {
            content: "COSTA PICKUP".to_string(),
            country_code: "GB".to_string(),
        },
        EnrichmentRequest {
            content: "UBER TRIP".to_string(),
            country_code: "US".to_string(),
        },
    ];

    let collection = client.enrich_transactions(batch, None).await?;
    println!("Bulk Job ID: {}", collection.id);

    // After polling get_enrichment_status until EnrichmentStatus::Ready:
    let results = client.download_enrichment_collection(&collection.link).await?;

    for item in &results {
        println!("--- Enriched Transaction ---");
        println!("Merchant:    {}", item.merchant);
        println!("Description: {}", item.description);
        println!("Categories:  {:?}", item.categories);
        println!("Location:    {}", item.location);
        println!("Address:     {}", item.address);
    }

    Ok(())
}
```

---

## 🛡 Structured Error Handling (`ClientError`)

Every SDK method returns a `Result<T, ClientError>`. The `ClientError` struct provides explicit numeric HTTP status codes along with actionable server diagnostic messages:

```rust
use xyo_sdk::client::Client;
use xyo_sdk::error::ClientError;

#[tokio::main]
async fn main() {
    let client = Client::new("your-api-token", None);

    if let Err(err) = client.enrich_transaction("UNKNOWN MERCHANT", "GB").await {
        match err.code {
            400 => eprintln!("Bad Request: Check content length (<=128 chars) or ISO 3166-1 country code."),
            401 => eprintln!("Unauthorized: Invalid or expired API token. Check dashboard credentials."),
            403 => eprintln!("Forbidden: Account permissions or quota restricted."),
            404 => eprintln!("Not Found: Resource or job ID not located."),
            422 => eprintln!("Unprocessable Entity: Unable to parse transaction description."),
            429 => eprintln!("Rate Limited: Request volume exceeded. Apply backoff and retry."),
            500..=599 => eprintln!("Server Error: Upstream service error. Fall back to secondary processor."),
            0 => eprintln!("Transport Error: Network connectivity, DNS, or TLS handshake failure ({}).", err.message),
            _ => eprintln!("API Error (HTTP {}): {}", err.code, err.message),
        }
    }
}
```

### HTTP Status Code Reference

| HTTP Code | Classification | Cause & Recommended Mitigation |
|:---|:---|:---|
| `400` | Bad Request | Malformed payload (e.g. invalid ISO-3166 alpha-2 country code). Inspect and discard/quarantine. |
| `401` | Unauthorized | Bearer token is missing, expired, or invalid. Verify credentials at [XYO Dashboard](https://xyo.financial/dashboard). |
| `403` | Forbidden | Insufficient plan permissions or suspended billing status. |
| `404` | Not Found | Bulk job work-item ID does not exist or expired. |
| `422` | Unprocessable Entity | Content cannot be parsed into a recognizable merchant format. |
| `429` | Rate Limited | API quota threshold reached. Implement exponential backoff with jitter. |
| `500` / `502` / `503` | Server Error | Temporary backend degradation. Route to dead-letter queue or retry. |
| `0` | Transport Error | TCP connection reset, timeout, DNS resolution, or TLS negotiation error. |

---

## 🚀 Framework & Architecture Integration

The **XYO Financial Rust SDK** is engineered for high-concurrency enterprise microservices, event-driven pipelines, and serverless runtimes. Delivering **compile-time memory safety**, **zero panic vectors**, and **sub-5ms cold starts for AWS Lambda Rust runtime and high-throughput microservices**, it seamlessly integrates into modern Rust asynchronous frameworks.

Because `Client` is `Send + Sync + Clone`, wrapping it in an `Arc<Client>` enables zero-copy shared state across multithreaded web handlers and Tokio worker pools.

---

### 1. Axum Web Framework (`Arc<Client>` State Sharing)

Integrate single transaction enrichment into [Axum](https://github.com/tokio-rs/axum) services using router state extraction:

```rust
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use std::sync::Arc;
use xyo_sdk::client::{Client, EnrichmentRequest, EnrichmentResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize shared client singleton
    let api_token = std::env::var("XYO_API_TOKEN").unwrap_or_else(|_| "xyo_live_key".to_string());
    let client = Arc::new(Client::new(api_token, None)?);

    let app = Router::new()
        .route("/enrich", post(enrich_handler))
        .with_state(client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Axum microservice running on http://0.0.0.0:8080");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn enrich_handler(
    State(client): State<Arc<Client>>,
    Json(payload): Json<EnrichmentRequest>,
) -> Result<Json<EnrichmentResponse>, (StatusCode, String)> {
    let result = client
        .enrich_transaction(&payload.content, &payload.country_code)
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.message))?;

    Ok(Json(result))
}
```

---

### 2. Actix-Web Framework (`web::Data` Application State)

Share the client across worker threads in [Actix-Web](https://actix.rs) using `web::Data`:

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use xyo_sdk::client::{Client, EnrichmentRequest};

async fn enrich_handler(
    client: web::Data<Arc<Client>>,
    payload: web::Json<EnrichmentRequest>,
) -> impl Responder {
    match client.enrich_transaction(&payload.content, &payload.country_code).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": err.message,
            "code": err.code
        })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api_token = std::env::var("XYO_API_TOKEN").unwrap_or_else(|_| "xyo_live_key".to_string());
    let client = Arc::new(Client::new(api_token, None).expect("failed to initialize XYO client"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/enrich", web::post().to(enrich_handler))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
```

---

### 3. AWS Lambda & Serverless Runtimes (Sub-5ms Cold Starts)

Deploy high-velocity transaction enrichment on [AWS Lambda](https://aws.amazon.com/lambda/) with the `provided.al2023` ARM64/x86_64 custom runtime. The zero-overhead compiled binary ensures instant execution with **sub-5ms cold boot latency**:

```rust
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use std::sync::Arc;
use xyo_sdk::client::{Client, EnrichmentRequest};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialized once during Lambda cold start
    let api_token = std::env::var("XYO_API_TOKEN").expect("XYO_API_TOKEN required");
    let client = Arc::new(Client::new(api_token, None)?);

    run(service_fn(move |event: Request| {
        let client = Arc::clone(&client);
        async move {
            let body_bytes = event.body();
            let payload: EnrichmentRequest = serde_json::from_slice(body_bytes)?;
            
            let enriched = client
                .enrich_transaction(&payload.content, &payload.country_code)
                .await?;

            let json_bytes = serde_json::to_vec(&enriched)?;

            Ok::<Response<Body>, Error>(
                Response::builder()
                    .status(200)
                    .header("content-type", "application/json")
                    .body(Body::Binary(json_bytes))?
            )
        }
    }))
    .await
}
```

---

### 4. High-Throughput Stream Processing (Tokio Concurrent Workers)

Process unbounded transaction streams concurrently with bounded parallelism using `tokio::sync::Semaphore` and `futures::stream`:

```rust
use futures::stream::{self, StreamExt};
use std::sync::Arc;
use tokio::sync::Semaphore;
use xyo_sdk::client::{Client, EnrichmentRequest, EnrichmentResponse};
use xyo_sdk::error::ClientError;

pub async fn enrich_stream_concurrent(
    client: Arc<Client>,
    transactions: Vec<EnrichmentRequest>,
    max_concurrency: usize,
) -> Vec<Result<EnrichmentResponse, ClientError>> {
    let semaphore = Arc::new(Semaphore::new(max_concurrency));

    stream::iter(transactions)
        .map(|req| {
            let client = Arc::clone(&client);
            let sem = Arc::clone(&semaphore);
            tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                client.enrich_transaction(&req.content, &req.country_code).await
            })
        })
        .buffer_unordered(max_concurrency)
        .map(|join_res| join_res.unwrap_or_else(|e| Err(ClientError {
            code: 0,
            message: format!("task join error: {}", e),
        })))
        .collect()
        .await
}
```

---

## ⚙️ Advanced Configuration

### Custom Base URL / Sandbox Environments

To route traffic through a mock server (e.g. WireMock in unit tests) or private enterprise gateway:

```rust
use xyo_sdk::client::Client;

let client = Client::new(
    "your-bearer-token",
    Some("https://sandbox.api.xyo.financial".to_string()),
);
```

---

## 📁 Ready-to-Run Examples

The repository includes executable examples demonstrating standard workflows. Run them directly using Cargo:

```bash
# Set your token
export XYO_API_TOKEN="your-bearer-token"

# Run Quickstart (Single Transaction)
cargo run --example quickstart

# Run Bulk Enrichment & Polling
cargo run --example bulk_enrichment

# Run Error Handling demonstration
cargo run --example error_handling
```

---

## 🔒 Security & Compliance

- **Data Minimisation**: Transmits only transaction string descriptions and ISO country codes. Never send PANs, CVVs, account numbers, or Personally Identifiable Information (PII).
- **Transport Encryption**: Enforces TLS 1.2+ on all outbound communication.
- **Supply-Chain Integrity**: Zero `unsafe` blocks in SDK code; dependency tree audited for enterprise distribution.

---

## 📞 Support

- **Developer Dashboard**: [https://xyo.financial/dashboard](https://xyo.financial/dashboard)
- **Technical Support**: [support@syniol.com](mailto:support@syniol.com)
- **Maintainer**: [Syniol Limited](https://syniol.com)

---

## 📄 License

This project is licensed under the **Apache License, Version 2.0** - see the [LICENSE](LICENSE) file for details.

Copyright &copy; 2026 Syniol Limited. All rights reserved.
