FROM rust:1.89-alpine

RUN apk add --update-cache \
    build-base \
    ca-certificates \
    openssl \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

RUN rustup component add rustfmt

RUN mkdir -p /var/local/xyo-sdk

COPY . /var/local/xyo-sdk/

# Running Tests and Build for SDK
WORKDIR /var/local/xyo-sdk
RUN cargo test --verbose

# Verify documentation coverage (fail build on any missing rustdoc)
RUN RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps --verbose

# Running Tests for OpenAPI Workspace dependency
WORKDIR /var/local/xyo-sdk/openapi
RUN cargo test --verbose

# Testing imported SDK and instantion
WORKDIR /var/local/xyo-sdk/example
RUN cargo run || exit 1

WORKDIR /var/local/xyo-sdk
