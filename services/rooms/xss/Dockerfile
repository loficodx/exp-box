FROM rust:1.94-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*
RUN useradd -r -s /bin/false runner \
    && mkdir -p /app/data \
    && chown -R runner:runner /app/data
COPY --from=builder /app/target/release/xss /usr/local/bin/xss
USER runner
EXPOSE 9000
CMD ["xss"]
