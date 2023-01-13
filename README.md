# Trusty Authentication Server

[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/riipandi/icondns?logo=rust&style=flat-square)](https://github.com/riipandi/icondns)
[![(Rust)](https://img.shields.io/badge/rust-v1.66-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Language](https://img.shields.io/github/languages/top/riipandi/trusty?style=flat-square)](https://github.com/riipandi/trusty)
[![License](https://img.shields.io/github/license/riipandi/trusty?style=flat-square)][choosealicense]
[![GitHub Sponsors](https://img.shields.io/static/v1?color=26B643&label=Sponsor&message=%E2%9D%A4&logo=GitHub&style=flat-square)](https://github.com/sponsors/riipandi)

<hr/>

Headless authentication server for securing your apps, inspired by Netlify GoTrue, but built with [Rust](https://www.rust-lang.org/).

> **WARNING!** This project still in development.
> Everything is experimental, breaking changes can happen and the long-term purpose of this project is not yet clear, use at your own risk!

## 🏁 Quick Start

### Prerequisites

The following are required to run the application in development or in production environment:

- [Rust](https://www.rust-lang.org/tools/install) v1.66 or greater.
- [PostgreSQL](https://www.postgresql.org/download/) PostgreSQL server v14 or greater.
- [sqlx](https://crates.io/crates/sqlx) for interacting with the database.
- [sqlx-cli](https://crates.io/crates/sqlx-cli) a command line tool for sqlx.
- [cargo watch](https://crates.io/crates/cargo-watch), a tool for watching the project files and recompiling when they change.
- [Docker](https://docs.docker.com/engine/install), v2.10 or greater. This is optional, only required when building container image.

### Generate Secret Key

Before you continue, you need to create `.env` file (you can duplicate `.env.example`) and
fill the `application secret key` with some random string. To generate a secret key, use
the following command:

```sh
cargo run -- generate-secret
```

### Up and Running

```sh
make dev             # run in development
make build           # build binary file
make docker-build    # build docker container
make docker-run      # run the docker container
```

Application will run at `http://localhost:9999`

## 🧑🏻‍💻 Development

To run the application in development mode, follow the steps below:

1. Clone this repository.
2. Copy `.env.example` to `.env`
3. Change the `TRUSTY_SECRET_KEY` and `DATABASE_URL` variables value.
4. Run `sqlx database create` to create the database from the specified `DATABASE_URL`
5. Run `sqlx migrate run` to run the migrations
6. use `sqlx migrate add -r <migration_name>` to add a new migration

**Note**: Use `sqlx database drop` to revert the change

## 🚀 Deployment

Please see the [documentation page](https://trusty-auth.netlify.app/docs/deployment/) for more detailed information.

## 👀 Endpoints

| Method | Path                   | Description                                                                                                                                                                                                                                        |
| ------ | ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| GET    | /health                | System health check                                                                                                                                                                                                                                |
| GET    | /settings              | Returns the publicly available settings for this gotrue instance                                                                                                                                                                                   |
| POST   | /admin/users/<user_id> | Creates the user based on the user_id specified.                                                                                                                                                                                                   |
| PUT    | /admin/users/<user_id> | Updates the user based on the user_id specified.                                                                                                                                                                                                   |
| POST   | /admin/generate_link   | Returns the corresponding email action link based on the type specified.                                                                                                                                                                           |
| POST   | /signup                | Register a new user with an email and password.                                                                                                                                                                                                    |
| POST   | /invite                | Invites a new user with an email. This endpoint requires the `service_role` or `trusty_admin` JWT set as an Auth Bearer header                                                                                                                     |
| POST   | /verify                | Verify a registration or a password recovery. Type can be `signup` or `recovery` or `invite` and the token is a token returned from either `/signup` or `/recover`. `password` is required for signup verification if no existing password exists. |
| GET    | /verify                | Verify a registration or a password recovery. Type can be `signup` or `recovery` or `magiclink` or `invite` and the token is a token returned from either `/signup` or `/recover` or `/magiclink`.                                                 |
| POST   | /otp                   | One-Time-Password. Will deliver a magiclink or sms otp to the user depending on whether the request body contains an `email or `phone` key.                                                                                                        |
| POST   | /magiclink             | Magic Link. Will deliver a link (e.g. `/verify?type=magiclink&token=aabbccddee1234`) to the user based on email address which they can use to redeem an `access_token`.                                                                            |
| POST   | /recover               | Password recovery. Will deliver a password recovery mail to the user based on email address.                                                                                                                                                       |
| POST   | /token                 | This is an OAuth2 endpoint that currently implements the password and refresh_token grant types.                                                                                                                                                   |
| GET    | /user                  | Get the JSON object for the logged in user (requires authentication)                                                                                                                                                                               |
| PUT    | /user                  | Update a user (Requires authentication). Apart from changing email/password, this method can be used to set custom user data. Changing the email will result in a magiclink being sent out.                                                        |
| GET    | /reauthenticate        | Sends a nonce to the user's email (preferred) or phone. This endpoint requires the user to be logged in / authenticated first. The user needs to have either an email or phone number for the nonce to be sent successfully.                       |
| POST   | /logout                | Logout a user (Requires authentication). This will revoke all refresh tokens for the user. Remember that the JWT tokens will still be valid for stateless auth until they expires.                                                                 |
| GET    | /authorize             | Get `access_token` from external oauth provider.                                                                                                                                                                                                   |
| GET    | /callback              | External provider should redirect to here.                                                                                                                                                                                                         |

For more detailed API documentation, go to: [`http://localhost:9999/swagger`](http://localhost:9999/swagger)

## 🧑🏻‍💻 Contributing

Welcome, and thank you for your interest in contributing to Trusty! There are many ways in which you can contribute,
beyond writing code. You can read this repository’s [Contributing Guidelines](./CONTRIBUTING.md) to learn how to contribute.

## Maintainer

Currently, [Aris Ripandi](htps://ripandis.com) ([@riipandi](https://twitter.com/riipandi)) is the only maintainer.

## License

This project is open-sourced software licensed under the [Apache License 2.0][choosealicense].

Copyrights in this project are retained by their contributors.

See the [license file](./LICENSE) for more information.

[choosealicense]: https://choosealicense.com/licenses/apache-2.0/

---

<sub>🤫 Psst! You can [support my work here](https://github.com/sponsors/riipandi).</sub>
