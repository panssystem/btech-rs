# Use the official Rust image as the base
FROM rust:1.88.1-slim AS builder

WORKDIR /app

# Copy source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image for the final container
FROM alpine:3.20

# Install necessary runtime dependencies for Rust binaries
RUN apk add --no-cache libgcc

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/loader /app/loader

# Set the startup command
CMD ["./loader"]