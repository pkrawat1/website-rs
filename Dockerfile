# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# If you’re using stable, use this instead
# FROM rust:1.70-bullseye as builder

RUN apt update && \
    apt install -y binaryen npm protobuf-compiler libssl-dev pkg-config musl-tools \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN rustup component add clippy

RUN cargo install cargo-generate
RUN cargo install cargo-leptos
RUN cargo install wasm-opt

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-buster-slim as runner
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/website_rs /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
# Run the server
CMD ["/app/website_rs"]

