# Stage 1: Build the Rust application
FROM rust:latest AS builder

# Set the working directory in the container
WORKDIR /usr/src/rust-hello-server

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the container
COPY src ./src

# Build the Rust project in release mode
RUN cargo build --release --bin rust-hello-server

# Debug: List the files in the target/release directory
RUN ls -l /usr/src/rust-hello-server/target/release

# Stage 2: Create a minimal runtime image
FROM alpine:latest

# Set the working directory in the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/rust-hello-server/target/release/rust-hello-server .

# Debug: List the files in the /app directory
RUN ls -l /app

# Set the entry point of the container
CMD ["./rust-hello-server"]