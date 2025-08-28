FROM rust:1.89-alpine

RUN apk add --update-cache \
    build-base \
    ca-certificates \
    openssl

RUN mkdir -p /var/local/xyo-sdk

COPY . /var/local/xyo-sdk/

# Running Tests and Build for SDK
WORKDIR /var/local/xyo-sdk
RUN cargo test --verbose

# Running Tests for XYO-HTTP Workspace dependency
WORKDIR /var/local/xyo-sdk/xyo-http
RUN cargo test --verbose

# Testing imported SDK and instantion
WORKDIR /var/local/xyo-sdk/example
RUN cargo run || exit 1

WORKDIR /var/local/xyo-sdk
