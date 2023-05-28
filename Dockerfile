# syntax=docker/dockerfile:1

# -----------------------------------------------------------------------------
# Sync version information between core and web app
# -----------------------------------------------------------------------------
FROM rust:1.69-slim-bullseye AS prebuild
ARG TOML_CLI_PKG="https://github.com/gnprice/toml-cli/releases/download/v0.2.3/toml-0.2.3-x86_64-linux.tar.gz"
RUN apt-get update && apt-get -y install jq wget && wget -O /tmp/toml.tar.gz ${TOML_CLI_PKG} &&\
 tar -xzvf /tmp/toml.tar.gz && rm -f /tmp/toml.tar.gz &&\
 chmod +x toml-0.2.3-x86_64-linux/toml &&\
 mv toml-0.2.3-x86_64-linux/toml /bin/toml
WORKDIR /app
COPY . .
RUN \
 export APP_VERSION="$(toml get Cargo.toml package.version --raw)" &&\
 export PKG_WEB_VERSION="$(cat package.json | jq -r .version)" &&\
 sed -i "s/${PKG_WEB_VERSION}/${APP_VERSION}/" package.json

# -----------------------------------------------------------------------------
# Builder for Web UI
# -----------------------------------------------------------------------------
FROM cgr.dev/chainguard/node:20 AS buildweb
COPY --from=prebuild --chown=node:node /app /app
RUN npm config set fund false && npm install --no-audit && npm run build

# -----------------------------------------------------------------------------
# Builder main application
# -----------------------------------------------------------------------------
FROM cgr.dev/chainguard/rust:1.69 AS builder
ARG TARGETPLATFORM
WORKDIR /app
COPY --from=buildweb /app /app
RUN cargo build --release

# -----------------------------------------------------------------------------
# Final image: https://kerkour.com/rust-small-docker-image
# -----------------------------------------------------------------------------
LABEL org.opencontainers.image.source "https://github.com/riipandi/fastrue"
LABEL org.opencontainers.image.description "Fastrue is a headless authentication server."
FROM cgr.dev/chainguard/glibc-dynamic:latest as runner

ARG BIND_PORT 9999
ARG BIND_ADDR 0.0.0.0
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

ENV BIND_PORT $BIND_PORT
ENV BIND_ADDR $BIND_ADDR
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

# Import compiled binaries from builder
COPY --from=builder /app/target/release/fastrue /sbin/fastrue

EXPOSE $BIND_PORT

ENTRYPOINT ["/sbin/fastrue"]
