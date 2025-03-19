FROM rust:1.85-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev protobuf-compiler && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*
COPY firefly-api firefly-api
COPY firefly-state-sync firefly-state-sync
COPY protobuf protobuf
WORKDIR /app/firefly-state-sync
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
ARG POSTGRESQL_VERSION
WORKDIR /app
RUN apt-get update && \
    apt-get install -y gnupg wget && \
    echo "deb http://apt.postgresql.org/pub/repos/apt bookworm-pgdg main" > /etc/apt/sources.list.d/pgdg.list && \
    wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | gpg --dearmor -o /etc/apt/trusted.gpg.d/postgresql.gpg && \
    apt-get update && \
    apt-get install -y postgresql-client-${POSTGRESQL_VERSION} && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/firefly-state-sync/target/release/firefly-state-sync ./
STOPSIGNAL SIGINT
ENTRYPOINT ["/app/firefly-state-sync"]
