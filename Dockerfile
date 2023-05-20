# -----------------------------------------------------------------------------
# Builder
# -----------------------------------------------------------------------------
FROM rust:1.66-slim AS builder

ARG BUILD_DATE 2022-12-10T20:29:41Z
RUN rustup target add x86_64-unknown-linux-musl\
 && apt update && apt install -y musl-tools musl-dev\
 && update-ca-certificates

WORKDIR /app
COPY . .

# Use flag `x86_64-unknown-linux-musl` for building for Alpine or Scratch image
RUN cargo build --target x86_64-unknown-linux-musl --release

# -----------------------------------------------------------------------------
# Final image: https://kerkour.com/rust-small-docker-image
# -----------------------------------------------------------------------------
LABEL org.opencontainers.image.source="https://github.com/riipandi/fastrue"
FROM alpine:3.18 as runner

ARG FASTRUE_SECRET_KEY
ARG FASTRUE_DB_NAMESPACE
ARG DATABASE_URL
ARG FASTRUE_SMTP_HOST
ARG FASTRUE_SMTP_PORT
ARG FASTRUE_SMTP_USERNAME
ARG FASTRUE_SMTP_PASSWORD
ARG FASTRUE_SMTP_SECURE

ENV FASTRUE_SECRET_KEY $FASTRUE_SECRET_KEY
ENV FASTRUE_DB_NAMESPACE $FASTRUE_DB_NAMESPACE
ENV DATABASE_URL $DATABASE_URL
ENV FASTRUE_SMTP_HOST $FASTRUE_SMTP_HOST
ENV FASTRUE_SMTP_PORT $FASTRUE_SMTP_PORT
ENV FASTRUE_SMTP_USERNAME $FASTRUE_SMTP_USERNAME
ENV FASTRUE_SMTP_PASSWORD $FASTRUE_SMTP_PASSWORD
ENV FASTRUE_SMTP_SECURE $FASTRUE_SMTP_SECURE

ENV BIND_ADDR 0.0.0.0
ENV BIND_PORT 9999

WORKDIR /
RUN addgroup -g 1001 -S groot && adduser -S groot -u 1001

# Import compiled binaries from builder
COPY --from=builder --chown=groot:groot /app/target/x86_64-unknown-linux-musl/release/fastrue /usr/bin/

# Use an unprivileged user.
USER groot:groot
EXPOSE $BIND_PORT

ENTRYPOINT ["fastrue"]
