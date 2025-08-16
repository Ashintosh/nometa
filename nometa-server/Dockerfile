# ---------------------
# Stage 1: Build
# ---------------------
FROM rust:1.86-alpine as builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src ./src
RUN cargo build --release

# ---------------------
# Stage 2: Runtime
# ---------------------
FROM alpine:latest

# Install libssl and ca-certificates (needed by Rust HTTPS clients)
RUN apk add --no-cache libssl3 ca-certificates curl

COPY --from=builder /app/target/release/nometa-server /usr/local/bin/nometa-server

EXPOSE 8080
ENTRYPOINT [ "/usr/local/bin/nometa-server" ]