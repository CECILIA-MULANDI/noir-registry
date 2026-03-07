# Build stage
FROM rust:1.89 AS builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY server ./server
COPY cli-tool ./cli-tool

# Build only the server binary in release mode
RUN cargo build --release --bin noir-registry-server

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/noir-registry-server /usr/local/bin/noir-registry-server

ENV PORT=8080
EXPOSE 8080

CMD ["noir-registry-server"]
