FROM rust:1.77-buster as builder
WORKDIR /usr/src/myapp

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

COPY ./src ./src
RUN  cargo build --release

FROM debian:bookworm
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/myapp/target/release/rust-api /usr/local/bin/rust-api

WORKDIR /usr/local/bin
CMD ["rust-api"]
