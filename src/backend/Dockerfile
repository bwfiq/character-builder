# Use the latest Rust image
FROM rust:1.66 as builder
# Set the working directory
WORKDIR /usr/src/backend
# Copy the current directory contents into the container at /usr/src/app
COPY . .
# Build the Rust app
RUN cargo build --release

# Start a new stage from scratch
FROM debian:buster
WORKDIR /usr/src/app
# Copy the built app from the builder stage
COPY --from=builder /usr/src/backend/target/release/backend .
# Command to run the executable
CMD ["./backend"]