# Use the latest Rust image
FROM rust:bookworm AS builder
# Set the working directory
WORKDIR /app
# Copy the current directory contents into the container at /usr/src/app
COPY . .
# Build the Rust app
RUN cargo build --release

# Start a new stage from scratch
FROM debian:bookworm
WORKDIR /app
# Copy the built app from the builder stage
COPY --from=builder /app/target/release/backend .
COPY *.html .
# Expose the web server port
EXPOSE 7878
# Command to run the executable
CMD ["./backend"]