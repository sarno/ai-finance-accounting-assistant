# ─── Multi-stage Rust build ───────────────────────────────────────────────────

# Stage 1: Build
FROM rust:1.78-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies layer (relative to workspace root)
COPY Cargo.toml Cargo.lock ./
COPY apps/api/crates/domain/Cargo.toml   apps/api/crates/domain/
COPY apps/api/crates/application/Cargo.toml apps/api/crates/application/
COPY apps/api/crates/infrastructure/Cargo.toml apps/api/crates/infrastructure/
COPY apps/api/crates/api/Cargo.toml      apps/api/crates/api/
COPY apps/api/crates/worker/Cargo.toml   apps/api/crates/worker/

# Create dummy source files to cache deps
RUN mkdir -p apps/api/crates/domain/src apps/api/crates/application/src apps/api/crates/infrastructure/src apps/api/crates/api/src apps/api/crates/worker/src && \
    echo "fn main() {}" > apps/api/crates/api/src/main.rs && \
    echo "fn main() {}" > apps/api/crates/worker/src/main.rs && \
    touch apps/api/crates/domain/src/lib.rs apps/api/crates/application/src/lib.rs apps/api/crates/infrastructure/src/lib.rs

RUN cargo build --release -p finance_assistant_api -p finance_assistant_worker && \
    rm -f target/release/deps/finance_assistant_*

# Copy actual source code
COPY . .

# Final build for both api and worker binaries
RUN cargo build --release -p finance_assistant_api -p finance_assistant_worker

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --shell /bin/bash appuser
RUN mkdir -p /app/uploads && chown -R appuser:appuser /app
USER appuser

WORKDIR /app

COPY --from=builder /app/target/release/api ./api
COPY --from=builder /app/target/release/worker ./worker
COPY --from=builder /app/apps/api/migrations ./migrations

EXPOSE 8080

CMD ["./api"]
