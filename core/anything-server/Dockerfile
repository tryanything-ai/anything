# Build stage
FROM rust:1.76-bullseye as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    python3 \
    curl \
    git \
    unzip \
    libc6-dev

WORKDIR /usr/src/app

# Copy only Cargo.toml first
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/anything-server /app/anything-server

# Set the binary as the entrypoint
ENTRYPOINT ["/app/anything-server"]