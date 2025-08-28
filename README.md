# XYO Financial SDK for Rust
![workflow](https://github.com/syniol/xyo-sdk-rust/actions/workflows/makefile.yml/badge.svg)

This is an official SDK for XYO Financial in Rust Language.


> ### _NOT READY For Production_


## Quickstart Guide
Client is an entry point to consume the enrichment services.

```rust
use xyo_sdk::client::{new, ClientConfig};
use xyo_sdk::enrichment::EnrichmentRequest;

fn main() {
    let client = new(ClientConfig {
        api_key: "YourAPIKeyFromXYO.FinancialDashboard"
    });

    let result = client.enrich_transaction(&EnrichmentRequest{
        content: String::from("Syniol Tech"),
        country_code: String::from("GB"),
    });

    println!("Merchant Name: {}", result.merchant);
    println!("XYO SDK Initiated successfully");
}
```


#### Credits
Copyright &copy; 2025 Syniol Limited. All rights reserved.
