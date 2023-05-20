# syntax=docker/dockerfile:1

# -----------------------------------------------------------------------------
# Builder for Web UI
# -----------------------------------------------------------------------------
FROM cgr.dev/chainguard/node:18 AS buildweb
COPY --chown=node:node . .
RUN npm config set loglevel error && npm install --no-audit && npm run build

# -----------------------------------------------------------------------------
# Builder main application
# -----------------------------------------------------------------------------
FROM cgr.dev/chainguard/rust:1.69 AS builder
WORKDIR /app
COPY --from=buildweb /app /app
RUN cargo build --release

# -----------------------------------------------------------------------------
# Final image: https://kerkour.com/rust-small-docker-image
# -----------------------------------------------------------------------------
LABEL org.opencontainers.image.source="https://github.com/riipandi/fastrue"
FROM cgr.dev/chainguard/glibc-dynamic:latest as runner

ARG DATABASE_URL
ARG FASTRUE_SECRET_KEY
ARG FASTRUE_DB_NAMESPACE
ARG FASTRUE_AUTO_MIGRATE
ARG FASTRUE_HEADLESS_MODE
ARG FASTRUE_SMTP_HOST
ARG FASTRUE_SMTP_PORT
ARG FASTRUE_SMTP_USERNAME
ARG FASTRUE_SMTP_PASSWORD
ARG FASTRUE_SMTP_SECURE

ENV DATABASE_URL $DATABASE_URL
ENV FASTRUE_SECRET_KEY $FASTRUE_SECRET_KEY
ENV FASTRUE_DB_NAMESPACE $FASTRUE_DB_NAMESPACE
ENV FASTRUE_AUTO_MIGRATE $FASTRUE_AUTO_MIGRATE
ENV FASTRUE_HEADLESS_MODE $FASTRUE_HEADLESS_MODE
ENV FASTRUE_SMTP_HOST $FASTRUE_SMTP_HOST
ENV FASTRUE_SMTP_PORT $FASTRUE_SMTP_PORT
ENV FASTRUE_SMTP_USERNAME $FASTRUE_SMTP_USERNAME
ENV FASTRUE_SMTP_PASSWORD $FASTRUE_SMTP_PASSWORD
ENV FASTRUE_SMTP_SECURE $FASTRUE_SMTP_SECURE

ENV BIND_ADDR 0.0.0.0
ENV BIND_PORT 9999

# Import compiled binaries from builder
COPY --from=builder /app/target/release/fastrue /sbin/fastrue

EXPOSE $BIND_PORT

ENTRYPOINT ["/sbin/fastrue"]
