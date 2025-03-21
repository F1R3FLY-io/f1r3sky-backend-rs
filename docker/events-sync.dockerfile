FROM rust:1.85-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev protobuf-compiler && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*
COPY firefly-api firefly-api
COPY firefly-events-sync firefly-events-sync
COPY protobuf protobuf
WORKDIR /app/firefly-events-sync
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/firefly-events-sync/target/release/firefly-events-sync ./
STOPSIGNAL SIGINT
ENTRYPOINT ["/app/firefly-events-sync"]
