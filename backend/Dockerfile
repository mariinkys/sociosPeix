# syntax=docker/dockerfile:1.7
#
# Socios Peix backend (Rust/axum) with the registration-form OCR built in.
#
#   docker build -t sociospeix-backend backend/
#   docker build --build-arg CARGO_FEATURES= -t sociospeix-backend:no-ocr backend/   # without OCR

ARG RUST_VERSION=1.97

# ---------- build ----------
FROM docker.io/library/rust:${RUST_VERSION}-slim-trixie AS build
ARG CARGO_FEATURES=ocr
WORKDIR /src

RUN apt-get update \
 && apt-get install -y --no-install-recommends pkg-config libssl-dev build-essential ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY . .

# Cache mounts keep the crates registry and build artefacts between builds,
# so only changed code is recompiled.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/src/target \
    cargo build --release --locked --no-default-features --features "${CARGO_FEATURES}" \
 && cp target/release/sociospeix /usr/local/bin/sociospeix

# Bake the OCR models into the image: no download at runtime, no volume.
ENV OAR_HOME=/models
RUN mkdir -p /models \
 && if echo "${CARGO_FEATURES}" | grep -q ocr; then sociospeix fetch-ocr-models; fi

# ---------- runtime ----------
# Distroless: glibc + libstdc++ (needed by ONNX Runtime) + CA certificates,
# no shell or package manager. Runs as an unprivileged user.
FROM gcr.io/distroless/cc-debian13:nonroot
COPY --from=build /usr/local/bin/sociospeix /usr/local/bin/sociospeix
COPY --from=build --chown=nonroot:nonroot /models /models

ENV OAR_HOME=/models \
    BIND_ADDR=0.0.0.0:8080 \
    RUST_LOG=info,sqlx=warn,tower_http=info \
    LOG_FORMAT=json

EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
    CMD ["/usr/local/bin/sociospeix", "healthcheck"]
ENTRYPOINT ["/usr/local/bin/sociospeix"]
CMD ["serve"]
