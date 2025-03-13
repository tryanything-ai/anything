FROM rust:1.83.0-bullseye as builder

# Set environment variables for build
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV RUSTFLAGS="-C target-cpu=native -C opt-level=3"
ENV CARGO_PROFILE_RELEASE_LTO=true
ENV CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
ENV RUST_MIN_STACK=8388608

WORKDIR /app

# Copy only the dependency files first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Now copy the real source code
COPY ./src ./src

# Clean and rebuild with actual source
RUN cargo clean && cargo build --release

FROM debian:bullseye-slim

# Set runtime environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV RUST_MIN_STACK=8388608
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=3001

# Install required packages
RUN apt-get update && apt-get install -y \
    tini \
    libssl1.1 \
    ca-certificates \
    libpthread-stubs0-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/anything-server ./anything-server

# Copy the src directory structure to maintain the expected path
COPY ./src/system_plugins/registry/schemas ./src/system_plugins/registry/schemas

# Add healthcheck
# HEALTHCHECK --interval=30s --timeout=3s \
#     CMD curl -f http://localhost:3001/health || exit 1

# Use tini as entrypoint
ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["./anything-server"]

EXPOSE 3001