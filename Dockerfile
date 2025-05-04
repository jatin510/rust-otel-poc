FROM rust:latest as builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY Cargo.toml ./
# Create a dummy main.rs file and build to cache dependencies
RUN mkdir -p src && \
    echo 'fn main() { println!("Dummy build for dependencies"); }' > src/main.rs && \
    cargo build --release

# Remove the dummy source code and built binary
RUN rm -rf src target/release/deps/rust_otel* target/release/rust-otel*

# Copy your actual source code
COPY . .
# Build the real application
RUN cargo build --release

# Final stage
FROM ubuntu:22.04

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-otel /usr/local/bin/rust-otel

# Set the startup command
EXPOSE 8080
CMD ["rust-otel"] 