# ─── Multi-stage Rust build ───────────────────────────────────────────────────

# Stage 1: Build
FROM rust:1.78-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies layer
COPY Cargo.toml Cargo.lock ./
COPY crates/domain/Cargo.toml   crates/domain/
COPY crates/application/Cargo.toml crates/application/
COPY crates/infrastructure/Cargo.toml crates/infrastructure/
COPY crates/api/Cargo.toml      crates/api/
COPY crates/worker/Cargo.toml   crates/worker/

# Create dummy source files to cache deps
RUN mkdir -p crates/domain/src crates/application/src crates/infrastructure/src crates/api/src crates/worker/src && \
    echo "fn main() {}" > crates/api/src/main.rs && \
    echo "fn main() {}" > crates/worker/src/main.rs && \
    touch crates/domain/src/lib.rs crates/application/src/lib.rs crates/infrastructure/src/lib.rs

RUN cargo build --release -p finance_assistant_api && \
    rm -f target/release/deps/finance_assistant_*

# Copy actual source code
COPY . .

# Final build
RUN cargo build --release -p finance_assistant_api

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --shell /bin/bash appuser
USER appuser

WORKDIR /app

COPY --from=builder /app/target/release/api ./api
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

CMD ["./api"]
