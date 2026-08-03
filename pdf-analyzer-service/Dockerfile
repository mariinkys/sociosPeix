FROM rust:1.95-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release -p pdf-analyzer-service

# Runtime
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/pdf-analyzer-service .
COPY --from=builder /app/pdf-analyzer/template.toml ./pdf-analyzer/template.toml

EXPOSE 8081

ENV RUST_LOG=warn,pdf_analyzer_service=info

CMD ["./pdf-analyzer-service"]
