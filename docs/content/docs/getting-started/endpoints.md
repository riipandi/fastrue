+++
title = "API Endpoint"
description = "List of API endpoints."
date = 2021-05-01T08:20:00+00:00
updated = 2021-05-01T08:20:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "List of all API endpoints."
toc = true
top = false
+++

| Method   | Path                     | Description                                                                                                                                                                                                                                  |
| -------- | ------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **GET**  | `/health`                | System health check                                                                                                                                                                                                                          |
| **GET**  | `/settings`              | Returns the publicly available settings for this gotrue instance                                                                                                                                                                             |
| **POST** | `/admin/users/<user_id>` | Creates the user based on the user_id specified.                                                                                                                                                                                             |
| **PUT**  | `/admin/users/<user_id>` | Updates the user based on the user_id specified.                                                                                                                                                                                             |
| **POST** | `/admin/generate_link`   | Returns the corresponding email action link based on the type specified.                                                                                                                                                                     |
| **POST** | `/signup`                | Register a new user with an email and password.                                                                                                                                                                                              |
| **POST** | `/invite`                | Invites a new user with an email. This endpoint requires the`service_role` or`fastrue_admin` JWT set as an Auth Bearer header                                                                                                                 |
| **POST** | `/verify`                | Verify a registration or a password recovery. Type can be`signup` or`recovery` or`invite` and the token is a token returned from either`/signup` or`/recover`.`password` is required for signup verification if no existing password exists. |
| **GET**  | `/verify`                | Verify a registration or a password recovery. Type can be`signup` or`recovery` or`magiclink` or`invite` and the token is a token returned from either`/signup` or`/recover` or`/magiclink`.                                                  |
| **POST** | `/otp`                   | One-Time-Password. Will deliver a magiclink or sms otp to the user depending on whether the request body contains an`email or`phone` key.                                                                                                    |
| **POST** | `/magiclink`             | Magic Link. Will deliver a link (e.g.`/verify?type=magiclink&token=aabbccddee1234`) to the user based on email address which they can use to redeem an`access_token`.                                                                        |
| **POST** | `/recover`               | Password recovery. Will deliver a password recovery mail to the user based on email address.                                                                                                                                                 |
| **POST** | `/token`                 | This is an OAuth2 endpoint that currently implements the password and refresh_token grant types.                                                                                                                                             |
| **GET**  | `/user`                  | Get the JSON object for the logged in user (requires authentication)                                                                                                                                                                         |
| **PUT**  | `/user`                  | Update a user (Requires authentication). Apart from changing email/password, this method can be used to set custom user data. Changing the email will result in a magiclink being sent out.                                                  |
| **GET**  | `/reauthenticate`        | Sends a nonce to the user's email (preferred) or phone. This endpoint requires the user to be logged in / authenticated first. The user needs to have either an email or phone number for the nonce to be sent successfully.                 |
| **POST** | `/logout`                | Logout a user (Requires authentication). This will revoke all refresh tokens for the user. Remember that the JWT tokens will still be valid for stateless auth until they expires.                                                           |
| **GET**  | `/authorize`             | Get`access_token` from external oauth provider.                                                                                                                                                                                              |
| **GET**  | `/callback`              | External provider should redirect to here.                                                                                                                                                                                                   |

For more detailed API documentation, go to: [`http://localhost:9090/swagger`](http://localhost:9090/swagger)
