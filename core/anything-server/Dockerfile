FROM rust:1.83.0-bullseye as builder

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

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/anything-server ./anything-server

# Copy the src directory structure to maintain the expected path
COPY ./src/system_plugins/registry/schemas ./src/system_plugins/registry/schemas

CMD ["./anything-server"]

EXPOSE 3001