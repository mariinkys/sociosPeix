FROM rust:1.95-slim AS builder

WORKDIR /app

COPY . .

RUN cargo build --release -p pdf-analyzer-service

# Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/pdf-analyzer-service .
COPY --from=builder /app/pdf-analyzer/template.toml ./pdf-analyzer/template.toml

EXPOSE 8080

ENV RUST_LOG=warn,pdf_analyzer_service=info

CMD ["./pdf-analyzer-service"]
