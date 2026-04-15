# Stage 1: Build the application
FROM rust:1.94-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    wget \
    tar \
    curl \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-leptos using the installer script for better platform compatibility
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.2/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Create a workspace directory
WORKDIR /app

# Copy the workspace manifest and source code
COPY . .

# Build the application in release mode
# cargo-leptos handles building both the server and the frontend (WASM)
RUN cargo leptos build --release

# Stage 2: Run the application
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the server binary and the site-root (static assets and WASM) from the builder stage
# The binary is named 'showcase' as per Cargo.toml bin-package
COPY --from=builder /app/target/release/showcase /app/server
COPY --from=builder /app/target/release/hash.txt /app/
COPY --from=builder /app/target/site /app/site
RUN mkdir /app/exports

# Set environment variables for the Leptos runtime
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_ENV=PROD
ENV LEPTOS_HASH_FILES=true

# Expose the application port
EXPOSE 3000

# Run the server
CMD ["/app/server"]
