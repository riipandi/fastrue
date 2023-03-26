# Fastrue JS Library

[![Creator Badge](https://badgen.net/badge/icon/Made%20by%20Aris%20Ripandi?icon=bitcoin-lightning&label&color=blue&labelColor=black&style=flat-square)](https://ripandis.com)
[![Twitter Badge](https://badgen.net/badge/icon/Follow%20Twitter?icon=twitter&label&color=blue&labelColor=black&style=flat-square)](https://twitter.com/riipandi)
[![GitHub contributors](https://img.shields.io/github/contributors/riipandi/fastrue-js?style=flat-square)](https://github.com/riipandi/fastrue-js/graphs/contributors)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](./CODE_OF_CONDUCT.md)
[![License Bagde](https://badgen.net/github/license/riipandi/fastrue-js?label=license&color=blue&labelColor=black&style=flat-square)](./LICENSE)
[![Github Sponsor](https://badgen.net/badge/icon/sponsors?icon=github&label&color=green&labelColor=black&style=flat-square)](https://github.com/sponsors/riipandi)

<hr/>

> **WARNING!** This project still in development.
> Everything is experimental, breaking changes can happen and the long-term purpose of this project is not yet clear, use at your own risk!

This is a JavaScript client library for the [Fastrue](https://github.com/riipandi/fastrue) API.

It lets you create and authenticate users and is a building block for constructing the UI for signups, password recovery, login and logout.

## Installation

```sh
# Install with npm
npm install fastrue-js

# Install with yarn
yarn add fastrue-js

# Install with pnpm
pnpm install fastrue-js
```

## Usage

```js
import Fastrue from "fastrue-js";

/**
 * Instantiate the Fastrue auth client with an optional configuration.
 */

auth = new Fastrue({
    apiUrl: "http://localhost:9999",
    audience: "my-web-app",
    setCookie: false,
});
```

## License

This project is open-sourced software licensed under the [MIT license][choosealicense]

Copyrights in this project are retained by their contributors.

See the [license file](./LICENSE) for more information.

[choosealicense]: https://choosealicense.com/licenses/mit/

---

<sub>🤫 Psst! If you like my work you can support me via [GitHub sponsors](https://github.com/sponsors/riipandi).
