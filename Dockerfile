FROM rust:1.89-alpine

RUN mkdir -p /var/local/xyo-sdk

COPY . /var/local/xyo-sdk/

# Running Tests and Build for SDK
WORKDIR /var/local/xyo-sdk
RUN cargo test

# Testing imported SDK and instantion
WORKDIR /var/local/xyo-sdk/example
RUN cargo run || exit 1
