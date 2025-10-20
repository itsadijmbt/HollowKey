# We start with the official Rust image that matches our version contract
FROM rust:1.90 AS builder

# Set the working directory inside the container. This is where our code will live.
WORKDIR /usr/src/app

# Copy our local project files into the container.
COPY . .

# Build our application in release mode for maximum performance.
RUN cargo install --path .

# Now, we start fresh with a minimal, secure base image.
FROM debian:bookworm-slim

# Copy *only* the single compiled application binary from the "builder" stage.
COPY --from=builder /usr/local/cargo/bin/hollowkey_api /usr/local/bin/hollowkey_api

# Expose the port that our Axum server is listening on. This is like opening a port in a firewall.
EXPOSE 3001

# It simply executes our compiled application.
CMD ["hollowkey_api"]