# syntax=docker/dockerfile:1

# -----------------------------------------------------------------------------
# Sync version information between core and web app
# -----------------------------------------------------------------------------
FROM cgr.dev/chainguard/wolfi-base:latest as base
WORKDIR /app
COPY . .
RUN apk update && apk add --no-cache --update-cache curl jq
RUN export PKG_WEB_VERSION=$(cat package.json | jq -r .version) &&\
 export APP_VERSION=$(sed -nE 's/^\s*version = "(.*?)"/\1/p' Cargo.toml) &&\
 sed -i "s/\"version\": \"$PKG_WEB_VERSION\"/\"version\": \"$APP_VERSION\"/" package.json

# -----------------------------------------------------------------------------
# Builder for Web UI
# -----------------------------------------------------------------------------
FROM cgr.dev/chainguard/node:20 AS buildweb
COPY --from=base --chown=node:node /app /app
RUN npm config set fund false && npm install --no-audit && npm run build

# -----------------------------------------------------------------------------
# Builder main application: https://endler.dev/2020/rust-compile-times
# -----------------------------------------------------------------------------
FROM cgr.dev/chainguard/rust:1.69 AS builder
WORKDIR /app
COPY --from=buildweb /app /app
RUN cargo build --release --bin fastrue --locked

# # Step 1: Install cargo-chef subcommand
# FROM cgr.dev/chainguard/rust:1.69 AS chef
# WORKDIR /app
# RUN cargo install cargo-chef --locked

# # Step 2: Compute a recipe file
# FROM chef as planner
# COPY --from=buildweb /app /app
# RUN cargo chef prepare --recipe-path recipe.json

# # Step 3: Cache dependencies and build
# FROM chef as builder
# COPY --from=planner /app/recipe.json recipe.json
# RUN cargo chef cook --release --recipe-path recipe.json
# RUN cargo build --release --bin fastrue --locked

# -----------------------------------------------------------------------------
# Final image: https://kerkour.com/rust-small-docker-image
# -----------------------------------------------------------------------------
LABEL org.opencontainers.image.source "https://github.com/riipandi/fastrue"
LABEL org.opencontainers.image.description "Fastrue is a headless authentication server."
FROM cgr.dev/chainguard/glibc-dynamic:latest as runner

ARG BIND_PORT 9090
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
