FROM rust:1.88-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd -r -s /bin/false runner
COPY --from=builder /app/target/release/xss /usr/local/bin/xss
USER runner
EXPOSE 9000
CMD ["xss"]
