FROM rust:1.94-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends libsqlite3-dev pkg-config && rm -rf /var/lib/apt/lists/*

# Mirror the repo layout: Cargo.toml and migrations at /app/services/gateway
# so that migrate!("./migrations") resolves identically in Docker and locally.
WORKDIR /app/services/gateway

COPY services/gateway/Cargo.toml services/gateway/Cargo.lock ./
COPY services/gateway/build.rs ./build.rs
COPY services/gateway/src ./src
COPY services/gateway/migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /app/data

COPY --from=builder /app/services/gateway/target/release/gateway-service /usr/local/bin/gateway-service

EXPOSE 8000

CMD ["gateway-service"]
