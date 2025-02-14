FROM rust:1.83.0-bullseye as builder

RUN cargo new  --bin anything-server
WORKDIR ./anything-server

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# Static Files used to serve base action templates
COPY ./template_db ./template_db 

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /anything-server/target/release/anything-server ./anything-server

# Copy the template_db directory
COPY ./template_db ./template_db

CMD ["./anything-server"]

EXPOSE 3001