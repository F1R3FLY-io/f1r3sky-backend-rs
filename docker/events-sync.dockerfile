FROM rust:1.86-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml cargo.main
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev protobuf-compiler && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*; \
    sed "s|^members\ *=\ *\[.*\]|members = \[\"firefly-api\", \"firefly-events-sync\"]|" < cargo.main > Cargo.toml && \
    rm cargo.main && \
    cat Cargo.toml
COPY firefly-api firefly-api
COPY firefly-events-sync firefly-events-sync
COPY protobuf protobuf
WORKDIR /app/firefly-events-sync
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && \
    apt-get install -y libssl3 && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*;
COPY --from=builder /app/target/release/firefly-events-sync ./
STOPSIGNAL SIGINT
ENTRYPOINT ["/app/firefly-events-sync"]
