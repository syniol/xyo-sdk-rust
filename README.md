# XYO Financial SDK for Rust
![workflow](https://github.com/syniol/xyo-sdk-rust/actions/workflows/makefile.yml/badge.svg)

This is an official SDK for XYO Financial in Rust Language.


> ### _NOT READY For Production_


## Quickstart Guide

```rust
use xyo_sdk::client::{new_client, ClientConfig};
use xyo_sdk::enrichment::EnrichmentRequest;

fn main() {
    let client = new_client(ClientConfig{api_key: ""});
    let result = client.enrich_transaction(EnrichmentRequest{
        content: "Syniol Tech".to_string(),
        country_code: "GB".to_string(),
    });

    println!("Merchant Name: {}", result.merchant);
    println!("XYO SDK Initiated successfully");
}
```


#### Credits
Copyright &copy; 2025 Syniol Limited. All rights reserved.
