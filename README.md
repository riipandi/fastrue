# Fastrue Authentication Server

[![License](https://badgers.space/github/license/riipandi/fastrue?color=green&corner_radius=0)](./LICENSE)
[![GitHub contributors](https://badgers.space/github/contributors/riipandi/fastrue?color=green&corner_radius=0)](https://github.com/riipandi/fastrue/graphs/contributors)
[![Contributions](https://img.shields.io/badge/Contributions-welcome-blue.svg?style=flat-square)](./CODE_OF_CONDUCT.md)
[![(Rust)](https://img.shields.io/badge/rust-v1.69-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![GitHub release](https://img.shields.io/github/v/release/riipandi/fastrue?logo=rust&style=flat-square)](https://github.com/riipandi/fastrue)
[![Twitter Badge](https://badgen.net/badge/icon/Follow%20Twitter?icon=twitter&label&color=blue&labelColor=black&style=flat-square)](https://twitter.com/riipandi)
[![Netlify Status](https://api.netlify.com/api/v1/badges/a8f331bd-3c3a-4080-84a3-70cebb40480c/deploy-status)](https://app.netlify.com/sites/fastrue/deploys)

<hr/>

Fastrue (formerly Trusty) is a headless authentication server inspired from Netlify GoTrue, built with [Rust](https://www.rust-lang.org/).

> **WARNING!** This project still in development.
> Everything is experimental, breaking changes can happen and the long-term purpose of this project is not yet clear, use at your own risk!

## 🏁 Quick Start

### Prerequisites

The following are required to run the application in development or in production environment:

- [Rust](https://www.rust-lang.org/tools/install) v1.69 or greater.
- [Node.js](https://nodejs.org/en/download) v18.16.0 or greater.
- [PostgreSQL](https://www.postgresql.org/download/) PostgreSQL server v14 or greater.
- [sqlx](https://crates.io/crates/sqlx) for interacting with the database.
- [sqlx-cli](https://crates.io/crates/sqlx-cli) a command line tool for sqlx.
- [cargo watch](https://crates.io/crates/cargo-watch), a tool for watching the project files and recompiling when they change.
- [cargo-make](https://sagiegurari.github.io/cargo-make/#installation), task runner and build tool for Rust project.
- [toml-cli](https://github.com/gnprice/toml-cli), a simple CLI for editing and querying TOML files.
- [Docker](https://docs.docker.com/engine/install), v2.10 or greater. This is optional, only required when building container image.

### Generate Secret Key

Before you continue, you need to create `.env` file (you can duplicate `.env.example`) and
fill the `application secret key` with some random string. To generate a secret key, use
the following command:

```sh
cargo run -- generate-secret
```

### Run Database Migration

```sh
cargo run -- migrate
```

### Up and Running

```sh
docker-compose -f compose-development.yaml up -d
```

```sh
cargo make dev             # run in development
cargo make build           # build binary file
cargo make docker-build    # build docker container
cargo make docker-run      # run the docker container
```

Application will run at `http://localhost:9999`

## 🧑🏻‍💻 Development

To run the application in development mode, follow the steps below:

1. Clone this repository.
2. Copy `.env.example` to `.env`
3. Change the `FASTRUE_SECRET_KEY` and `DATABASE_URL` variables value.
4. Run `sqlx database create` to create the database from the specified `DATABASE_URL`
5. Run `sqlx migrate run` to run the migrations
6. use `sqlx migrate add -r <migration_name>` to add a new migration

**Note**: Use `sqlx database drop` to revert the change

### Publish Docker Image

```sh
echo $GH_TOKEN | docker login ghcr.io --username CHANGEME --password-stdin
```

### Simple Load Testing

Using [`hey`](https://github.com/rakyll/hey) to perform a simple load testing.

```sh
hey -m GET -n 200 -z 10s http://127.0.0.1:9999/api
```

## 🚀 Deployment

Please see the [documentation page](https://fastrue.netlify.app/docs/getting-started/introduction/) for more detailed information.

## 🧑🏻‍💻 Contributing

Welcome, and thank you for your interest in contributing to Fastrue! There are many ways in which you can contribute,
beyond writing code. You can read this repository’s [Contributing Guidelines](./CONTRIBUTING.md) to learn how to contribute.

## Maintainer

Currently, [Aris Ripandi](htps://ripandis.com) ([@riipandi](https://twitter.com/riipandi)) is the only maintainer.

## License

This project is open-sourced software licensed under the [Apache License 2.0][choosealicense].

Copyrights in this project are retained by their contributors.

See the [license file](./LICENSE) for more information.

[choosealicense]: https://choosealicense.com/licenses/apache-2.0/

---

<sub>🤫 Psst! If you like my work you can support me via [GitHub sponsors](https://github.com/sponsors/riipandi).</sub>
