FROM rust:1.94-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd -r -s /bin/false runner
RUN echo "EXPBOX{rce_first_blood_a1b2c3}" > /flag.txt && chmod 644 /flag.txt
COPY --from=builder /app/target/release/rce /usr/local/bin/rce
USER runner
EXPOSE 9000
CMD ["rce"]
