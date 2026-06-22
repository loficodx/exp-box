FROM rust:1.94-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends libsqlite3-dev pkg-config && rm -rf /var/lib/apt/lists/*

# Mirror the repo layout: Cargo.toml at /app/backend, migrations at /app/migrations
# so that migrate!("../migrations") resolves identically in Docker and locally.
WORKDIR /app/backend

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src
COPY migrations ../migrations

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /app/data

COPY --from=builder /app/backend/target/release/backend /usr/local/bin/backend

EXPOSE 8000

CMD ["backend"]
