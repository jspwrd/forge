FROM rust:1.85-bookworm AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc \
    libc6-dev \
    make \
    file \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/forge /usr/local/bin/forge

WORKDIR /project
ENTRYPOINT ["forge"]
