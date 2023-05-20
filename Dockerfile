# -----------------------------------------------------------------------------
# Builder
# -----------------------------------------------------------------------------
FROM rust:1.66-slim AS builder

ARG BUILD_DATE 2022-12-10T20:29:41Z
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
LABEL org.opencontainers.image.source="https://github.com/riipandi/fastrue"
FROM alpine:3.17 as runner

ARG TRUSTY_SECRET_KEY
ARG TRUSTY_DB_NAMESPACE
ARG DATABASE_URL
ARG TRUSTY_SMTP_HOST
ARG TRUSTY_SMTP_PORT
ARG TRUSTY_SMTP_USERNAME
ARG TRUSTY_SMTP_PASSWORD
ARG TRUSTY_SMTP_SECURE

ENV BIND_PORT 9999
ENV BIND_ADDR 0.0.0.0
ENV TRUSTY_SECRET_KEY $TRUSTY_SECRET_KEY
ENV TRUSTY_DB_NAMESPACE $TRUSTY_DB_NAMESPACE
ENV DATABASE_URL $DATABASE_URL
ENV TRUSTY_SMTP_HOST $TRUSTY_SMTP_HOST
ENV TRUSTY_SMTP_PORT $TRUSTY_SMTP_PORT
ENV TRUSTY_SMTP_USERNAME $TRUSTY_SMTP_USERNAME
ENV TRUSTY_SMTP_PASSWORD $TRUSTY_SMTP_PASSWORD
ENV TRUSTY_SMTP_SECURE $TRUSTY_SMTP_SECURE

WORKDIR /
RUN addgroup -g 1001 -S groot && adduser -S groot -u 1001

# Import compiled binaries from builder
COPY --from=builder --chown=groot:groot /app/target/x86_64-unknown-linux-musl/release/fastrue /usr/bin/

# Use an unprivileged user.
USER groot:groot
EXPOSE $BIND_PORT

ENTRYPOINT ["fastrue"]
