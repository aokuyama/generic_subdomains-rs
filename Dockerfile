FROM rust:1.63.0-slim

RUN apt-get update && \
    apt-get install -y git libssl-dev pkg-config

RUN rustup component add rustfmt
