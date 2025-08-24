FROM rust:1.89-alpine

RUN mkdir -p /var/local/xyo-sdk

COPY . /var/local/xyo-sdk/

WORKDIR /var/local/xyo-sdk

RUN cargo test
