# syntax=docker/dockerfile:1

# -----------------------------------------------------------------------------
# Sync version information and build the frontend
# -----------------------------------------------------------------------------
FROM node:20-alpine as base
WORKDIR /app
COPY --chown=node:node . .
RUN apk update && apk add --no-cache --update-cache jq
RUN export PKG_WEB_VERSION=$(cat package.json | jq -r .version) &&\
    export APP_VERSION=$(sed -nE 's/^\s*version = "(.*?)"/\1/p' Cargo.toml) &&\
    sed -i "s/\"version\": \"$PKG_WEB_VERSION\"/\"version\": \"$APP_VERSION\"/" package.json
RUN npm config set fund false && npm install --no-audit && npm run build

# -----------------------------------------------------------------------------
# Builder main application: https://endler.dev/2020/rust-compile-times
# -----------------------------------------------------------------------------
FROM cgr.dev/chainguard/rust:1.69 AS builder
WORKDIR /app
COPY --chown=$(whoami): --from=base /app /app
RUN cargo build --release --locked --bin fastrue

# -----------------------------------------------------------------------------
# Final image: https://kerkour.com/rust-small-docker-image
# -----------------------------------------------------------------------------
LABEL org.opencontainers.image.source "https://github.com/riipandi/fastrue"
LABEL org.opencontainers.image.description "Fastrue is a headless authentication server, inspired from Netlify GoTrue."
FROM cgr.dev/chainguard/glibc-dynamic:latest as runner

ARG BIND_PORT 9090
ARG BIND_ADDR 0.0.0.0
ARG DATABASE_URL
ARG DATABASE_AUTO_MIGRATE
ARG FASTRUE_SECRET_KEY
ARG FASTRUE_HEADLESS_MODE

ENV BIND_PORT $BIND_PORT
ENV BIND_ADDR $BIND_ADDR
ENV DATABASE_URL $DATABASE_URL
ENV DATABASE_AUTO_MIGRATE $DATABASE_AUTO_MIGRATE
ENV FASTRUE_SECRET_KEY $FASTRUE_SECRET_KEY
ENV FASTRUE_HEADLESS_MODE $FASTRUE_HEADLESS_MODE

# Import compiled binaries from builder
COPY --from=builder /app/target/release/fastrue /sbin/fastrue

EXPOSE $BIND_PORT

ENTRYPOINT ["/sbin/fastrue", "--port", "${BIND_PORT}"]
