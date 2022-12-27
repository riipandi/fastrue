# -----------------------------------------------------------------------------
# References: https://kerkour.com/rust-small-docker-image
# -----------------------------------------------------------------------------
# Builder: docker build -t wasta:alpine -f Dockerfile .
# -----------------------------------------------------------------------------
FROM rust:1.66-slim AS builder

RUN rustup target add x86_64-unknown-linux-musl \
  && apt update && apt install -y musl-tools musl-dev \
  && update-ca-certificates

WORKDIR /app
COPY . .

# Use flag `x86_64-unknown-linux-musl` for building for Alpine or Scratch image
RUN cargo build --target x86_64-unknown-linux-musl --release

# -----------------------------------------------------------------------------
# Final image: docker run -it --rm -p 3030:3030 wasta:alpine
# -----------------------------------------------------------------------------
LABEL org.opencontainers.image.source="https://github.com/riipandi/wasta"
FROM alpine:3.17 as runner

WORKDIR /
RUN addgroup -g 1001 -S groot && adduser -S groot -u 1001
# RUN adduser --disabled-password --gecos "" --home "/nonexistent" \
#   --shell "/sbin/nologin" --no-create-home --uid 1001 groot

# Import from builder.
COPY --from=builder --chown=groot:groot /app/target/x86_64-unknown-linux-musl/release/wasta .

# Use an unprivileged user.
USER groot:groot

CMD ["/wasta"]
