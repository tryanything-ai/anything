FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

# Create and change to the app directory.
WORKDIR /app

FROM chef AS planner
COPY . ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . ./
RUN cargo build --release

CMD ["./target/release/anything-server"]


EXPOSE 3001