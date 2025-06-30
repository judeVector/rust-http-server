# Stage 1: Build
FROM rust:1.73-slim-bullseye as builder
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy actual source code
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim
WORKDIR /app

RUN useradd -ms /bin/bash appuser
USER appuser

COPY --from=builder /app/target/release/rust-http-server /app/rust-http-server

ENTRYPOINT ["/app/rust-http-server"]
