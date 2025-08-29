# XYO Financial SDK for Rust
![workflow](https://github.com/syniol/xyo-sdk-rust/actions/workflows/makefile.yml/badge.svg)    ![workflow](https://github.com/syniol/xyo-sdk-rust/actions/workflows/crates_xyo_http_publish.yml/badge.svg)    ![workflow](https://github.com/syniol/xyo-sdk-rust/actions/workflows/crates_publish.yml/badge.svg)

<p align="center">
    <a href="https://xyo.financial" target="blank"><img alt="crab mascot" width="50%" src="https://github.com/syniol/xyo-sdk-rust/blob/main/docs/mascot.png?raw=true" /></a>
</p>

This is an official SDK for XYO Financial in Rust Language.


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
