# Wasta Authentication Server

[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/otentikauth/wasta?logo=deno&style=flat-square)](https://github.com/otentikauth/wasta)
[![(Rust)](https://img.shields.io/badge/rust-v1.66-orange.svg?style=flat-square&logo=deno)](https://deno.land)
[![Language](https://img.shields.io/github/languages/top/otentikauth/wasta?style=flat-square)](https://github.com/otentikauth/wasta)
[![License](https://img.shields.io/github/license/otentikauth/wasta?style=flat-square)][choosealicense]
[![GitHub Sponsors](https://img.shields.io/static/v1?color=26B643&label=Sponsor&message=%E2%9D%A4&logo=GitHub&style=flat-square)](https://github.com/sponsors/riipandi)

<hr/>

Headless authentication server for securing your apps, inspired by Netlify GoTrue, but built with [Rust](https://www.rust-lang.org/).

> **_WARNING! This project still WIP._**

## 🏁 Quick Start

### Prerequisites

The following are required to run the application in development or in production environment

-   [Rust](https://www.rust-lang.org/tools/install) v1.66 or greater.
-   [PostgreSQL](https://www.postgresql.org/download/) PostgreSQL server v14 or greater.
-   [sqlx](https://crates.io/crates/sqlx) for interacting with the database.
-   [sqlx-cli](https://crates.io/crates/sqlx-cli) a command line tool for sqlx.
-   [cargo watch](https://crates.io/crates/cargo-watch), a tool for watching the project files and recompiling when they change.
-   [Docker](https://docs.docker.com/engine/install), v2.10 or greater. This is optional, only required when building container image.

### Generate Secret Key

Before you continue, you need to create `.env` file (you can duplicate `.env.example`) and
fill the `application secret key` with some random string. To generate a secret key, use
the following command:

```sh
openssl rand -base64 500 | tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1
```

### Up and Running

```sh
make dev             # run in development
make build           # build binary file
make docker-build    # build docker container
make docker-run      # run the docker container
```

Application will run at `http://localhost:3030`

## 🧑🏻‍💻 Development

To run the application in development mode, follow the steps below:

1. Clone Repository.
2. Copy `.env.example` to `.env`
3. Change the `WASTA_SECRET` and `DATABASE_URL` variables value.
4. Run `sqlx database create` to create the database from the specified `DATABASE_URL`
5. Run `sqlx migrate run` to run the migrations
6. use `sqlx migrate add <migration_name>` to add a new migration

**Note**: Use `sqlx database drop` to revert the change

## 🚀 Deployment

> **_TODO!_**

## Maintainer

Currently, [Aris Ripandi](htps://ripandis.com) ([@riipandi](https://twitter.com/riipandi)) is the only maintainer.

## License

This project is open-sourced software licensed under the [Apache License 2.0][choosealicense].

Copyrights in this project are retained by their contributors.

See the [license file](./LICENSE) for more information.

[choosealicense]: https://choosealicense.com/licenses/apache-2.0/
