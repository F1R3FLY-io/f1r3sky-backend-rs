# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.86-slim-bookworm AS builder

RUN apt update && apt install -y protobuf-compiler

WORKDIR /usr/src/rsky
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo build --package rsky-pds

FROM debian:bookworm-slim AS runtime

RUN apt update && apt install -y libpq-dev ca-certificates && update-ca-certificates

WORKDIR /usr/src/rsky
COPY --from=builder /usr/src/rsky/target/debug/rsky-pds rsky-pds

# Run the web service on container startup.
ENTRYPOINT ["/usr/src/rsky/rsky-pds"]
