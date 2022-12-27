# -----------------------------------------------------------------------------
# Builder
# -----------------------------------------------------------------------------
FROM rust:1.66-slim AS builder

ARG BUILD_VERSION 0.0.1

RUN rustup target add x86_64-unknown-linux-musl \
  && apt update && apt install -y musl-tools musl-dev \
  && update-ca-certificates

WORKDIR /app
COPY . .

# Use flag `x86_64-unknown-linux-musl` for building for Alpine or Scratch image
RUN cargo build --target x86_64-unknown-linux-musl --release

# -----------------------------------------------------------------------------
# Final image: https://kerkour.com/rust-small-docker-image
# -----------------------------------------------------------------------------
LABEL org.opencontainers.image.source="https://github.com/riipandi/wasta"
FROM alpine:3.17 as runner

ENV BIND_PORT 3030
ENV BIND_ADDR 0.0.0.0

WORKDIR /
RUN addgroup -g 1001 -S groot && adduser -S groot -u 1001

# Import compiled binaries from builder
COPY --from=builder --chown=groot:groot /app/target/x86_64-unknown-linux-musl/release/wasta /usr/bin/

# Use an unprivileged user.
USER groot:groot
EXPOSE $BIND_PORT

ENTRYPOINT ["wasta"]
