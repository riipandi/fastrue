# syntax=docker/dockerfile:1.4

# Arguments with default value (for build).
ARG NODE_ENV=production
ARG NODE_VERSION=20

# Arguments for envars in runner step.
ARG DATABASE_URL

# -----------------------------------------------------------------------------
# This is base image with `pnpm` package manager
# -----------------------------------------------------------------------------
FROM node:${NODE_VERSION}-alpine AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN apk update && apk add --no-cache jq libc6-compat
RUN corepack enable && corepack prepare pnpm@latest-8 --activate
RUN pnpm add -g turbo
WORKDIR /app

# -----------------------------------------------------------------------------
# Extract the target application package
# -----------------------------------------------------------------------------
FROM base AS pruner
WORKDIR /app
COPY --chown=node:node . .
RUN turbo prune --scope=standalone-server --scope=portal --docker

# -----------------------------------------------------------------------------
# Add lockfile and package.json's of isolated subworkspace
# -----------------------------------------------------------------------------
FROM base AS builder
WORKDIR /app

ARG TURBO_TEAM
ARG TURBO_TOKEN
ENV TURBO_TEAM $TURBO_TEAM
ENV TURBO_TOKEN $TURBO_TOKEN

# First install the dependencies (as they change less often)
COPY --from=pruner /app/out/json/ .
COPY --from=pruner /app/out/full/ ./
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install
RUN turbo run build --no-cache --filter=!website

# Prune pnpm package, then install dependencies for production
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm --filter=standalone-server --prod deploy pruned

# -----------------------------------------------------------------------------
# Production image, copy all the files and run the application
# -----------------------------------------------------------------------------
FROM node:${NODE_VERSION}-alpine AS runner
LABEL org.opencontainers.image.source="https://github.com/riipandi/fastrue"
WORKDIR /app

ENV DATABASE_URL $DATABASE_URL

ENV NODE_ENV production
ENV HOST 0.0.0.0
ENV PORT 8090

# Don't run production as root, spawns command as a child process.
RUN addgroup --system --gid 1001 nonroot && adduser --system --uid 1001 nonroot
RUN apk update && apk add --no-cache tini

COPY --from=builder --chown=nonroot:nonroot /app/pruned/package.json ./package.json
COPY --from=builder --chown=nonroot:nonroot /app/pruned/node_modules ./node_modules
COPY --from=builder --chown=nonroot:nonroot /app/pruned/dist ./dist
COPY --from=builder --chown=nonroot:nonroot /app/apps/portal/dist ./public

USER nonroot:nonroot
EXPOSE $PORT

ENTRYPOINT ["/sbin/tini", "--"]
CMD ["node", "dist/app.js"]
