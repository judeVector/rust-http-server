# Stage 1: Build the application
FROM rust:1.73-slim-bullseye as builder

WORKDIR /app

# Install build dependencies and sccache for caching builds
RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev

# Copy only the files needed for dependency resolution to maximize caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the rest of the files and build the real app
COPY src ./src
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bullseye-slim

WORKDIR /app

# Create a non-root user and switch to it
RUN useradd -ms /bin/bash appuser
USER appuser

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/axum-railway-template /app/axum-railway-template

# Make sure the binary name matches your Cargo.toml package name
# If your package name is different, change it here
ENTRYPOINT ["/app/axum-railway-template"]
